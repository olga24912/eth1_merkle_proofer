use crate::indexed_attestation::IndexedAttestation;

pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}
