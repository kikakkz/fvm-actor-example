use fvm_ipld_encoding::RawBytes;
use base64;
use cid::Cid;
use std::convert::TryFrom;
use fvm_ipld_encoding::{
    tuple::{Deserialize_tuple, Serialize_tuple},
    Cbor, strict_bytes,
};
use fvm_shared::sector::StoragePower;
use fvm_shared::smooth::FilterEstimate;
use fvm_shared::econ::TokenAmount;
use fvm_shared::bigint::bigint_ser;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::address::{
    Address,
};
use fvm_shared::sector::RegisteredPoStProof;
use num_bigint::BigInt;
use std::str::FromStr;
use libp2p::PeerId;
use fvm_ipld_encoding::BytesDe;

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
struct CidParams {
    cid: Cid
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct PowerActorState {
    #[serde(with = "bigint_ser")]
    pub total_raw_byte_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_bytes_committed: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_quality_adj_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub total_qa_bytes_committed: StoragePower,

    pub total_pledge_collateral: TokenAmount,

    #[serde(with = "bigint_ser")]
    pub this_epoch_raw_byte_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub this_epoch_quality_adj_power: StoragePower,

    pub this_epoch_pledge_collateral: TokenAmount,
    pub this_epoch_qa_power_smoothed: FilterEstimate,
    pub miner_count: i64,
    pub miner_above_min_power_count: i64,
    pub cron_event_queue: Cid,
    pub first_cron_epoch: ChainEpoch,
    pub claims: Cid,
    pub proof_validation_batch: Option<Cid>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct WithdrawalParams {
    pub amount: TokenAmount
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMinerParams {
    pub owner: Address,
    pub worker: Address,
    pub window_po_st_proof_type: RegisteredPoStProof,
    // 12D3KooWBRqtxhJCtiLmCwKgAQozJtdGinEDdJGoS5oHw7vCjMGc
    // pub peer: PeerId,
    pub peer: String,
    pub multiaddrs: Vec<BytesDe>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct CreateMinerParamsReq {
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer: Vec<u8>,
}
impl Cbor for CreateMinerParamsReq {}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct CreateMinerParamsReq1 {
    pub owner: Address,
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer: Vec<u8>,
}
impl Cbor for CreateMinerParamsReq1 {}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct CreateMinerReturn {
    /// Canonical ID-based address for the actor.
    pub id_address: Address,
    /// Re-org safe address for created actor.
    pub robust_address: Address,
}

fn main() {
    let _cid = CidParams {
        cid: Cid::try_from("bafy2bzacea6bvgucghtd66eubqazpknwqqpfywtdgp5qxludjsa6tyd6cxwuy").unwrap(),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_cid).unwrap().bytes(), base64::STANDARD));

    let _withdraw = WithdrawalParams {
        amount: TokenAmount::from_nano(BigInt::from(3345000000 as i64)),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_withdraw).unwrap().bytes(), base64::STANDARD));

    let params = PeerId::from_str("12D3KooWBRqtxhJCtiLmCwKgAQozJtdGinEDdJGoS5oHw7vCjMGc").unwrap().to_bytes();
    println!("encode {:?}", base64::encode_config(params.clone(), base64::STANDARD));

    let _create_miner = CreateMinerParams {
        owner: Address::new_id(1113),
        worker: Address::new_id(1055),
        window_po_st_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
        peer: base64::encode_config(PeerId::from_bytes(&params.clone()).unwrap().to_bytes(), base64::STANDARD),
        multiaddrs: Vec::new(),
    };

    println!("create miner params 1 {:?}", _create_miner);
    println!("create miner params 1 {:?}", base64::encode_config(RawBytes::serialize(_create_miner).unwrap().bytes(), base64::STANDARD));

