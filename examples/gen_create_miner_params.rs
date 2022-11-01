use base64::encode;
use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{/*to_vec,*/ BytesDe, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::sector::RegisteredPoStProof;

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, PartialEq)]
pub struct CreateMinerParams {
    pub owner: Address,
    pub worker: Address,
    pub control_addresses: Vec<Address>,
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer_id: Vec<u8>,

    pub multi_addresses: Vec<BytesDe>,
}

pub fn gen_create_miner_params() -> String {
    let create_miner_params = CreateMinerParams {
        owner: Address::new_id(888),
        worker: Address::new_id(999),
        control_addresses: Default::default(),
        window_post_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
        peer_id: vec![1, 2, 3],
        multi_addresses: vec![BytesDe(vec![1, 2, 3])],
    };

    encode(RawBytes::serialize(create_miner_params).unwrap().bytes())
}

fn main() {
    println!("create miner params {:?}", gen_create_miner_params());
}
