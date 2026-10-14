mod integrante;
mod red;
mod sip;

use caudal_ledger::{Cause, Refusal, Reserve, Transfer};
use caudal_types::{Bic, InvalidValue, PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};
use uuid::Uuid;

pub use integrante::{Answer, Integrante, Script};
pub use red::Red;
pub use sip::Sip;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Party {
    pub name: String,
    pub document: String,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: PaymentId,
    pub created: Timestamp,
    pub ordering_red: Bic,
    pub beneficiary_red: Bic,
    pub beneficiary: Bic,
    pub amount: Usd,
    pub debtor: Party,
    pub creditor: Party,
}

impl Order {
    pub fn ordering(&self) -> Bic {
        self.id.ordering()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub id: PaymentId,
    pub state: PaymentState,
    pub reason: Option<ReasonCode>,
}

impl Report {
    pub fn new(id: &PaymentId, state: PaymentState, reason: Option<ReasonCode>) -> Self {
        Self { id: id.clone(), state, reason }
    }

    pub fn rejected(id: &PaymentId, reason: ReasonCode) -> Self {
        Self::new(id, PaymentState::Rjct, Some(reason))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Order(Order),
    Report(Report),
}

impl Message {
    pub fn payment(&self) -> &PaymentId {
        match self {
            Self::Order(o) => &o.id,
            Self::Report(r) => &r.id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerKind {
    EndToEnd,
    ToSip,
    Deliver,
    Confirm,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timer {
    pub payment: PaymentId,
    pub kind: TimerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Reserve(Reserve),
    Post(PaymentId),
    Void(PaymentId, ReasonCode),
    Transfer(Transfer),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub command: Command,
    pub result: Result<(), Refusal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Received { from: Bic, message: Message },
    Ledger(Outcome),
    Deadline(Timer),
    Settled(Vec<PaymentId>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Reserved(PaymentId),
    Posted(PaymentId),
    Voided(PaymentId, ReasonCode),
    Refused(PaymentId, ReasonCode),
    PorConciliar(PaymentId),
    Settled(PaymentId),
    Credited(PaymentId),
    Outcome(Report),
    InProcess(PaymentId),
}

impl Event {
    pub fn payment(&self) -> &PaymentId {
        match self {
            Self::Reserved(id)
            | Self::Posted(id)
            | Self::Voided(id, _)
            | Self::Refused(id, _)
            | Self::PorConciliar(id)
            | Self::Settled(id)
            | Self::Credited(id)
            | Self::InProcess(id) => id,
            Self::Outcome(r) => &r.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Send { to: Bic, message: Message },
    Ledger(Command),
    Arm { at: Timestamp, timer: Timer },
    Event(Event),
}

/// What a machine may read about the step it is taking. Ids and time come from the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ctx {
    pub now: Timestamp,
    pub event: Uuid,
    pub digest: [u8; 32],
}

impl Ctx {
    pub fn cause(&self) -> Cause {
        Cause { event: self.event, digest: self.digest }
    }
}

/// Each stage's timeout is shorter than the stage above it, inside the end-to-end limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deadlines {
    end_to_end: SignedDuration,
    to_sip: SignedDuration,
    deliver: SignedDuration,
    confirm: SignedDuration,
    credit: SignedDuration,
}

impl Deadlines {
    pub fn new(
        end_to_end: SignedDuration,
        to_sip: SignedDuration,
        deliver: SignedDuration,
        confirm: SignedDuration,
        credit: SignedDuration,
    ) -> Result<Self, InvalidValue> {
        let nested = credit < confirm && deliver + confirm < end_to_end && to_sip + deliver + confirm < end_to_end;
        if !nested || [to_sip, deliver, confirm, credit].iter().any(|d| !d.is_positive()) {
            return Err(InvalidValue::new("Deadlines", "positive stage timeouts nested inside the end-to-end limit"));
        }
        Ok(Self { end_to_end, to_sip, deliver, confirm, credit })
    }

    pub fn end_to_end(&self) -> SignedDuration {
        self.end_to_end
    }
}

impl Default for Deadlines {
    fn default() -> Self {
        let s = SignedDuration::from_secs;
        Self::new(s(20), s(3), s(3), s(10), s(8)).expect("the defaults nest")
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    Sip(Sip),
    Red(Red),
    Integrante(Integrante),
}

impl Role {
    pub fn bic(&self) -> Bic {
        match self {
            Self::Sip(m) => m.bic(),
            Self::Red(m) => m.bic(),
            Self::Integrante(m) => m.bic(),
        }
    }

    pub fn step(&mut self, ctx: &Ctx, input: Input) -> Vec<Output> {
        match self {
            Self::Sip(m) => m.step(ctx, input),
            Self::Red(m) => m.step(ctx, input),
            Self::Integrante(m) => m.step(ctx, input),
        }
    }
}

fn send(to: Bic, message: Message) -> Output {
    Output::Send { to, message }
}

fn report(to: Bic, id: &PaymentId, state: PaymentState, reason: Option<ReasonCode>) -> Output {
    send(to, Message::Report(Report::new(id, state, reason)))
}

fn arm(at: Timestamp, payment: &PaymentId, kind: TimerKind) -> Output {
    Output::Arm { at, timer: Timer { payment: payment.clone(), kind } }
}