    let _create_miner = CreateMinerParams {
        owner: Address::from_str("f3xcxrombanlaoax5kimk4bu53b3vjsdacy77qs5c2jgvxxywtggig2iuivpbme5dz62hivevynqr7ictwnaqq").unwrap(),
        worker: Address::from_str("f3xcxrombanlaoax5kimk4bu53b3vjsdacy77qs5c2jgvxxywtggig2iuivpbme5dz62hivevynqr7ictwnaqq").unwrap(),
        window_po_st_proof_type: RegisteredPoStProof::StackedDRGWindow32GiBV1,
        peer: base64::encode_config(PeerId::from_bytes(&params.clone()).unwrap().to_bytes(), base64::STANDARD),
        multiaddrs: Vec::new(),
    };
    println!("create miner params 2 original {:?}", _create_miner);
    println!("create miner params 2 {:?}", base64::encode(RawBytes::serialize(_create_miner).unwrap().bytes()));

    let _create_miner_req = CreateMinerParamsReq {
        window_post_proof_type: RegisteredPoStProof::StackedDRGWindow2KiBV1,
        peer: PeerId::from_bytes(&params.clone()).unwrap().to_bytes(),
    };
    println!("create miner params req 2 original {:?}", _create_miner_req);
    println!("create miner params req 2 {:?}", base64::encode(RawBytes::serialize(_create_miner_req).unwrap().bytes()));

    let _create_miner_req1 = CreateMinerParamsReq1 {
        owner: Address::new_id(100),
        window_post_proof_type: RegisteredPoStProof::StackedDRGWindow2KiBV1,
        peer: PeerId::from_bytes(&params.clone()).unwrap().to_bytes(),
    };
    println!("create miner params req 3 original {:?}", _create_miner_req1);
    println!("create miner params req 3 {:?}", base64::encode(RawBytes::serialize(_create_miner_req1).unwrap().bytes()));

    let params = RawBytes::new(base64::decode("j0YACAAAAABGAAgAAAAARgBQAAAAAEYAUAAAAABLAAvSyfIuWJEFpk1GAAgAAAAARgBQAAAAAEsAC9LJ8i5YkQWmTYJYGAAK64ihzsmDd5PYL3LfvLjaILVstg/zdFcAAlF+YaH8UPCjlsJSKdm0zE69LDpNNAMC2CpYJwABcaDkAiD2gG8Ch5fPq5yTKKpBGFf5F1QQ7GTm9R/r6s0rE2MHxxkIZtgqWCcAAXGg5AIgYwx+zcj8/12XKisOfupB2/pKebaqrvlbsUSF2/Mfef72").unwrap());
    println!("{:?}", params.deserialize::<PowerActorState>().unwrap());

    let params = RawBytes::new(base64::decode("g0MA6AdDAO8HQwDpBw==").unwrap());
    println!("{:?}", params.deserialize::<Vec<Address>>().unwrap());

    let params = RawBytes::new(base64::decode("gdgqWCcAAXGg5AIg3e5WrKjR8Xsur3yO6gxuTWtXc0OTwF6X1mSVDcqkYn4=").unwrap());
    println!("{:?}", params.deserialize::<CidParams>().unwrap());

    let params = RawBytes::new(base64::decode("eEtXaXRoZHJhdyBXaXRoZHJhd2FsUGFyYW1zIHsgYW1vdW50OiBUb2tlbkFtb3VudCgwLjAwMDAwMDAwMzM0NSkgfSA9PiBmMDEwMTI=").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("eD1SZWNlaXB0IGV4aXRfY29kZSAyMSwgcmV0dXJuX2RhdGE6IFJhd0J5dGVzIHsgIH0sIGdhc191c2VkOiAw").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("ACQIARIgA2SKyAMp0kUK16HKsNMHnhqgFVZwayIaMPc4yuz3lms=").unwrap());
    let params = PeerId::from_bytes(params.bytes()).unwrap();
    println!("decode {:?}", params.to_string());

    let params = PeerId::from_str(&params.to_string()).unwrap().to_bytes();
    println!("encode {:?}", base64::encode_config(params, base64::STANDARD));

