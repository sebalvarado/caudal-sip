use std::collections::HashMap;

use caudal_ledger::core::{self, Decision, Replay, View};
use caudal_ledger::{
    AccountId, AsOf, Balance, Book, Cause, Entry, EntryId, EntryKind, EntryQuery, HeadroomAccount, Ledger, LedgerError,
    Leg, Position, Refusal, Reservation, Reserve, Transfer, TransferId,
};
use caudal_types::{Bic, PaymentId, ReasonCode, Usd};
use jiff::Timestamp;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{PgConnection, Row};
use uuid::Uuid;

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    let mut migrator = sqlx::migrate!("./migrations");
    migrator.create_schema("ledger");
    migrator.dangerous_set_table_name("ledger._sqlx_migrations");
    migrator.run(pool).await
}

pub struct PgLedger {
    pool: PgPool,
    clock: Box<dyn Fn() -> Timestamp + Send + Sync>,
}

// One writer at a time, so commit times strictly increase.
const WRITE_LOCK: i64 = 0x6c65_6467_6572;

macro_rules! sums {
    () => {
        "sum(debits_pending)::bigint AS debits_pending, sum(debits_posted)::bigint AS debits_posted, \
         sum(credits_pending)::bigint AS credits_pending, sum(credits_posted)::bigint AS credits_posted"
    };
}

impl PgLedger {
    pub fn new(pool: PgPool, clock: impl Fn() -> Timestamp + Send + Sync + 'static) -> Self {
        Self { pool, clock: Box::new(clock) }
    }

    async fn commit<T>(
        &self,
        load: Load<'_>,
        decide: impl FnOnce(&Loaded) -> Result<Decision<T>, Refusal>,
    ) -> Result<T, LedgerError> {
        let mut tx = self.pool.begin().await.map_err(unknown)?;
        sqlx::query("SELECT pg_advisory_xact_lock($1)").bind(WRITE_LOCK).execute(&mut *tx).await.map_err(unknown)?;
        let loaded = load.run(&mut tx).await?;
        let decision = decide(&loaded)?;
        if let Some(draft) = decision.draft {
            let previous: Option<i64> =
                sqlx::query_scalar("SELECT (extract(epoch FROM max(at)) * 1000000)::bigint FROM ledger.entry")
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(unknown)?;
            let previous = previous.map(from_micros).transpose()?;
            let at = core::next_commit_time(previous, (self.clock)());
            insert(&mut tx, &Entry { at, kind: draft.kind, legs: draft.legs }).await?;
            tx.commit().await.map_err(unknown)?;
        }
        Ok(decision.answer)
    }
}

struct Load<'a> {
    payment: Option<&'a PaymentId>,
    transfer: Option<TransferId>,
    accounts: Vec<AccountId>,
}

impl Load<'_> {
    async fn run(self, conn: &mut PgConnection) -> Result<Loaded, LedgerError> {
        let mut entries = Vec::new();
        if let Some(p) = self.payment {
            entries.extend(select_entries(conn, Filter::Payment(p)).await?);
        }
        if let Some(id) = self.transfer {
            entries.extend(select_entries(conn, Filter::Transfer(id)).await?);
        }
        let mut balances = HashMap::new();
        for account in self.accounts {
            balances.insert(account, select_balance(conn, account, AsOf::Latest).await?);
        }
        Ok(Loaded { replay: Replay::of(&entries), balances })
    }
}

struct Loaded {
    replay: Replay,
    balances: HashMap<AccountId, Balance>,
}

impl View for Loaded {
    fn reservation(&self, payment: &PaymentId) -> Option<Reservation> {
        self.replay.reservation(payment)
    }

    fn transfer(&self, id: TransferId) -> Option<Vec<Leg>> {
        self.replay.transfer(id)
    }

    fn balance(&self, account: AccountId) -> Balance {
        *self.balances.get(&account).expect("the decision reads only the constrained accounts it was given")
    }
}

fn headroom_of(payment: &PaymentId) -> Vec<AccountId> {
    vec![HeadroomAccount::Integrante(payment.ordering()).into()]
}

impl Ledger for PgLedger {
    async fn reserve(&self, r: Reserve) -> Result<Reservation, LedgerError> {
        let load = Load { payment: Some(&r.payment), transfer: None, accounts: headroom_of(&r.payment) };
        self.commit(load, |v| core::reserve(v, r.clone())).await
    }

    async fn post(&self, payment: PaymentId) -> Result<Reservation, LedgerError> {
        let load = Load { payment: Some(&payment), transfer: None, accounts: headroom_of(&payment) };
        self.commit(load, |v| core::post(v, &payment)).await
    }

    async fn void(&self, payment: PaymentId, reason: ReasonCode) -> Result<Reservation, LedgerError> {
        let load = Load { payment: Some(&payment), transfer: None, accounts: headroom_of(&payment) };
        self.commit(load, |v| core::void(v, &payment, reason)).await
    }

