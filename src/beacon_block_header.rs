use ethereum_types::H256;
use json::{parse, JsonValue};
use std::str::FromStr;

pub type Hash256 = H256;

pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body_root: Hash256,
}

impl BeaconBlockHeader {
    pub fn from_json(jv: json::JsonValue) -> BeaconBlockHeader {
         let mut beacon_h = BeaconBlockHeader{slot: 100, proposer_index: 100, parent_root: H256([0; 32]), state_root: H256([0; 32]), body_root: H256([0; 32])};
 
        match jv {
           json::JsonValue::Object(jv_inner) => {
               if let JsonValue::Short(ref jv_str) =  jv_inner["slot"] {
                   beacon_h.slot = jv_str.parse::<u64>().unwrap();
               }

               if let JsonValue::Short(ref jv_str) = jv_inner["proposerIndex"] {
                   beacon_h.proposer_index = jv_str.parse::<u64>().unwrap();
               }

               if let JsonValue::String(ref jv_str) = jv_inner["parentRoot"] {
                   beacon_h.parent_root = ethereum_types::H256::from_str(jv_str).unwrap();
               }

               if let JsonValue::String(ref jv_str) = jv_inner["stateRoot"] {
                   beacon_h.state_root = ethereum_types::H256::from_str(jv_str).unwrap();
               }

               if let JsonValue::String(ref jv_str) = jv_inner["bodyRoot"] {
                   beacon_h.body_root = ethereum_types::H256::from_str(jv_str).unwrap();
               }

           }
           _ => panic!(),
        };

        beacon_h
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn header_from_json() {
        let header_json = json::parse(r#"
        {
            "slot": "0",
            "proposerIndex": "1",
            "parentRoot": "0x1cfedbc04788917c188bdab08bf1ed4ece4f352782b61989e142a211fe876c4c",
            "stateRoot": "0xc40e5fae29997182dbafa0e091d41b27d9bbd6ac388df271e9224d3c0240017f",
            "bodyRoot": "0xb4d27c714e935a2970033c00ebb1d756336ded865e84fd22bec3395971158ab6"
        }
        "#);
       
        println!("{:?}", header_json);

        let bheader = crate::BeaconBlockHeader::from_json(header_json.unwrap());

        assert_eq!(bheader.slot, 0);
        assert_eq!(bheader.proposer_index, 1);
        assert_eq!(bheader.body_root[0], 0xb4);
        assert_eq!(format!("{:?}", bheader.body_root), "0xb4d27c714e935a2970033c00ebb1d756336ded865e84fd22bec3395971158ab6");
        assert_eq!(format!("{:?}", bheader.parent_root), "0x1cfedbc04788917c188bdab08bf1ed4ece4f352782b61989e142a211fe876c4c");
        assert_eq!(format!("{:?}", bheader.state_root), "0xc40e5fae29997182dbafa0e091d41b27d9bbd6ac388df271e9224d3c0240017f");
    }
}
