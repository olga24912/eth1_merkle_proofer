use crate::indexed_attestation::IndexedAttestation;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}
