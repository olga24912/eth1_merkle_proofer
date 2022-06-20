use ssz_types::typenum::U2048;
use ssz_types::VariableList;
use crate::attestation_data::AttestationData;
use bls::AggregateSignature;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IndexedAttestation {
    /// Lists validator registry indices, not committee indices.
    #[serde(with = "quoted_variable_list_u64")]
    pub attesting_indices: VariableList<u64, U2048>,
    pub data: AttestationData,
    pub signature: AggregateSignature,
}

/// Serialize a variable list of `u64` such that each int is quoted. Deserialize a variable
/// list supporting both quoted and un-quoted ints.
///
/// E.g.,`["0", "1", "2"]`
mod quoted_variable_list_u64 {
    use super::*;
    use ssz_types::typenum::Unsigned;
    use eth2_serde_utils::quoted_u64_vec::{QuotedIntVecVisitor, QuotedIntWrapper};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S, T>(value: &VariableList<u64, T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Unsigned,
    {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for &int in value.iter() {
            seq.serialize_element(&QuotedIntWrapper { int })?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<VariableList<u64, T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Unsigned,
    {
        deserializer
            .deserialize_any(QuotedIntVecVisitor)
            .and_then(|vec| {
                VariableList::new(vec)
                    .map_err(|e| serde::de::Error::custom(format!("invalid length: {:?}", e)))
            })
    }
}
