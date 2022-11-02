use base64::encode;
use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{/*to_vec,*/ Cbor, BytesDe, RawBytes};
use fvm_shared::address::Address;
use fvm_shared::sector::RegisteredPoStProof;

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, PartialEq)]
pub struct CreateMinerParams {
    pub owner: Address,
    pub worker: Address,
    // pub control_addresses: Vec<Address>,
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer: Vec<u8>,

    pub multiaddrs: Vec<BytesDe>,
}

impl Cbor for CreateMinerParams {}

pub fn gen_create_miner_params() -> String {
    let create_miner_params = CreateMinerParams {
        owner: Address::new_id(888),
        worker: Address::new_id(999),
        // control_addresses: Default::default(),
        window_post_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
        peer: vec![1, 2, 3],
        multiaddrs: vec![BytesDe(vec![1, 2, 3])],
    };

    encode(RawBytes::serialize(create_miner_params).unwrap().bytes())
}

fn main() {
    println!("create miner params {:?}", gen_create_miner_params());
}

#[cfg(test)]
mod test {
    use base64::decode;
    use fvm_ipld_encoding::RawBytes;

    #[test]
    fn result() {
        let params = RawBytes::new(
            decode(
                // eD1SZWNlaXB0IGV4aXRfY29kZSAyMSwgcmV0dXJuX2RhdGE6IFJhd0J5dGVzIHsgIH0sIGdhc191c2VkOiAw
                "eD1SZWNlaXB0IGV4aXRfY29kZSAxNiwgcmV0dXJuX2RhdGE6IFJhd0J5dGVzIHsgIH0sIGdhc191c2VkOiAw",
            )
            .unwrap(),
        );
        println!("{:?}", params.deserialize::<String>().unwrap());
    }
}
