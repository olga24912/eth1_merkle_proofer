use crate::voluntary_exit::VoluntaryExit;
use bls::Signature;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: Signature,
}