    async fn transfer(&self, t: Transfer) -> Result<EntryId, LedgerError> {
        let load = Load { payment: None, transfer: Some(t.id), accounts: core::constrained(&t.legs) };
        self.commit(load, |v| core::transfer(v, t.clone())).await
    }

    async fn balance(&self, account: AccountId, at: AsOf) -> Result<Balance, LedgerError> {
        let mut conn = self.pool.acquire().await.map_err(unknown)?;
        select_balance(&mut conn, account, at).await
    }

    async fn positions(&self, book: Book, at: AsOf) -> Result<Vec<Position>, LedgerError> {
        let rows = sqlx::query(concat!(
            "SELECT account, ",
            sums!(),
            " FROM ledger.posting WHERE $1::timestamptz IS NULL OR at <= $1::timestamptz GROUP BY account"
        ))
        .bind(as_of(at))
        .fetch_all(&self.pool)
        .await
        .map_err(unknown)?;
        let mut positions = Vec::new();
        for row in rows {
            let account: AccountId = row.get::<String, _>("account").parse().map_err(corrupt)?;
            if account.book() == book {
                positions.push(Position { account, balance: balance_from(&row)? });
            }
        }
        positions.sort_by_key(|p| p.account);
        Ok(positions)
    }

    async fn entries(&self, q: EntryQuery) -> Result<Vec<Entry>, LedgerError> {
        let mut conn = self.pool.acquire().await.map_err(unknown)?;
        match &q {
            EntryQuery::All => select_entries(&mut conn, Filter::All).await,
            EntryQuery::Payment(p) => select_entries(&mut conn, Filter::Payment(p)).await,
        }
    }
}

async fn select_balance(conn: &mut PgConnection, account: AccountId, at: AsOf) -> Result<Balance, LedgerError> {
    let row = sqlx::query(concat!(
        "SELECT ",
        sums!(),
        " FROM ledger.posting WHERE account = $1 AND ($2::timestamptz IS NULL OR at <= $2::timestamptz)"
    ))
    .bind(account.to_string())
    .bind(as_of(at))
    .fetch_one(conn)
    .await
    .map_err(unknown)?;
    balance_from(&row)
}

fn balance_from(row: &PgRow) -> Result<Balance, LedgerError> {
    let sum = |column| {
        let value: Option<i64> = row.get(column);
        u128::try_from(value.unwrap_or(0)).map_err(corrupt)
    };
    Ok(Balance {
        debits_pending: sum("debits_pending")?,
        debits_posted: sum("debits_posted")?,
        credits_pending: sum("credits_pending")?,
        credits_posted: sum("credits_posted")?,
    })
}

enum Filter<'a> {
    All,
    Payment(&'a PaymentId),
    Transfer(TransferId),
}

async fn select_entries(conn: &mut PgConnection, filter: Filter<'_>) -> Result<Vec<Entry>, LedgerError> {
    let (ordering, code, transfer) = match filter {
        Filter::All => (None, None, None),
        Filter::Payment(p) => (Some(p.ordering().to_string()), Some(p.as_str()), None),
        Filter::Transfer(id) => (None, None, Some(id.0)),
    };
    let rows = sqlx::query(
        "SELECT seq, (extract(epoch FROM at) * 1000000)::bigint AS at_us, kind, ordering, payment, beneficiary, \
         amount, fee, tax, reason, transfer_id, cause_event, cause_digest FROM ledger.entry \
         WHERE ($1::text IS NULL OR (ordering = $1 AND payment = $2)) AND ($3::uuid IS NULL OR transfer_id = $3) \
         ORDER BY seq",
    )
    .bind(ordering)
    .bind(code)
    .bind(transfer)
    .fetch_all(&mut *conn)
    .await
    .map_err(unknown)?;
    let seqs: Vec<i64> = rows.iter().map(|r| r.get("seq")).collect();
    let mut legs: HashMap<i64, Vec<Leg>> = HashMap::new();
    for row in sqlx::query("SELECT seq, debit, credit, amount FROM ledger.leg WHERE seq = ANY($1) ORDER BY seq, idx")
        .bind(&seqs)
        .fetch_all(&mut *conn)
        .await
        .map_err(unknown)?
    {
        legs.entry(row.get("seq")).or_default().push(leg_from(&row)?);
    }
    rows.iter().map(|row| entry_from(row, legs.remove(&row.get::<i64, _>("seq")).unwrap_or_default())).collect()
}

