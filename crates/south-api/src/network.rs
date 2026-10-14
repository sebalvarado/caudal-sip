use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::Arc;

use caudal_clearing::Malla;
use caudal_ledger::{Cause, EntryQuery, Ledger, LedgerError, Leg, Refusal, Transfer, TransferId};
use caudal_msgplane::{Envelope, Keyring, Schedule, Signer, SimClock, digest};
use caudal_payments::{Command, Ctx, Event, Input, Message, Order, Outcome, Output, Role, Timer};
use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};
use pollster::block_on;
use uuid::Uuid;
use web_time::Instant;

use crate::stats::{Op, Stats};
use crate::wire;

pub type Fault = Arc<dyn Fn(Bic, Bic, &Message) -> bool + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hop {
    pub at: Timestamp,
    pub from: Bic,
    pub to: Bic,
    pub payment: PaymentId,
    pub definition: &'static str,
    pub state: Option<PaymentState>,
    pub reason: Option<ReasonCode>,
    pub xml: Vec<u8>,
    pub dropped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: PaymentId,
    pub from: Bic,
    pub to: Bic,
    pub amount: Usd,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub state: PaymentState,
    pub reason: Option<ReasonCode>,
    pub por_conciliar: bool,
    pub in_process: bool,
}

fn rank(state: PaymentState) -> u8 {
    match state {
        PaymentState::Rcvd | PaymentState::Pdng => 0,
        PaymentState::Actc => 1,
        PaymentState::Acsp => 2,
        PaymentState::Acsc | PaymentState::Rjct => 3,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    pub at: Timestamp,
    pub malla: Malla,
}

enum Pending {
    Initiate(Order),
    Deliver(Envelope),
    Deadline(Bic, Timer),
}

struct Node {
    role: Role,
    signer: Signer,
}

pub struct Network<L> {
    clock: SimClock,
    schedule: Schedule<Pending>,
    nodes: BTreeMap<Bic, Node>,
    keyring: Keyring,
    sip: Bic,
    ledger: L,
    latency: SignedDuration,
    faults: Vec<Fault>,
    ids: u128,
    record_hops: bool,
    hops: Vec<Hop>,
    rows: Vec<Row>,
    index: HashMap<PaymentId, usize>,
    reserved_at: HashMap<PaymentId, Timestamp>,
    settlements: Vec<Settlement>,
    rejected: usize,
    stats: Stats,
}

impl<L: Ledger> Network<L> {
    pub fn new(ledger: L, clock: SimClock, roles: Vec<Role>, sip: Bic, seed: &[u8], latency: SignedDuration) -> Self {
        let mut keyring = Keyring::default();
        let mut nodes = BTreeMap::new();
        for role in roles {
            let signer = Signer::derive(role.bic(), seed);
            keyring.trust(role.bic(), signer.verifying_key());
            nodes.insert(role.bic(), Node { role, signer });
        }
        Self {
            clock,
            schedule: Schedule::default(),
            nodes,
            keyring,
            sip,
            ledger,
            latency,
            faults: Vec::new(),
            ids: 0,
            record_hops: true,
            hops: Vec::new(),
            rows: Vec::new(),
            index: HashMap::new(),
            reserved_at: HashMap::new(),
            settlements: Vec::new(),
            rejected: 0,
            stats: Stats::default(),
        }
    }

    pub fn record_hops(&mut self, on: bool) {
        self.record_hops = on;
    }

    pub fn now(&self) -> Timestamp {
        self.clock.now()
    }

    pub fn ledger(&self) -> &L {
        &self.ledger
    }

    pub fn role(&self, bic: Bic) -> Option<&Role> {
        self.nodes.get(&bic).map(|n| &n.role)
    }

    pub fn role_mut(&mut self, bic: Bic) -> Option<&mut Role> {
        self.nodes.get_mut(&bic).map(|n| &mut n.role)
    }

    pub fn add_fault(&mut self, fault: Fault) {
        self.faults.push(fault);
    }

    pub fn clear_faults(&mut self) {
        self.faults.clear();
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn row(&self, id: &PaymentId) -> Option<&Row> {
        self.index.get(id).map(|&i| &self.rows[i])
    }

    pub fn hops(&self, id: &PaymentId) -> Vec<&Hop> {
        self.hops.iter().filter(|h| &h.payment == id).collect()
    }

    pub fn settlements(&self) -> &[Settlement] {
        &self.settlements
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn rejected_envelopes(&self) -> usize {
        self.rejected
    }

    pub fn pending(&self) -> usize {
        self.schedule.len()
    }

    fn next_id(&mut self) -> Uuid {
        self.ids += 1;
        Uuid::from_u128(self.ids)
    }

    pub fn fund(&mut self, legs: Vec<Leg>) -> Result<(), LedgerError> {
        if legs.is_empty() {
            return Ok(());
        }
        let event = self.next_id();
        let transfer = Transfer {
            id: TransferId(event),
            purpose: caudal_ledger::Purpose::Headroom,
            legs,
            cause: Cause { event, digest: [0; 32] },
        };
        block_on(self.ledger.transfer(transfer)).map(drop)
    }

    pub fn schedule_payment(&mut self, at: Timestamp, order: Order) {
        self.schedule.at(at, Pending::Initiate(order));
    }

    pub fn run_until(&mut self, until: Timestamp) {
        while self.schedule.peek_time().is_some_and(|t| t <= until) {
            self.dispatch_next();
        }
        self.clock.advance_to(until);
    }

    pub fn run_until_idle(&mut self) {
        while !self.schedule.is_empty() {
            self.dispatch_next();
        }
    }

    fn dispatch_next(&mut self) {
        let Some((at, pending)) = self.schedule.pop() else { return };
        self.clock.advance_to(at);
        match pending {
            Pending::Initiate(order) => self.initiate(order),
            Pending::Deliver(envelope) => self.deliver(envelope),
            Pending::Deadline(bic, timer) => {
                let ctx = self.ctx([0; 32]);
                self.run(bic, ctx, Input::Deadline(timer));
            }
        }
    }

    fn ctx(&mut self, digest: [u8; 32]) -> Ctx {
        Ctx { now: self.clock.now(), event: self.next_id(), digest }
    }

    fn initiate(&mut self, order: Order) {
        let ordering = order.ordering();
        let now = self.clock.now();
        if !self.index.contains_key(&order.id) {
            self.index.insert(order.id.clone(), self.rows.len());
            self.rows.push(Row {
                id: order.id.clone(),
                from: ordering,
                to: order.beneficiary,
                amount: order.amount,
                created: order.created,
                updated: now,
                state: PaymentState::Rcvd,
                reason: None,
                por_conciliar: false,
                in_process: false,
            });
        }
        let Some(Node { role: Role::Integrante(integrante), .. }) = self.nodes.get_mut(&ordering) else { return };
        let started = Instant::now();
        let outputs = integrante.pay(order);
        self.stats.record(Op::Rules, started.elapsed());
        self.handle(ordering, outputs, &mut VecDeque::new());
    }

    fn run(&mut self, bic: Bic, mut ctx: Ctx, input: Input) {
        let mut work = VecDeque::from([input]);
        while let Some(input) = work.pop_front() {
            let Some(node) = self.nodes.get_mut(&bic) else { return };
            let started = Instant::now();
            let outputs = node.role.step(&ctx, input);
            self.stats.record(Op::Rules, started.elapsed());
            self.handle(bic, outputs, &mut work);
            ctx.event = self.next_id();
        }
    }

    fn handle(&mut self, bic: Bic, outputs: Vec<Output>, work: &mut VecDeque<Input>) {
        for output in outputs {
            match output {
                Output::Send { to, message } => self.send(bic, to, message),
                Output::Arm { at, timer } => self.schedule.at(at, Pending::Deadline(bic, timer)),
                Output::Event(event) => self.observe(event),
                Output::Ledger(command) => {
                    let result = self.execute(command.clone());
                    work.push_back(Input::Ledger(Outcome { command, result }));
                }
            }
        }
    }

    fn execute(&mut self, command: Command) -> Result<caudal_ledger::Effect, Refusal> {
        for _ in 0..5 {
            let started = Instant::now();
            let result = match command.clone() {
                Command::Reserve(r) => block_on(self.ledger.reserve(r)).map(|a| a.effect),
                Command::Post(p) => block_on(self.ledger.post(p)).map(|a| a.effect),
                Command::Void(p, reason) => block_on(self.ledger.void(p, reason)).map(|a| a.effect),
                Command::Transfer(t) => block_on(self.ledger.transfer(t)).map(|a| a.effect),
            };
            self.stats.record(Op::Ledger, started.elapsed());
            match result {
                Ok(effect) => return Ok(effect),
                Err(LedgerError::Refused(r)) => return Err(r),
                Err(LedgerError::Unknown(_)) => continue,
            }
        }
        panic!("the ledger's outcome stayed unknown after five attempts")
    }

    fn send(&mut self, from: Bic, to: Bic, message: Message) {
        let now = self.clock.now();
        let msg_id = format!("M{}", self.next_id().simple());
        let started = Instant::now();
        let iso = wire::to_iso(from, to, &message, now, msg_id);
        let body = caudal_iso20022::encode(&iso);
        self.stats.record(Op::Codec, started.elapsed());
        let dropped = self.faults.iter().any(|f| f(from, to, &message));
        if self.record_hops {
            let (state, reason) = match &message {
                Message::Report(r) => (Some(r.state), r.reason),
                Message::Order(_) => (None, None),
            };
            self.hops.push(Hop {
                at: now,
                from,
                to,
                payment: message.payment().clone(),
                definition: iso.document.definition(),
                state,
                reason,
                xml: body.clone(),
                dropped,
            });
        }
        if dropped {
            return;
        }
        let Some(node) = self.nodes.get(&from) else { return };
        let started = Instant::now();
        let envelope = node.signer.seal(to, body);
        self.stats.record(Op::Signature, started.elapsed());
        self.schedule.at(now + self.latency, Pending::Deliver(envelope));
    }

    fn deliver(&mut self, envelope: Envelope) {
        let started = Instant::now();
        let opened = self.keyring.open(&envelope);
        self.stats.record(Op::Signature, started.elapsed());
        let Ok(body) = opened else {
            self.rejected += 1;
            return;
        };
        let started = Instant::now();
        let message = caudal_iso20022::decode(body).and_then(|m| wire::from_iso(&m));
        self.stats.record(Op::Codec, started.elapsed());
        let Ok(message) = message else {
            self.rejected += 1;
            return;
        };
        let ctx = self.ctx(digest(&envelope.body));
        self.run(envelope.to, ctx, Input::Received { from: envelope.from, message });
    }

    fn observe(&mut self, event: Event) {
        let now = self.clock.now();
        match &event {
            Event::Reserved(id) => {
                self.reserved_at.insert(id.clone(), now);
            }
            Event::Posted(id) => {
                if let Some(at) = self.reserved_at.remove(id) {
                    self.stats.record_reserve_to_post(now.duration_since(at));
                }
            }
            Event::Voided(id, _) | Event::Refused(id, _) => {
                self.reserved_at.remove(id);
            }
            _ => {}
        }
        let Some(&i) = self.index.get(event.payment()) else { return };
        let row = &mut self.rows[i];
        row.updated = now;
        match event {
            Event::Reserved(_) => advance(row, PaymentState::Actc, None),
            Event::Posted(_) => advance(row, PaymentState::Acsp, None),
            Event::Voided(_, reason) | Event::Refused(_, reason) => advance(row, PaymentState::Rjct, Some(reason)),
            Event::PorConciliar(_) => row.por_conciliar = true,
            Event::Settled(_) => advance(row, PaymentState::Acsc, None),
            Event::InProcess(_) => row.in_process = true,
            Event::Outcome(report) => {
                row.in_process = false;
                advance(row, report.state, report.reason);
            }
            Event::Credited(_) => {}
        }
    }

    pub fn settle(&mut self) -> Result<Settlement, LedgerError> {
        let entries = block_on(self.ledger.entries(EntryQuery::All))?;
        let malla = caudal_clearing::malla(&entries);
        let ctx = self.ctx([0; 32]);
        if let Some(transfer) = caudal_clearing::settlement(&malla, TransferId(ctx.event), ctx.cause()) {
            let _ = block_on(self.ledger.transfer(transfer))?;
        }
        self.run(self.sip, ctx, Input::Settled(malla.payments.clone()));
        let settlement = Settlement { at: ctx.now, malla };
        self.settlements.push(settlement.clone());
        Ok(settlement)
    }
}

fn advance(row: &mut Row, state: PaymentState, reason: Option<ReasonCode>) {
    let allowed = match (row.state, state) {
        (PaymentState::Rjct | PaymentState::Acsc, _) | (PaymentState::Acsp, PaymentState::Rjct) => false,
        (current, next) => rank(next) > rank(current),
    };
    if allowed {
        row.state = state;
        row.reason = reason;
    }
}
