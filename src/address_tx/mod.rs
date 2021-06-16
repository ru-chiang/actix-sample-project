impl AddressTx {
    pub const COLLECTION_NAME: &'static str = "address_balance";
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddressTx {
    #[serde(serialize_with = "serialize_object_id", rename = "_id")]
    id: Option<ObjectId>,
    address: String,
    : String,
}