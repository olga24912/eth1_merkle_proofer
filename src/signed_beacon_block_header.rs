use bls::Signature;
use crate::beacon_block_header;

pub struct SignedBeaconBlockHeader {
    pub message: beacon_block_header::BeaconBlockHeader,
    pub signature: bls::Signature,
}
