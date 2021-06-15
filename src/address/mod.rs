use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub use controller::*;
pub use service::*;

use crate::utils::common::*;

mod controller;
mod service;


impl Address {
    pub const COLLECTION_NAME: &'static str = "address";
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Address {
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    pub address: String,
    pub user_name: Option<String>,
}