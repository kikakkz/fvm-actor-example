use fvm_ipld_encoding::RawBytes;
use base64;
use cid::Cid;
use std::convert::TryFrom;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
struct CidParams {
    cid: Cid
}

fn main() {
    let _cid = CidParams {
        cid: Cid::try_from("bafy2bzaceax3ounnbvdbkxa4divufisiz5ylmroka5gsfarg5nfnkfksdxmgq").unwrap(),
    };
    println!("{:?}", base64::encode_config(RawBytes::serialize(_cid).unwrap().bytes(), base64::STANDARD));
}
