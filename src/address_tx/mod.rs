use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub use service::*;

use crate::utils::common::*;

mod service;

impl AddressTx {
    pub const COLLECTION_NAME: &'static str = "address_tx";
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddressTx {
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    address: String,
    hash: String,
}

