use caudal_types::{PaymentId, ReasonCode};

use crate::{
    AccountId, Applied, AsOf, Balance, Book, Cause, Entry, EntryId, EntryQuery, LedgerError, Position, Reservation,
    Reserve, Transfer, TransferId,
};

pub trait Ledger {
    fn reserve(&self, r: Reserve) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;
    fn post(&self, payment: PaymentId) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;
    fn void(
        &self,
        payment: PaymentId,
        reason: ReasonCode,
    ) -> impl Future<Output = Result<Applied<Reservation>, LedgerError>> + Send;

    fn transfer(&self, t: Transfer) -> impl Future<Output = Result<Applied<EntryId>, LedgerError>> + Send;

    /// Appends the mirror of every leg of a transfer. A payment reservation is not a transfer, so this cannot
    /// undo a posted payment. A repeat returns the reversal already on the books.
    fn reverse(
        &self,
        of: TransferId,
        cause: Cause,
    ) -> impl Future<Output = Result<Applied<EntryId>, LedgerError>> + Send;

    fn balance(&self, account: AccountId, at: AsOf) -> impl Future<Output = Result<Balance, LedgerError>> + Send;
    fn positions(&self, book: Book, at: AsOf) -> impl Future<Output = Result<Vec<Position>, LedgerError>> + Send;
    fn entries(&self, q: EntryQuery) -> impl Future<Output = Result<Vec<Entry>, LedgerError>> + Send;
}
