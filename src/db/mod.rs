use std::env;

use anyhow::anyhow;
use lazy_static::lazy_static;
use mongodb::{Client, Collection};

pub use service::*;

use crate::utils::common::ApplicationError;

mod service;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    let mongo_connection_string = get_connection_string();
    Client::with_uri_str(&mongo_connection_string)
        .expect("Failed to initialize standalone mongo client.")
}

fn get_connection_string() -> String {
    let host = env::var("MONGO_HOST").expect("MONGO_HOST env not set.");
    let port = env::var("MONGO_PORT").expect("MONGO_PORT env not set.");
    "mongodb://".to_owned() + &host + ":" + &port
}

pub fn collection(coll_name: &str) -> Collection {
    MONGO.database("actix_db").collection(coll_name)
}

impl From<mongodb::error::Error> for ApplicationError {
    fn from(e: mongodb::error::Error) -> Self {
        log::error!("mongodb error, {}", e.to_string());
        ApplicationError::InternalError { source: anyhow!(e) }
    }
}