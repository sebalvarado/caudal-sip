use std::future::{Ready, ready};
use std::sync::Mutex;

use caudal_types::{PaymentId, ReasonCode};
use jiff::Timestamp;

use crate::core::{self, Decision, Replay};
use crate::{
    AccountId, AsOf, Balance, Book, Entry, EntryId, EntryQuery, Ledger, LedgerError, Position, Refusal, Reservation,
    Reserve, Transfer,
};

pub struct MemLedger {
    clock: Box<dyn Fn() -> Timestamp + Send + Sync>,
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    log: Vec<Entry>,
    latest: Replay,
}

impl MemLedger {
    pub fn new(clock: impl Fn() -> Timestamp + Send + Sync + 'static) -> Self {
        Self { clock: Box::new(clock), state: Mutex::default() }
    }

    fn commit<T>(&self, decide: impl FnOnce(&Replay) -> Result<Decision<T>, Refusal>) -> Result<T, LedgerError> {
        let mut state = self.state.lock().expect("a panic while committing leaves nothing half-applied");
        let decision = decide(&state.latest)?;
        if let Some(draft) = decision.draft {
            let at = core::next_commit_time(state.log.last().map(|e| e.at), (self.clock)());
            let entry = Entry { at, kind: draft.kind, legs: draft.legs };
            state.latest.apply(&entry);
            state.log.push(entry);
        }
        Ok(decision.answer)
    }

    fn read<T>(&self, at: AsOf, f: impl FnOnce(&Replay) -> T) -> Ready<Result<T, LedgerError>> {
        let state = self.state.lock().expect("a panic while committing leaves nothing half-applied");
        ready(Ok(match at {
            AsOf::Latest => f(&state.latest),
            AsOf::At(t) => f(&Replay::of(state.log.iter().take_while(|e| e.at <= t))),
        }))
    }
}

impl Ledger for MemLedger {
    fn reserve(&self, r: Reserve) -> impl Future<Output = Result<Reservation, LedgerError>> + Send {
        ready(self.commit(|v| core::reserve(v, r)))
    }

    fn post(&self, payment: PaymentId) -> impl Future<Output = Result<Reservation, LedgerError>> + Send {
        ready(self.commit(|v| core::post(v, &payment)))
    }

    fn void(
        &self,
        payment: PaymentId,
        reason: ReasonCode,
    ) -> impl Future<Output = Result<Reservation, LedgerError>> + Send {
        ready(self.commit(|v| core::void(v, &payment, reason)))
    }

    fn transfer(&self, t: Transfer) -> impl Future<Output = Result<EntryId, LedgerError>> + Send {
        ready(self.commit(|v| core::transfer(v, t)))
    }

    fn balance(&self, account: AccountId, at: AsOf) -> impl Future<Output = Result<Balance, LedgerError>> + Send {
        self.read(at, |r| core::View::balance(r, account))
    }

    fn positions(&self, book: Book, at: AsOf) -> impl Future<Output = Result<Vec<Position>, LedgerError>> + Send {
        self.read(at, |r| r.positions().into_iter().filter(|p| p.account.book() == book).collect())
    }

    fn entries(&self, q: EntryQuery) -> impl Future<Output = Result<Vec<Entry>, LedgerError>> + Send {
        let state = self.state.lock().expect("a panic while committing leaves nothing half-applied");
        ready(Ok(match q {
            EntryQuery::All => state.log.clone(),
            EntryQuery::Payment(p) => state.log.iter().filter(|e| e.payment() == Some(&p)).cloned().collect(),
        }))
    }
}
