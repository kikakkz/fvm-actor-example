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

fn main() {
    let _cid = CidParams {
        cid: Cid::try_from("bafy2bzaceax3ounnbvdbkxa4divufisiz5ylmroka5gsfarg5nfnkfksdxmgq").unwrap(),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_cid).unwrap().bytes(), base64::STANDARD));

    let params = RawBytes::new(base64::decode("j0YADQAAAABGAA0AAAAARgBVAAAAAEYAVQAAAABLAC/qVLPfaQQbqa1GAA0AAAAARgBVAAAAAEsAL+pUs99pBBuprYJXAKkHaQWIUwvXQb2iV2YUW8AQc/vet5RWAAjMvkwOXkCsBO0Lea6ncUmpBtcBygMD2CpYJwABcaDkAiDGonLq7utrTApIObEIAmVgD+1jb6DZjjE9EH9ymV33TRkhydgqWCcAAXGg5AIgyxTx3cXXx/zECh7CWgnT55YfMQzX+biIs6XYJzlxemn2").unwrap());
    println!("{:?}", params.deserialize::<PowerActorState>().unwrap());

    let params = RawBytes::new(base64::decode("g0MA6AdDAO8HQwDpBw==").unwrap());
    println!("{:?}", params.deserialize::<Vec<Address>>().unwrap());
}
