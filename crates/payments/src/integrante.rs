use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode};

use crate::{Ctx, Deadlines, Event, Input, Message, Order, Output, Report, TimerKind, arm, report, send};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Accept,
    Reject(ReasonCode),
    Silent,
}

/// How a mock institution's core banking answers a credit.
#[derive(Clone)]
pub struct Script(Arc<dyn Fn(&Order) -> Answer + Send + Sync>);

impl Script {
    pub fn new(f: impl Fn(&Order) -> Answer + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }

    pub fn accept() -> Self {
        Self::new(|_| Answer::Accept)
    }
}

impl fmt::Debug for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Script")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sent {
    acknowledged: bool,
    finished: bool,
}

#[derive(Debug, Clone)]
pub struct Integrante {
    bic: Bic,
    red: Bic,
    deadlines: Deadlines,
    script: Script,
    sent: HashMap<PaymentId, Sent>,
    answered: HashMap<PaymentId, Option<Report>>,
}

impl Integrante {
    pub fn new(bic: Bic, red: Bic, deadlines: Deadlines, script: Script) -> Self {
        Self { bic, red, deadlines, script, sent: HashMap::new(), answered: HashMap::new() }
    }

    pub fn bic(&self) -> Bic {
        self.bic
    }

    pub fn red(&self) -> Bic {
        self.red
    }

    pub fn set_script(&mut self, script: Script) {
        self.script = script;
    }

    /// The customer confirmed: debit and reserve on the core, then send.
    pub fn pay(&mut self, order: Order) -> Vec<Output> {
        let at = order.created + self.deadlines.end_to_end();
        let id = order.id.clone();
        self.sent.insert(id.clone(), Sent { acknowledged: false, finished: false });
        vec![send(self.red, Message::Order(order)), arm(at, &id, TimerKind::EndToEnd)]
    }

    pub fn step(&mut self, _ctx: &Ctx, input: Input) -> Vec<Output> {
        match input {
            Input::Received { from, message } if from == self.red => match message {
                Message::Order(order) => self.credit(order),
                Message::Report(r) => self.outcome(r),
            },
            Input::Deadline(timer) if timer.kind == TimerKind::EndToEnd => self.expire(&timer.payment),
            _ => vec![],
        }
    }

    fn credit(&mut self, order: Order) -> Vec<Output> {
        let id = order.id.clone();
        let mut out = vec![report(self.red, &id, PaymentState::Rcvd, None)];
        if let Some(previous) = self.answered.get(&id) {
            out.extend(previous.clone().map(|r| send(self.red, Message::Report(r))));
            return out;
        }
        let answer = match (self.script.0)(&order) {
            Answer::Accept => {
                out.push(Output::Event(Event::Credited(id.clone())));
                Some(Report::new(&id, PaymentState::Acsp, None))
            }
            Answer::Reject(reason) => Some(Report::rejected(&id, reason)),
            Answer::Silent => None,
        };
        out.extend(answer.clone().map(|r| send(self.red, Message::Report(r))));
        self.answered.insert(id, answer);
        out
    }

    fn outcome(&mut self, r: Report) -> Vec<Output> {
        let Some(sent) = self.sent.get_mut(&r.id) else { return vec![] };
        if r.state == PaymentState::Rcvd {
            sent.acknowledged = true;
            return vec![];
        }
        sent.finished = true;
        vec![Output::Event(Event::Outcome(r))]
    }

    fn expire(&mut self, id: &PaymentId) -> Vec<Output> {
        let Some(sent) = self.sent.get_mut(id) else { return vec![] };
        if sent.finished {
            return vec![];
        }
        if sent.acknowledged {
            return vec![Output::Event(Event::InProcess(id.clone()))];
        }
        sent.finished = true;
        vec![Output::Event(Event::Outcome(Report::rejected(id, ReasonCode::Dt01)))]
    }
}
