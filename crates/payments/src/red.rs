use std::collections::{BTreeSet, HashMap};

use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode};

use crate::{Ctx, Deadlines, Input, Message, Order, Output, Report, TimerKind, arm, report, send};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    ToSip { acknowledged: bool },
    Local,
    FromSip,
}

#[derive(Debug, Clone)]
struct Payment {
    order: Order,
    route: Route,
    answered: bool,
}

#[derive(Debug, Clone)]
pub struct Red {
    bic: Bic,
    sip: Bic,
    members: BTreeSet<Bic>,
    deadlines: Deadlines,
    payments: HashMap<PaymentId, Payment>,
}

impl Red {
    pub fn new(bic: Bic, sip: Bic, members: impl IntoIterator<Item = Bic>, deadlines: Deadlines) -> Self {
        Self { bic, sip, members: members.into_iter().collect(), deadlines, payments: HashMap::new() }
    }

    pub fn bic(&self) -> Bic {
        self.bic
    }

    pub fn step(&mut self, ctx: &Ctx, input: Input) -> Vec<Output> {
        match input {
            Input::Received { from, message: Message::Order(order) } if from == self.sip => self.inbound(ctx, order),
            Input::Received { from, message: Message::Order(order) } => self.outbound(ctx, from, order),
            Input::Received { from, message: Message::Report(r) } => self.report(from, r),
            Input::Deadline(timer) => self.deadline(&timer.payment, timer.kind),
            Input::Ledger(_) | Input::Settled(_) => vec![],
        }
    }

    fn outbound(&mut self, ctx: &Ctx, from: Bic, order: Order) -> Vec<Output> {
        let id = order.id.clone();
        if self.payments.contains_key(&id) {
            return vec![report(from, &id, PaymentState::Rcvd, None)];
        }
        if !self.members.contains(&from) || order.ordering() != from || order.ordering_red != self.bic {
            return vec![report(from, &id, PaymentState::Rjct, Some(ReasonCode::Ms01))];
        }
        let mut out = vec![report(from, &id, PaymentState::Rcvd, None)];
        if order.beneficiary_red == self.bic {
            if !self.members.contains(&order.beneficiary) {
                out.push(report(from, &id, PaymentState::Rjct, Some(ReasonCode::Ms01)));
                return out;
            }
            out.push(send(order.beneficiary, Message::Order(order.clone())));
            out.push(arm(ctx.now + self.deadlines.credit, &id, TimerKind::Credit));
            self.payments.insert(id, Payment { order, route: Route::Local, answered: false });
        } else {
            out.push(send(self.sip, Message::Order(order.clone())));
            out.push(arm(ctx.now + self.deadlines.to_sip, &id, TimerKind::ToSip));
            self.payments.insert(id, Payment { order, route: Route::ToSip { acknowledged: false }, answered: false });
        }
        out
    }

    fn inbound(&mut self, ctx: &Ctx, order: Order) -> Vec<Output> {
        let id = order.id.clone();
        let mut out = vec![report(self.sip, &id, PaymentState::Rcvd, None)];
        if self.payments.contains_key(&id) {
            return out;
        }
        if !self.members.contains(&order.beneficiary) {
            out.push(report(self.sip, &id, PaymentState::Rjct, Some(ReasonCode::Ms01)));
            return out;
        }
        out.push(send(order.beneficiary, Message::Order(order.clone())));
        out.push(arm(ctx.now + self.deadlines.credit, &id, TimerKind::Credit));
        self.payments.insert(id, Payment { order, route: Route::FromSip, answered: false });
        out
    }

    fn report(&mut self, from: Bic, r: Report) -> Vec<Output> {
        let Some(p) = self.payments.get_mut(&r.id) else { return vec![] };
        let ordering = p.order.ordering();
        match p.route {
            Route::ToSip { .. } if from == self.sip => {
                if r.state == PaymentState::Rcvd {
                    p.route = Route::ToSip { acknowledged: true };
                    return vec![];
                }
                p.answered = true;
                vec![send(ordering, Message::Report(r))]
            }
            Route::Local | Route::FromSip if from == p.order.beneficiary && r.state != PaymentState::Rcvd => {
                if p.answered {
                    return vec![];
                }
                p.answered = true;
                let to = if p.route == Route::Local { ordering } else { self.sip };
                vec![send(to, Message::Report(r))]
            }
            _ => vec![],
        }
    }

    fn deadline(&mut self, id: &PaymentId, kind: TimerKind) -> Vec<Output> {
        let Some(p) = self.payments.get_mut(id) else { return vec![] };
        match (p.route, kind, p.answered) {
            (Route::ToSip { acknowledged: false }, TimerKind::ToSip, false) => {
                p.answered = true;
                vec![report(p.order.ordering(), id, PaymentState::Rjct, Some(ReasonCode::Ta01))]
            }
            (Route::Local, TimerKind::Credit, false) => {
                p.answered = true;
                vec![report(p.order.ordering(), id, PaymentState::Rjct, Some(ReasonCode::Dt01))]
            }
            (Route::FromSip, TimerKind::Credit, false) => {
                p.answered = true;
                vec![report(self.sip, id, PaymentState::Rjct, Some(ReasonCode::Dt01))]
            }
            _ => vec![],
        }
    }
}
