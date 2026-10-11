use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use caudal_types::Bic;
use ed25519_dalek::{Signature, Signer as _, SigningKey, Verifier as _, VerifyingKey};
use jiff::Timestamp;
use sha2::{Digest, Sha256};

pub fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

#[derive(Clone, PartialEq, Eq)]
pub struct Envelope {
    pub from: Bic,
    pub to: Bic,
    pub body: Vec<u8>,
    signature: [u8; 64],
}

impl fmt::Debug for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Envelope({} -> {}, {} bytes)", self.from, self.to, self.body.len())
    }
}

fn signed_bytes(from: Bic, to: Bic, body: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(body.len() + 24);
    for part in [from.as_str().as_bytes(), b"\0", to.as_str().as_bytes(), b"\0", body] {
        bytes.extend_from_slice(part);
    }
    bytes
}

pub struct Signer {
    bic: Bic,
    key: SigningKey,
}

impl Signer {
    /// A key derived from a seed, for in-process networks only. Real deployments load their keys.
    pub fn derive(bic: Bic, seed: &[u8]) -> Self {
        let mut material = seed.to_vec();
        material.extend_from_slice(bic.as_str().as_bytes());
        Self { bic, key: SigningKey::from_bytes(&digest(&material)) }
    }

    pub fn bic(&self) -> Bic {
        self.bic
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.key.verifying_key()
    }

    pub fn seal(&self, to: Bic, body: Vec<u8>) -> Envelope {
        let signature = self.key.sign(&signed_bytes(self.bic, to, &body)).to_bytes();
        Envelope { from: self.bic, to, body, signature }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rejected {
    UnknownSender,
    BadSignature,
}

impl fmt::Display for Rejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnknownSender => "unknown sender",
            Self::BadSignature => "bad signature",
        })
    }
}

impl std::error::Error for Rejected {}

#[derive(Debug, Clone, Default)]
pub struct Keyring(HashMap<Bic, VerifyingKey>);

impl Keyring {
    pub fn trust(&mut self, bic: Bic, key: VerifyingKey) {
        self.0.insert(bic, key);
    }

    pub fn open<'a>(&self, envelope: &'a Envelope) -> Result<&'a [u8], Rejected> {
        let key = self.0.get(&envelope.from).ok_or(Rejected::UnknownSender)?;
        let signature = Signature::from_bytes(&envelope.signature);
        key.verify(&signed_bytes(envelope.from, envelope.to, &envelope.body), &signature)
            .map_err(|_| Rejected::BadSignature)?;
        Ok(&envelope.body)
    }
}

/// Simulated time, shared by every node of an in-process network. It only moves forward.
#[derive(Debug, Clone)]
pub struct SimClock(Arc<AtomicI64>);

impl SimClock {
    pub fn starting_at(at: Timestamp) -> Self {
        Self(Arc::new(AtomicI64::new(at.as_microsecond())))
    }

    pub fn now(&self) -> Timestamp {
        Timestamp::from_microsecond(self.0.load(Ordering::SeqCst)).expect("set from a timestamp")
    }

    pub fn advance_to(&self, at: Timestamp) {
        self.0.fetch_max(at.as_microsecond(), Ordering::SeqCst);
    }
}

/// Events in simulated-time order; events at the same instant leave in the order they were scheduled.
#[derive(Debug, Clone)]
pub struct Schedule<E> {
    queue: BTreeMap<(Timestamp, u64), E>,
    next: u64,
}

impl<E> Default for Schedule<E> {
    fn default() -> Self {
        Self { queue: BTreeMap::new(), next: 0 }
    }
}

impl<E> Schedule<E> {
    pub fn at(&mut self, at: Timestamp, event: E) {
        self.queue.insert((at, self.next), event);
        self.next += 1;
    }

    pub fn pop(&mut self) -> Option<(Timestamp, E)> {
        self.queue.pop_first().map(|((at, _), e)| (at, e))
    }

    pub fn peek_time(&self) -> Option<Timestamp> {
        self.queue.first_key_value().map(|((at, _), _)| *at)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::SignedDuration;

    fn bic(s: &str) -> Bic {
        s.parse().unwrap()
    }

    #[test]
    fn a_sealed_envelope_opens_only_unchanged_and_from_a_trusted_sender() {
        let alfa = Signer::derive(bic("RALFECEQ"), b"seed");
        let mut keyring = Keyring::default();
        let envelope = alfa.seal(bic("SIPCECEQ"), b"<pacs/>".to_vec());
        assert_eq!(keyring.open(&envelope), Err(Rejected::UnknownSender));
        keyring.trust(alfa.bic(), alfa.verifying_key());
        assert_eq!(keyring.open(&envelope), Ok(&b"<pacs/>"[..]));
        let mut tampered = envelope.clone();
        tampered.body[1] = b'P';
        assert_eq!(keyring.open(&tampered), Err(Rejected::BadSignature));
        let mut redirected = envelope;
        redirected.to = bic("RBETECEQ");
        assert_eq!(keyring.open(&redirected), Err(Rejected::BadSignature));
        let impostor = Signer::derive(bic("RALFECEQ"), b"other seed");
        assert_eq!(keyring.open(&impostor.seal(bic("SIPCECEQ"), vec![])), Err(Rejected::BadSignature));
    }

    #[test]
    fn schedule_orders_by_time_then_by_arrival() {
        let t0: Timestamp = "2026-10-12T14:00:00Z".parse().unwrap();
        let mut s = Schedule::default();
        s.at(t0 + SignedDuration::from_secs(2), "late");
        s.at(t0, "first");
        s.at(t0, "second");
        let order: Vec<_> = std::iter::from_fn(|| s.pop()).map(|(_, e)| e).collect();
        assert_eq!(order, ["first", "second", "late"]);
    }

    #[test]
    fn the_clock_never_goes_back() {
        let t0: Timestamp = "2026-10-12T14:00:00Z".parse().unwrap();
        let clock = SimClock::starting_at(t0);
        clock.advance_to(t0 + SignedDuration::from_secs(5));
        clock.advance_to(t0);
        assert_eq!(clock.now(), t0 + SignedDuration::from_secs(5));
    }
}
