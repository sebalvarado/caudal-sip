use caudal_types::{PaymentId, ReasonCode};

use crate::{
    AccountId, AsOf, Balance, Book, Entry, EntryId, EntryQuery, LedgerError, Position, Reservation, Reserve, Transfer,
};

pub trait Ledger {
    fn reserve(&self, r: Reserve) -> impl Future<Output = Result<Reservation, LedgerError>> + Send;
    fn post(&self, payment: PaymentId) -> impl Future<Output = Result<Reservation, LedgerError>> + Send;
    fn void(
        &self,
        payment: PaymentId,
        reason: ReasonCode,
    ) -> impl Future<Output = Result<Reservation, LedgerError>> + Send;

    fn transfer(&self, t: Transfer) -> impl Future<Output = Result<EntryId, LedgerError>> + Send;

    fn balance(&self, account: AccountId, at: AsOf) -> impl Future<Output = Result<Balance, LedgerError>> + Send;
    fn positions(&self, book: Book, at: AsOf) -> impl Future<Output = Result<Vec<Position>, LedgerError>> + Send;
    fn entries(&self, q: EntryQuery) -> impl Future<Output = Result<Vec<Entry>, LedgerError>> + Send;
}
