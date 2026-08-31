use std::cmp::Ordering;

use uuid::Uuid;

use super::conflict::FieldConflict;
use super::envelope::FieldEnvelope;
use super::field::EncryptedField;
use super::fingerprint::fingerprint;

#[derive(Debug, Clone)]
pub struct FieldOutcome {
    pub winner: FieldEnvelope,
    pub conflict: Option<FieldConflict>,
}

pub fn resolve_field(
    entry_id: Uuid,
    field: EncryptedField,
    held: Option<FieldEnvelope>,
    incoming: FieldEnvelope,
) -> FieldOutcome {
    let Some(held) = held else {
        return FieldOutcome {
            winner: incoming,
            conflict: None,
        };
    };

    if held == incoming {
        return FieldOutcome {
            winner: held,
            conflict: None,
        };
    }

    let incoming_wins = match incoming.updated_at_ms.cmp(&held.updated_at_ms) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => incoming.device_id > held.device_id,
    };

    let (winner, loser) = if incoming_wins {
        (incoming, held)
    } else {
        (held, incoming)
    };

    let byte_len = i64::try_from(loser.ciphertext.len()).unwrap_or(i64::MAX);
    let conflict = FieldConflict {
        entry_id,
        field,
        winner_updated_at_ms: winner.updated_at_ms,
        loser_updated_at_ms: loser.updated_at_ms,
        loser_device_id: loser.device_id,
        loser_ciphertext_hash: fingerprint(&loser.ciphertext),
        loser_byte_len: byte_len,
    };

    FieldOutcome {
        winner,
        conflict: Some(conflict),
    }
}
