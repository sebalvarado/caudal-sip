use std::collections::HashMap;

use caudal_ledger::{Leg, ObligationAccount, Refusal, Reserve, Transfer, TransferId};
use caudal_registry::{Registry, Status};
use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode, Usd};

use crate::{
    Command, Ctx, Deadlines, Event, Input, Message, Order, Outcome, Output, Report, TimerKind, arm, report, send,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Reserving,
    Delivering,
    Confirming,
    Posting { por_conciliar: bool },
    Voiding(ReasonCode),
    Done(PaymentState, Option<ReasonCode>),
}

#[derive(Debug, Clone)]
struct Payment {
    order: Order,
    stage: Stage,
}

#[derive(Debug, Clone)]
pub struct Sip {
    bic: Bic,
    registry: Registry,
    deadlines: Deadlines,
    payments: HashMap<PaymentId, Payment>,
}

impl Sip {
    pub fn new(bic: Bic, registry: Registry, deadlines: Deadlines) -> Self {
        Self { bic, registry, deadlines, payments: HashMap::new() }
    }

    pub fn bic(&self) -> Bic {
        self.bic
    }

    pub fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }

    pub fn step(&mut self, ctx: &Ctx, input: Input) -> Vec<Output> {
        match input {
            Input::Received { from, message: Message::Order(order) } => self.order(ctx, from, order),
            Input::Received { from, message: Message::Report(r) } => self.report(ctx, from, r),
            Input::Ledger(outcome) => self.outcome(ctx, outcome),
            Input::Deadline(timer) => self.deadline(&timer.payment, timer.kind),
            Input::Settled(payments) => self.settled(&payments),
        }
    }

    fn order(&mut self, ctx: &Ctx, from: Bic, order: Order) -> Vec<Output> {
        let id = order.id.clone();
        if let Some(known) = self.payments.get(&id) {
            if known.order != order {
                return vec![report(from, &id, PaymentState::Rjct, Some(ReasonCode::Du01))];
            }
            let mut out = vec![report(from, &id, PaymentState::Rcvd, None)];
            if let Stage::Done(state, reason) = known.stage {
                out.push(report(from, &id, state, reason));
            }
            return out;
        }
        let mut out = vec![report(from, &id, PaymentState::Rcvd, None)];
        if let Err(reason) = self.validate(ctx, from, &order) {
            self.payments.insert(id.clone(), Payment { order, stage: Stage::Done(PaymentState::Rjct, Some(reason)) });
            out.push(Output::Event(Event::Refused(id.clone(), reason)));
            out.push(report(from, &id, PaymentState::Rjct, Some(reason)));
            return out;
        }
        out.push(Output::Ledger(Command::Reserve(Reserve {
            payment: id.clone(),
            beneficiary: order.beneficiary,
            amount: order.amount,
            fee: Usd::ZERO,
            tax: Usd::ZERO,
            cause: ctx.cause(),
        })));
        self.payments.insert(id, Payment { order, stage: Stage::Reserving });
        out
    }

    fn validate(&self, ctx: &Ctx, from: Bic, order: &Order) -> Result<(), ReasonCode> {
        let ordering = self.registry.integrante(order.ordering()).ok_or(ReasonCode::Ms01)?;
        let beneficiary = self.registry.integrante(order.beneficiary).ok_or(ReasonCode::Ms01)?;
        if ordering.red != from || order.ordering_red != from || beneficiary.red != order.beneficiary_red {
            return Err(ReasonCode::Ms01);
        }
        if order.ordering_red == order.beneficiary_red {
            return Err(ReasonCode::Ms01);
        }
        if ordering.status == Status::Suspended {
            return Err(ReasonCode::Am04);
        }
        if beneficiary.status == Status::Suspended {
            return Err(ReasonCode::Ac06);
        }
        if order.amount == Usd::ZERO {
            return Err(ReasonCode::Am01);
        }
        if ctx.now > order.created + self.deadlines.end_to_end {
            return Err(ReasonCode::Dt01);
        }
        Ok(())
    }

    fn report(&mut self, ctx: &Ctx, from: Bic, r: Report) -> Vec<Output> {
        let Some(p) = self.payments.get_mut(&r.id) else { return vec![] };
        if from != p.order.beneficiary_red {
            return vec![];
        }
        match (p.stage, r.state, r.reason) {
            (Stage::Delivering, PaymentState::Rcvd, _) => {
                p.stage = Stage::Confirming;
                vec![arm(ctx.now + self.deadlines.confirm, &r.id, TimerKind::Confirm)]
            }
            (Stage::Delivering | Stage::Confirming, PaymentState::Acsp, _) => {
                p.stage = Stage::Posting { por_conciliar: false };
                vec![Output::Ledger(Command::Post(r.id))]
            }
            (Stage::Delivering | Stage::Confirming, PaymentState::Rjct, reason) => {
                let reason = reason.unwrap_or(ReasonCode::Ff10);
                p.stage = Stage::Voiding(reason);
                vec![Output::Ledger(Command::Void(r.id, reason))]
            }
            _ => vec![],
        }
    }

    fn deadline(&mut self, id: &PaymentId, kind: TimerKind) -> Vec<Output> {
        let Some(p) = self.payments.get_mut(id) else { return vec![] };
        match (p.stage, kind) {
            (Stage::Delivering, TimerKind::Deliver) => {
                p.stage = Stage::Voiding(ReasonCode::Ta01);
                vec![Output::Ledger(Command::Void(id.clone(), ReasonCode::Ta01))]
            }
            (Stage::Confirming, TimerKind::Confirm) => {
                p.stage = Stage::Posting { por_conciliar: true };
                vec![Output::Ledger(Command::Post(id.clone()))]
            }
            _ => vec![],
        }
    }

    fn outcome(&mut self, ctx: &Ctx, outcome: Outcome) -> Vec<Output> {
        let id = match &outcome.command {
            Command::Reserve(r) => r.payment.clone(),
            Command::Post(id) | Command::Void(id, _) => id.clone(),
            Command::Transfer(_) => return vec![],
        };
        let Some(p) = self.payments.get_mut(&id) else { return vec![] };
        let ordering_red = p.order.ordering_red;
        match (p.stage, outcome.result) {
            (Stage::Reserving, Ok(())) => {
                p.stage = Stage::Delivering;
                vec![
                    Output::Event(Event::Reserved(id.clone())),
                    send(p.order.beneficiary_red, Message::Order(p.order.clone())),
                    arm(ctx.now + self.deadlines.deliver, &id, TimerKind::Deliver),
                ]
            }
            (Stage::Reserving, Err(refusal)) => {
                let reason = match refusal {
                    Refusal::InsufficientHeadroom => ReasonCode::Am04,
                    Refusal::Conflict => ReasonCode::Du01,
                    _ => ReasonCode::Ff10,
                };
                p.stage = Stage::Done(PaymentState::Rjct, Some(reason));
                vec![
                    Output::Event(Event::Refused(id.clone(), reason)),
                    report(ordering_red, &id, PaymentState::Rjct, Some(reason)),
                ]
            }
            (Stage::Posting { por_conciliar }, Ok(())) => {
                p.stage = Stage::Done(PaymentState::Acsp, None);
                let mut out = vec![Output::Event(Event::Posted(id.clone()))];
                if por_conciliar {
                    let memo = Leg::Obligations {
                        debit: ObligationAccount::Suspense,
                        credit: ObligationAccount::Suspense,
                        amount: p.order.amount,
                    };
                    out.push(Output::Ledger(Command::Transfer(Transfer {
                        id: TransferId(ctx.event),
                        legs: vec![memo],
                        cause: ctx.cause(),
                    })));
                    out.push(Output::Event(Event::PorConciliar(id.clone())));
                }
                out.push(report(ordering_red, &id, PaymentState::Acsp, None));
                out
            }
            (Stage::Voiding(reason), Ok(())) => {
                p.stage = Stage::Done(PaymentState::Rjct, Some(reason));
                vec![
                    Output::Event(Event::Voided(id.clone(), reason)),
                    report(ordering_red, &id, PaymentState::Rjct, Some(reason)),
                ]
            }
            (Stage::Posting { .. } | Stage::Voiding(_), Err(_)) => {
                vec![Output::Event(Event::Refused(id, ReasonCode::Ff10))]
            }
            _ => vec![],
        }
    }

    fn settled(&mut self, payments: &[PaymentId]) -> Vec<Output> {
        let mut out = Vec::new();
        for id in payments {
            let Some(p) = self.payments.get_mut(id) else { continue };
            if p.stage == Stage::Done(PaymentState::Acsp, None) {
                p.stage = Stage::Done(PaymentState::Acsc, None);
                out.push(Output::Event(Event::Settled(id.clone())));
                out.push(report(p.order.ordering_red, id, PaymentState::Acsc, None));
            }
        }
        out
    }

    pub fn state(&self, id: &PaymentId) -> Option<(PaymentState, Option<ReasonCode>)> {
        match self.payments.get(id)?.stage {
            Stage::Done(state, reason) => Some((state, reason)),
            _ => Some((PaymentState::Actc, None)),
        }
    }
}
