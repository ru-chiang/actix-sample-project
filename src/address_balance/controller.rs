use actix_web::{Error, http, HttpResponse, web};
use actix_web::dev::Service;
use reqwest::Response;

use crate::db::MongodbCrudService;

use super::{ADDRESS_BALANCE_SERVICE, AddressBalance};


pub async fn update_balance() -> Result<HttpResponse, Error> {
    let res = ADDRESS_BALANCE_SERVICE.refresh_balance();
    (Ok(actix_web::HttpResponse::from("jj")))
}