    let params = RawBytes::new(base64::decode("eEJXaXRoZHJhdyBXaXRoZHJhd2FsUGFyYW1zIHsgYW1vdW50OiBUb2tlbkFtb3VudCgzLjM0NSkgfSA9PiBmMDEwNTM=").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("eD1SZWNlaXB0IGV4aXRfY29kZSAxOCwgcmV0dXJuX2RhdGE6IFJhd0J5dGVzIHsgIH0sIGdhc191c2VkOiAw").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("eHJSZWNlaXB0IGV4aXRfY29kZSAwLCByZXR1cm5fZGF0YTogUmF3Qnl0ZXMgeyA4MjQzMDBlZTA3NTUwMmNlNDE2ODM0NjM2MTczZWFhYjk0ODRkOTk2MjVjODcwMmYyOTQwNzUgfSwgZ2FzX3VzZWQ6IDA=").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());


    // let params = RawBytes::new(base64::decode("hVgxA7ivFzAgasDgX6pDFcDTuw7qmQwCx/8JdFpJq3vi0zGQbSKIq8LCdHn2joqSuGwj9FgxA7ivFzAgasDgX6pDFcDTuw7qmQwCx/8JdFpJq3vi0zGQbSKIq8LCdHn2joqSuGwj9AdYJgAkCAESIMsrq9DjnCJpGRxh6dLHaCfPMMB0Fvuz/51vnaEIU7hFgA==").unwrap());
    // println!("{:?}", params.deserialize::<CreateMinerParams>().unwrap());

    let params = RawBytes::new(base64::decode("eQGSUGFyYW1zIENyZWF0ZU1pbmVyUGFyYW1zIHsgb3duZXI6IEFkZHJlc3MgeyBwYXlsb2FkOiBJRCgxMTEzKSB9LCB3b3JrZXI6IEFkZHJlc3MgeyBwYXlsb2FkOiBJRCgxMDU1KSB9LCB3aW5kb3dfcG9fc3RfcHJvb2ZfdHlwZTogU3RhY2tlZERSR1dpbmRvdzMyR2lCVjEsIHBlZXI6IFsxMjAsIDUyLCA0OSwgNTAsIDY4LCA1MSwgNzUsIDExMSwgMTExLCA4NywgNjYsIDgyLCAxMTMsIDExNiwgMTIwLCAxMDQsIDc0LCA2NywgMTE2LCAxMDUsIDc2LCAxMDksIDY3LCAxMTksIDc1LCAxMDMsIDY1LCA4MSwgMTExLCAxMjIsIDc0LCAxMTYsIDEwMCwgNzEsIDEwNSwgMTEwLCA2OSwgNjgsIDEwMCwgNzQsIDcxLCAxMTEsIDgzLCA1MywgMTExLCA3MiwgMTE5LCA1NSwgMTE4LCA2NywgMTA2LCA3NywgNzEsIDk5XSB9").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("eNdDcmVhdGVNaW5lciBDcmVhdGVNaW5lclJldHVybiB7IGlkX2FkZHJlc3M6IEFkZHJlc3MgeyBwYXlsb2FkOiBJRCgxMDA3KSB9LCByb2J1c3RfYWRkcmVzczogQWRkcmVzcyB7IHBheWxvYWQ6IEFjdG9yKFs1NCwgMjM2LCAyMjUsIDE4LCAzMiwgMjMsIDI0NSwgMjAwLCAxMjMsIDExNCwgNTAsIDI1MywgMjYsIDEwOCwgMTcxLCAxMzgsIDIyMSwgMjM4LCAyNTUsIDExOF0pIH0gfQ==").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());

    let params = RawBytes::new(base64::decode("gkMA8gdVAoGNqG6MBkCrIwv1WdiYryBg3ue6").unwrap());
    println!("{:?}", params.deserialize::<CreateMinerReturn>().unwrap());

    let params = Address::new_id(1011);
    println!("create miner params req 3 {:?}", base64::encode(RawBytes::serialize(params).unwrap().bytes()));
}
