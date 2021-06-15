use actix_web::{Error, http, HttpResponse, web};

use crate::db::MongodbCrudService;

use super::{Address, ADDRESS_SERVICE};
use crate::utils::common::{RespResult, Resp, BusinessError};
use bson::oid::ObjectId;

pub async fn save(
    address: web::Json<Address>
) -> RespResult {
    let address: Address = address.into_inner();
    let res = ADDRESS_SERVICE.db_create_resource(address)
        .await?;
    Resp::ok(res).to_json_result()
}

pub async fn get_all() -> RespResult {
    let res = ADDRESS_SERVICE.db_read_all_resources()
        .await?;
    Resp::ok(res).to_json_result()
}


pub async fn get(
    id: web::Path<String>
) -> RespResult {
    let res = ADDRESS_SERVICE.db_read_resource(&id)
        .await?;
    Resp::ok(res).to_json_result()
}

pub async fn update(
    id: web::Path<String>,
    address: web::Json<Address>
) -> RespResult {
    let oid = ObjectId::with_string(&id).unwrap();
    let res = ADDRESS_SERVICE.update_by_oid(oid, &address)
        .await?;
    Resp::ok(res).to_json_result()
}


pub async fn remove(
    id: web::Path<String>
) -> RespResult {
    let res = ADDRESS_SERVICE.remove_by_oid(&id)
        .await?;
    Resp::ok(res).to_json_result()
}