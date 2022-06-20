use ssz_types::BitVector;
use ssz_types::typenum::U512;
use bls::AggregateSignature;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: BitVector<U512>,
    pub sync_committee_signature: AggregateSignature,
}
