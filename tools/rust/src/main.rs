use fvm_ipld_encoding::RawBytes;
use base64;
use cid::Cid;
use std::convert::TryFrom;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_shared::sector::StoragePower;
use fvm_shared::smooth::FilterEstimate;
use fvm_shared::econ::TokenAmount;
use fvm_shared::bigint::bigint_ser;
use fvm_shared::clock::ChainEpoch;
use fvm_shared::address::Address;

use num_bigint::BigInt;

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

fn main() {
    let _cid = CidParams {
        cid: Cid::try_from("bafy2bzacea6bvgucghtd66eubqazpknwqqpfywtdgp5qxludjsa6tyd6cxwuy").unwrap(),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_cid).unwrap().bytes(), base64::STANDARD));

    let _withdraw = WithdrawalParams {
        amount: TokenAmount::from_nano(BigInt::from(3345000000 as i64)),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_withdraw).unwrap().bytes(), base64::STANDARD));

    let params = RawBytes::new(base64::decode("j0YACAAAAABGAAgAAAAARgBQAAAAAEYAUAAAAABLAAvSyfIuWJEFpk1GAAgAAAAARgBQAAAAAEsAC9LJ8i5YkQWmTYJYGAAK64ihzsmDd5PYL3LfvLjaILVstg/zdFcAAlF+YaH8UPCjlsJSKdm0zE69LDpNNAMC2CpYJwABcaDkAiD2gG8Ch5fPq5yTKKpBGFf5F1QQ7GTm9R/r6s0rE2MHxxkIZtgqWCcAAXGg5AIgYwx+zcj8/12XKisOfupB2/pKebaqrvlbsUSF2/Mfef72").unwrap());
    println!("{:?}", params.deserialize::<PowerActorState>().unwrap());

    let params = RawBytes::new(base64::decode("g0MA6AdDAO8HQwDpBw==").unwrap());
    println!("{:?}", params.deserialize::<Vec<Address>>().unwrap());

    let params = RawBytes::new(base64::decode("gdgqWCcAAXGg5AIg3e5WrKjR8Xsur3yO6gxuTWtXc0OTwF6X1mSVDcqkYn4=").unwrap());
    println!("{:?}", params.deserialize::<CidParams>().unwrap());

    let params = RawBytes::new(base64::decode("eEtXaXRoZHJhdyBXaXRoZHJhd2FsUGFyYW1zIHsgYW1vdW50OiBUb2tlbkFtb3VudCgwLjAwMDAwMDAwMzM0NSkgfSA9PiBmMDEwMTI=").unwrap());
    println!("{:?}", params.deserialize::<String>().unwrap());
}
