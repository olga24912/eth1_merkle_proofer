use bls::AggregateSignature;
use crate::attestation_data::AttestationData;
use ssz_types::BitList;
use ssz_types::typenum::U2048;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct Attestation {
    pub aggregation_bits: BitList<U2048>,
    pub data: AttestationData,
    pub signature: AggregateSignature,
}
