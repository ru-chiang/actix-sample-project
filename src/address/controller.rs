use actix_web::web;
use bson::oid::ObjectId;

use crate::db::MongodbCrudService;
use crate::utils::common::{Resp, ResponseResult};

use super::{Address, ADDRESS_SERVICE};

pub async fn save(
    address: web::Json<Address>
) -> ResponseResult {
    let address: Address = address.into_inner();
    let res = ADDRESS_SERVICE.add_new_address(address)
        .await?;
    Resp::ok(res).to_json_result()
}

pub async fn get_all() -> ResponseResult {
    let res = ADDRESS_SERVICE.db_read_all_resources()
        .await?;
    Resp::ok(res).to_json_result()
}

pub async fn get(
    id: web::Path<String>
) -> ResponseResult {
    let res = ADDRESS_SERVICE.db_read_resource(&id)
        .await?;
    Resp::ok(res).to_json_result()
}

pub async fn update(
    id: web::Path<String>,
    address: web::Json<Address>,
) -> ResponseResult {
    let oid = ObjectId::with_string(&id).unwrap();
    let res = ADDRESS_SERVICE.update_by_oid(oid, &address)
        .await?;
    Resp::ok(res).to_json_result()
}


pub async fn remove(
    id: web::Path<String>
) -> ResponseResult {
    let res = ADDRESS_SERVICE.remove_by_oid(&id)
        .await?;
    Resp::ok(res).to_json_result()
}