use crate::voluntary_exit::VoluntaryExit;
use bls::Signature;

pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: Signature,
}