fn entry_from(row: &PgRow, legs: Vec<Leg>) -> Result<Entry, LedgerError> {
    let text = |column| row.get::<Option<String>, _>(column).ok_or_else(|| corrupt(column));
    let cents = |column| row.get::<Option<i64>, _>(column).map(cents).ok_or_else(|| corrupt(column))?;
    let payment = || PaymentId::new(text("ordering")?.parse().map_err(corrupt)?, &text("payment")?).map_err(corrupt);
    let cause = || -> Result<Cause, LedgerError> {
        let event: Option<Uuid> = row.get("cause_event");
        let digest: Option<Vec<u8>> = row.get("cause_digest");
        let digest = digest.and_then(|d| <[u8; 32]>::try_from(d).ok()).ok_or_else(|| corrupt("cause_digest"))?;
        Ok(Cause { event: event.ok_or_else(|| corrupt("cause_event"))?, digest })
    };
    let kind = match row.get::<&str, _>("kind") {
        "reserved" => EntryKind::Reserved(Reserve {
            payment: payment()?,
            beneficiary: text("beneficiary")?.parse::<Bic>().map_err(corrupt)?,
            amount: cents("amount")?,
            fee: cents("fee")?,
            tax: cents("tax")?,
            cause: cause()?,
        }),
        "posted" => EntryKind::Posted(payment()?),
        "voided" => EntryKind::Voided(payment()?, text("reason")?.parse().map_err(corrupt)?),
        "transferred" => {
            let id: Option<Uuid> = row.get("transfer_id");
            EntryKind::Transferred(TransferId(id.ok_or_else(|| corrupt("transfer_id"))?), cause()?)
        }
        other => return Err(corrupt(format!("entry kind {other}"))),
    };
    Ok(Entry { at: from_micros(row.get("at_us"))?, kind, legs })
}

fn leg_from(row: &PgRow) -> Result<Leg, LedgerError> {
    let account = |column| row.get::<String, _>(column).parse::<AccountId>().map_err(corrupt);
    let amount = cents(row.get("amount"))?;
    match (account("debit")?, account("credit")?) {
        (AccountId::Obligations(debit), AccountId::Obligations(credit)) => {
            Ok(Leg::Obligations { debit, credit, amount })
        }
        (AccountId::Headroom(debit), AccountId::Headroom(credit)) => {
            Ok(Leg::Headroom { debit, credit, amount: caudal_ledger::Headroom::of(amount) })
        }
        _ => Err(corrupt("a leg that crosses books")),
    }
}

async fn insert(conn: &mut PgConnection, entry: &Entry) -> Result<(), LedgerError> {
    let (kind, reserve, payment, reason, transfer, cause) = match &entry.kind {
        EntryKind::Reserved(r) => ("reserved", Some(r), Some(&r.payment), None, None, Some(r.cause)),
        EntryKind::Posted(p) => ("posted", None, Some(p), None, None, None),
        EntryKind::Voided(p, reason) => ("voided", None, Some(p), Some(reason.code()), None, None),
        EntryKind::Transferred(id, cause) => ("transferred", None, None, None, Some(id.0), Some(*cause)),
    };
    let term = |usd: fn(&Reserve) -> Usd| reserve.map(|r| stored(usd(r).cents())).transpose();
    let seq: i64 = sqlx::query_scalar(
        "INSERT INTO ledger.entry (at, kind, ordering, payment, beneficiary, amount, fee, tax, reason, transfer_id, \
         cause_event, cause_digest) VALUES ($1::timestamptz, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING seq",
    )
    .bind(entry.at.to_string())
    .bind(kind)
    .bind(payment.map(|p| p.ordering().to_string()))
    .bind(payment.map(PaymentId::as_str))
    .bind(reserve.map(|r| r.beneficiary.to_string()))
    .bind(term(|r| r.amount)?)
    .bind(term(|r| r.fee)?)
    .bind(term(|r| r.tax)?)
    .bind(reason)
    .bind(transfer)
    .bind(cause.map(|c| c.event))
    .bind(cause.map(|c| c.digest.to_vec()))
    .fetch_one(&mut *conn)
    .await
    .map_err(unknown)?;
    for (idx, leg) in entry.legs.iter().enumerate() {
        sqlx::query("INSERT INTO ledger.leg (seq, idx, debit, credit, amount) VALUES ($1, $2, $3, $4, $5)")
            .bind(seq)
            .bind(i16::try_from(idx).map_err(unknown)?)
            .bind(leg.debit().to_string())
            .bind(leg.credit().to_string())
            .bind(stored(leg.units())?)
            .execute(&mut *conn)
            .await
            .map_err(unknown)?;
    }
    Ok(())
}

fn as_of(at: AsOf) -> Option<String> {
    match at {
        AsOf::Latest => None,
        AsOf::At(t) => Some(t.to_string()),
    }
}

fn stored(units: u64) -> Result<i64, LedgerError> {
    i64::try_from(units).map_err(unknown)
}

fn cents(value: i64) -> Result<Usd, LedgerError> {
    u64::try_from(value).map(Usd::from_cents).map_err(corrupt)
}

fn from_micros(us: i64) -> Result<Timestamp, LedgerError> {
    Timestamp::from_microsecond(us).map_err(corrupt)
}

fn unknown(e: impl std::error::Error + Send + Sync + 'static) -> LedgerError {
    LedgerError::Unknown(Box::new(e))
}

fn corrupt(what: impl std::fmt::Display) -> LedgerError {
    LedgerError::Unknown(format!("unreadable ledger row: {what}").into())
}
