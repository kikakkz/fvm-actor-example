mod blockstore;

use crate::blockstore::Blockstore;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR, BytesDe, Cbor};
use fvm_ipld_encoding::strict_bytes;
use fvm_sdk as sdk;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;
use fvm_shared::bigint::bigint_ser;
use fvm_shared::econ::TokenAmount;
use fvm_shared::sector::{RegisteredPoStProof, StoragePower};
use fvm_shared::clock::ChainEpoch;
use fvm_shared::smooth::FilterEstimate;
use fvm_ipld_hamt::Hamt;
use fvm_shared::{HAMT_BIT_WIDTH, METHOD_SEND};
use fvm_shared::address::Address;

/// A macro to abort concisely.
/// This should be part of the SDK as it's very handy.
macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

/// The actor's WASM entrypoint. It takes the ID of the parameters block,
/// and returns the ID of the return value block, or NO_DATA_BLOCK_ID if no
/// return value.
///
/// Should probably have macros similar to the ones on fvm.filecoin.io snippets.
/// Put all methods inside an impl struct and annotate it with a derive macro
/// that handles state serde and dispatch.
#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    // Conduct method dispatch. Handle input parameters and return data.
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(),
        2 => say_hello(),
        3 => get_state_cid(),
        4 => echo_raw_bytes(params),
        5 => get_state_cid_cbor(),
        6 => echo_cid_params(params),
        7 => get_old_state(params),
        8 => get_state_as_bytes(params),
        9 => get_power_actor_state(params),
        10 => get_current_balance(),
        11 => get_power_actor_miners(params),
        12 => withdraw(params),
        13 => create_miner(params),
        14 => fund_t04(params),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    // Insert the return data block if necessary, and return the correct
    // block ID.
    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

/// The constructor populates the initial state.
///
/// Method num 1. This is part of the Filecoin calling convention.
/// InitActor#Exec will call the constructor on method_num = 1.
pub fn constructor() -> Option<RawBytes> {
    // This constant should be part of the SDK.
    const INIT_ACTOR_ADDR: ActorID = 1;

    // Should add SDK sugar to perform ACL checks more succinctly.
    // i.e. the equivalent of the validate_* builtin-actors runtime methods.
    // https://github.com/filecoin-project/builtin-actors/blob/master/actors/runtime/src/runtime/fvm.rs#L110-L146
    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let state = State::default();
    state.save();
    None
}

/// Method num 2.
pub fn say_hello() -> Option<RawBytes> {
    let mut state = State::load();
    state.count += 1;
    state.save();

    let ret = to_vec(format!("Hello world #{}!", &state.count).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

/// Method num 3.
pub fn get_state_cid() -> Option<RawBytes> {
    let state_cid = sdk::sself::root().unwrap();
    Some(RawBytes::new(state_cid.to_bytes()))
}

/// Method num 4.
pub fn echo_raw_bytes(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);

    let ret = to_vec(format!("Params {:?}", params).as_str());

    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct CidParams {
    pub cid: Cid
}

/// Method num 5.
pub fn get_state_cid_cbor() -> Option<RawBytes> {
    let state_cid = sdk::sself::root().unwrap();
    let cid_for_cbor = CidParams {
        cid: state_cid,
    };
    Some(RawBytes::serialize(cid_for_cbor).unwrap())
}

/// Method num 6.
pub fn echo_cid_params(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: CidParams = params.deserialize().unwrap();

    let ret = to_vec(format!("Params {:?}", params).as_str());

    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

/// Method num 7.
pub fn get_old_state(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: CidParams = params.deserialize().unwrap();
    let old_state_cid = params.cid;

    let old_state = Blockstore.get_cbor::<State>(&old_state_cid).unwrap();
    Some(RawBytes::serialize(&old_state).unwrap())
}

/// Method num 8.
pub fn get_state_as_bytes(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: CidParams = params.deserialize().unwrap();
    let old_state_cid = params.cid;

    let old_state_vec = sdk::ipld::get(&old_state_cid).unwrap();
    Some(RawBytes::new(old_state_vec))
}

/// Storage power actor state
#[derive(Default, Serialize_tuple, Deserialize_tuple)]
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

/// Method num 9.
pub fn get_power_actor_state(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: CidParams = params.deserialize().unwrap();
    let state_cid = params.cid;

    let state = Blockstore.get_cbor::<PowerActorState>(&state_cid).unwrap();
    Some(RawBytes::serialize(&state).unwrap())
}

/// Method num 10.
pub fn get_current_balance() -> Option<RawBytes> {
    let balance = sdk::sself::current_balance();
    Some(RawBytes::serialize(balance.to_string()).unwrap())
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple, Clone, PartialEq)]
pub struct Claim {
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "bigint_ser")]
    pub raw_byte_power: StoragePower,
    #[serde(with = "bigint_ser")]
    pub quality_adj_power: StoragePower,
}

