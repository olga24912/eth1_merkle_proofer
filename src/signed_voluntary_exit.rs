use crate::voluntary_exit::VoluntaryExit;
use bls::Signature;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: Signature,
}
