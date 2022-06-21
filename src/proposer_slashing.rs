use serde_derive::{Deserialize, Serialize};
use crate::signed_beacon_block_header::SignedBeaconBlockHeader;
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}
