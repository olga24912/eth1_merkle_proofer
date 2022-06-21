use ssz_types::BitVector;
use ssz_types::typenum::U512;
use bls::AggregateSignature;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct SyncAggregate {
    pub sync_committee_bits: BitVector<U512>,
    pub sync_committee_signature: AggregateSignature,
}
