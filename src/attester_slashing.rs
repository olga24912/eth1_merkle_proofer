use crate::indexed_attestation::IndexedAttestation;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}
