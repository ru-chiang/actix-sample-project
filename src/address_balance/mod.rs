use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub use controller::*;
pub use service::*;

use crate::utils::common::*;

mod controller;
mod service;


impl AddressBalance {
    pub const COLLECTION_NAME: &'static str = "address_balance";
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddressBalance {
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    address: String,
    balance: String,
}