/// Method num 11.
pub fn get_power_actor_miners(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: CidParams = params.deserialize().unwrap();
    let state_cid = params.cid;

    let state = Blockstore.get_cbor::<PowerActorState>(&state_cid).unwrap().unwrap();
    let claims = Hamt::<Blockstore, _>::load_with_bit_width(&state.claims, Blockstore, HAMT_BIT_WIDTH).unwrap();
    let mut miners = Vec::new();
    claims.for_each(|k, _: &Claim| {
        miners.push(Address::from_bytes(&k.0)?);
        Ok(())
    }).ok()?;
    Some(RawBytes::serialize(&miners).unwrap())
}

#[derive(Debug, Deserialize_tuple)]
pub struct WithdrawalParams {
    pub amount: TokenAmount
}

/// Method num 12.
pub fn withdraw(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: WithdrawalParams = params.deserialize().unwrap();
    let caller = sdk::message::caller();
    let address = Address::new_id(caller);
    let send_params = RawBytes::default();

    let _receipt = fvm_sdk::send::send(
        &address,
        METHOD_SEND,
        send_params,
        params.amount.clone(),
    ).unwrap();

    let ret = to_vec(format!("Withdraw {:?} => f0{}", params, caller).as_str());
    
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CreateMinerParamsReq {
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer: Vec<u8>,
}
impl Cbor for CreateMinerParamsReq {}

#[derive(Serialize_tuple, Deserialize_tuple, Clone)]
pub struct CreateMinerParams {
    pub owner: Address,
    pub worker: Address,
    pub window_post_proof_type: RegisteredPoStProof,
    #[serde(with = "strict_bytes")]
    pub peer: Vec<u8>,
    pub multiaddrs: Vec<BytesDe>,
}
impl Cbor for CreateMinerParams {}

/// Method num 13.
pub fn create_miner(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let req: CreateMinerParamsReq = params.deserialize().unwrap();
    // caller: who invoke this contract
    let my_actor_id = sdk::message::receiver();
    let owner = Address::new_id(my_actor_id);
    let power_actor = Address::new_id(4);

    let params = CreateMinerParams {
        owner: owner,
        worker: owner,
        window_post_proof_type: req.window_post_proof_type,
        peer: req.peer,
        multiaddrs: Vec::new(),
    };
    let send_params = RawBytes::serialize(params).unwrap();

    let receipt = fvm_sdk::send::send(
        &power_actor,
        2,
        send_params,
        TokenAmount::from_atto(0),
    );

    match receipt {
        Ok(receipt) => {
            if !receipt.exit_code.is_success() {
                abort!(
                    USR_ILLEGAL_STATE,
                    "fail create miner"
                );
            }

            let ret = to_vec(
                format!(
                    "Receipt exit_code {}, return_data: {:?}, gas_used: {}",
                    receipt.exit_code,
                    // receipt.return_data.deserialize::<String>().unwrap(),
                    receipt.return_data,
                    receipt.gas_used,
                ).as_str(),
            );

            match ret {
                Ok(ret) => Some(RawBytes::new(ret)),
                Err(err) => {
                    abort!(
                        USR_ILLEGAL_STATE,
                        "failed to serialize return value: {:?}",
                        err
                    );
                }
            }
        },
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "fail create miner: {:?}",
                err
            );
        }
    }
}

/// Method num 14.
pub fn fund_t04(params: u32) -> Option<RawBytes> {
    let params = sdk::message::params_raw(params).unwrap().1;
    let params = RawBytes::new(params);
    let params: WithdrawalParams = params.deserialize().unwrap();
    let power_actor = Address::new_id(4);
    let send_params = RawBytes::default();

    let _receipt = fvm_sdk::send::send(
        &power_actor,
        METHOD_SEND,
        send_params,
        params.amount.clone(),
    ).unwrap();

    let ret = to_vec(format!("Withdraw {:?} => f04", params).as_str());

    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
