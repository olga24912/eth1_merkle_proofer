use ssz_types::typenum::U2048;
use ssz_types::VariableList;
use crate::attestation_data::AttestationData;
use bls::AggregateSignature;

pub struct IndexedAttestation {
    /// Lists validator registry indices, not committee indices.
    pub attesting_indices: VariableList<u64, U2048>,
    pub data: AttestationData,
    pub signature: AggregateSignature,
}
