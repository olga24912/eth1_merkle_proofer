use bls::Signature;
use crate::beacon_block_header;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignedBeaconBlockHeader {
    pub message: beacon_block_header::BeaconBlockHeader,
    pub signature: bls::Signature,
}
