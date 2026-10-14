CREATE TABLE ledger.entry (
    seq          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    at           timestamptz NOT NULL UNIQUE,
    kind         text NOT NULL CHECK (kind IN ('reserved', 'posted', 'voided', 'transferred')),
    ordering     text,
    payment      text,
    beneficiary  text,
    amount       bigint CHECK (amount >= 0),
    fee          bigint CHECK (fee >= 0),
    tax          bigint CHECK (tax >= 0),
    reason       text,
    transfer_id  uuid UNIQUE,
    cause_event  uuid,
    cause_digest bytea CHECK (octet_length(cause_digest) = 32),
    CHECK ((payment IS NOT NULL AND ordering IS NOT NULL) = (kind <> 'transferred')),
    CHECK ((beneficiary IS NOT NULL AND amount IS NOT NULL AND fee IS NOT NULL AND tax IS NOT NULL) = (kind = 'reserved')),
    CHECK ((reason IS NOT NULL) = (kind = 'voided')),
    CHECK ((transfer_id IS NOT NULL) = (kind = 'transferred')),
    CHECK ((cause_event IS NOT NULL AND cause_digest IS NOT NULL) = (kind IN ('reserved', 'transferred')))
);

CREATE UNIQUE INDEX entry_payment_phase ON ledger.entry (ordering, payment, kind) WHERE payment IS NOT NULL;

CREATE TABLE ledger.leg (
    seq    bigint NOT NULL REFERENCES ledger.entry (seq),
    idx    smallint NOT NULL,
    debit  text NOT NULL,
    credit text NOT NULL,
    amount bigint NOT NULL CHECK (amount >= 0),
    PRIMARY KEY (seq, idx),
    -- Only headroom-book account names contain 'headroom'.
    CHECK ((debit LIKE '%headroom%') = (credit LIKE '%headroom%'))
);

CREATE VIEW ledger.posting AS
SELECT e.at, l.debit AS account,
       CASE e.kind WHEN 'reserved' THEN l.amount WHEN 'transferred' THEN 0 ELSE -l.amount END AS debits_pending,
       CASE e.kind WHEN 'posted' THEN l.amount WHEN 'transferred' THEN l.amount ELSE 0 END AS debits_posted,
       0::bigint AS credits_pending,
       0::bigint AS credits_posted
FROM ledger.entry e JOIN ledger.leg l USING (seq)
UNION ALL
SELECT e.at, l.credit,
       0, 0,
       CASE e.kind WHEN 'reserved' THEN l.amount WHEN 'transferred' THEN 0 ELSE -l.amount END,
       CASE e.kind WHEN 'posted' THEN l.amount WHEN 'transferred' THEN l.amount ELSE 0 END
FROM ledger.entry e JOIN ledger.leg l USING (seq);

CREATE INDEX leg_debit ON ledger.leg (debit);
CREATE INDEX leg_credit ON ledger.leg (credit);

DO $$
BEGIN
    CREATE ROLE ledger_runtime NOLOGIN;
EXCEPTION WHEN duplicate_object OR unique_violation THEN
    NULL;
END
$$;

GRANT USAGE ON SCHEMA ledger TO ledger_runtime;
GRANT SELECT, INSERT ON ledger.entry, ledger.leg TO ledger_runtime;
GRANT SELECT ON ledger.posting TO ledger_runtime;
