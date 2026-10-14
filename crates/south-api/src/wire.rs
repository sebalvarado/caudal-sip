use caudal_iso20022::{AppHdr, BusinessMessage, CreditTransfer, DecodeError, Document, StatusReport};
use caudal_payments::{Message, Order, Party, Report};
use caudal_types::{Bic, PaymentId, ReasonCode, ReceivedReason};
use jiff::Timestamp;

fn party(p: &Party) -> caudal_iso20022::Party {
    caudal_iso20022::Party { name: p.name.clone(), id: p.document.clone(), account: p.account.clone() }
}

fn domain_party(p: &caudal_iso20022::Party) -> Party {
    Party { name: p.name.clone(), document: p.id.clone(), account: p.account.clone() }
}

/// The payment's creation time travels in the group header; the application header carries the hop's.
pub fn to_iso(from: Bic, to: Bic, message: &Message, now: Timestamp, msg_id: String) -> BusinessMessage {
    let header = AppHdr { from, to, id: msg_id.clone(), created: now, possible_duplicate: false };
    let document = match message {
        Message::Order(o) => Document::CreditTransfer(CreditTransfer {
            msg_id,
            created: o.created,
            instr_id: o.id.to_string(),
            end_to_end_id: o.id.to_string(),
            amount: o.amount,
            instructing_agent: o.ordering_red,
            instructed_agent: o.beneficiary_red,
            debtor: party(&o.debtor),
            debtor_agent: o.ordering(),
            creditor: party(&o.creditor),
            creditor_agent: o.beneficiary,
        }),
        Message::Report(r) => Document::StatusReport(StatusReport {
            msg_id,
            created: now,
            original_instr_id: r.id.to_string(),
            original_debtor_agent: r.id.ordering(),
            status: r.state,
            reason: r.reason.map(ReceivedReason::Listed),
        }),
    };
    BusinessMessage { header, document }
}

pub fn from_iso(m: &BusinessMessage) -> Result<Message, DecodeError> {
    let id = |ordering: Bic, code: &str| {
        PaymentId::new(ordering, code)
            .map_err(|e| DecodeError { reason: ReasonCode::Ms01, detail: format!("payment id {code:?}: {e}") })
    };
    Ok(match &m.document {
        Document::CreditTransfer(t) => Message::Order(Order {
            id: id(t.debtor_agent, &t.instr_id)?,
            created: t.created,
            ordering_red: t.instructing_agent,
            beneficiary_red: t.instructed_agent,
            beneficiary: t.creditor_agent,
            amount: t.amount,
            debtor: domain_party(&t.debtor),
            creditor: domain_party(&t.creditor),
        }),
        Document::StatusReport(r) => Message::Report(Report {
            id: id(r.original_debtor_agent, &r.original_instr_id)?,
            state: r.status,
            reason: match r.reason {
                None => None,
                Some(ReceivedReason::Listed(code)) => Some(code),
                Some(ReceivedReason::Unlisted(_)) => Some(ReasonCode::Ff10),
            },
        }),
    })
}
