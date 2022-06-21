use bls::Signature;
use crate::beacon_block_header;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct SignedBeaconBlockHeader {
    pub message: beacon_block_header::BeaconBlockHeader,
    pub signature: bls::Signature,
}
