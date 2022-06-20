use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub const GRAFFITI_BYTES_LEN: usize = 32;

#[derive(Deserialize, Serialize)]
pub struct Graffiti(#[serde(with = "serde_graffiti")] pub [u8; GRAFFITI_BYTES_LEN]);

pub mod serde_graffiti {
    use super::*;

    pub fn serialize<S>(bytes: &[u8; GRAFFITI_BYTES_LEN], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&eth2_serde_utils::hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; GRAFFITI_BYTES_LEN], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let bytes = eth2_serde_utils::hex::decode(&s).map_err(D::Error::custom)?;

        if bytes.len() != GRAFFITI_BYTES_LEN {
            return Err(D::Error::custom(format!(
                "incorrect byte length {}, expected {}",
                bytes.len(),
                GRAFFITI_BYTES_LEN
            )));
        }

        let mut array = [0; GRAFFITI_BYTES_LEN];
        array[..].copy_from_slice(&bytes);

        Ok(array)
    }
}
