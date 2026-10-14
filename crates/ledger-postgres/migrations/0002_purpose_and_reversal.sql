-- A transfer names its scheme event, and a reversal is a new entry that mirrors one.
-- Rows written before purposes existed are marked correction: the purpose was not stored.

ALTER TABLE ledger.entry ADD COLUMN purpose text;
ALTER TABLE ledger.entry ADD COLUMN reverses uuid;

UPDATE ledger.entry SET purpose = 'correction' WHERE kind = 'transferred' AND purpose IS NULL;

DO $$
DECLARE
    name text;
BEGIN
    FOR name IN
        SELECT con.conname
        FROM pg_constraint con
        WHERE con.conrelid = 'ledger.entry'::regclass
          AND con.contype = 'c'
          AND pg_get_constraintdef(con.oid) LIKE '%kind%'
    LOOP
        EXECUTE format('ALTER TABLE ledger.entry DROP CONSTRAINT %I', name);
    END LOOP;
END $$;

ALTER TABLE ledger.entry ADD CONSTRAINT entry_kind_check
    CHECK (kind IN ('reserved', 'posted', 'voided', 'transferred', 'reversed'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_payment_check
    CHECK ((payment IS NOT NULL AND ordering IS NOT NULL) = (kind IN ('reserved', 'posted', 'voided')));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_reserve_fields_check
    CHECK ((beneficiary IS NOT NULL AND amount IS NOT NULL AND fee IS NOT NULL AND tax IS NOT NULL) = (kind = 'reserved'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_reason_check
    CHECK ((reason IS NOT NULL) = (kind = 'voided'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_transfer_id_check
    CHECK ((transfer_id IS NOT NULL) = (kind = 'transferred'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_cause_check
    CHECK ((cause_event IS NOT NULL AND cause_digest IS NOT NULL) = (kind IN ('reserved', 'transferred', 'reversed')));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_purpose_check
    CHECK ((purpose IS NOT NULL) = (kind = 'transferred'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_purpose_value_check
    CHECK (purpose IS NULL OR purpose IN ('headroom', 'settlement', 'suspense', 'correction'));
ALTER TABLE ledger.entry ADD CONSTRAINT entry_reverses_check
    CHECK ((reverses IS NOT NULL) = (kind = 'reversed'));

ALTER TABLE ledger.entry ADD CONSTRAINT entry_reverses_key UNIQUE (reverses);
ALTER TABLE ledger.entry ADD CONSTRAINT entry_reverses_fk FOREIGN KEY (reverses) REFERENCES ledger.entry (transfer_id);

CREATE OR REPLACE VIEW ledger.posting AS
SELECT e.at, l.debit AS account,
       CASE e.kind WHEN 'reserved' THEN l.amount WHEN 'transferred' THEN 0 WHEN 'reversed' THEN 0 ELSE -l.amount END AS debits_pending,
       CASE e.kind WHEN 'posted' THEN l.amount WHEN 'transferred' THEN l.amount WHEN 'reversed' THEN l.amount ELSE 0 END AS debits_posted,
       0::bigint AS credits_pending,
       0::bigint AS credits_posted
FROM ledger.entry e JOIN ledger.leg l USING (seq)
UNION ALL
SELECT e.at, l.credit,
       0, 0,
       CASE e.kind WHEN 'reserved' THEN l.amount WHEN 'transferred' THEN 0 WHEN 'reversed' THEN 0 ELSE -l.amount END,
       CASE e.kind WHEN 'posted' THEN l.amount WHEN 'transferred' THEN l.amount WHEN 'reversed' THEN l.amount ELSE 0 END
FROM ledger.entry e JOIN ledger.leg l USING (seq);
