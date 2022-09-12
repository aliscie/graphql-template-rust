use actix_web::web::Data;
use async_graphql::*;
use chrono::Utc;
use tokio::sync::{mpsc, RwLock};
use std::collections::HashMap;
use std::io;
use std::sync::Arc;


use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use async_graphql::EmptyMutation;

use serde::{Deserialize, Serialize};
use serde_json::Result;



// use serde::ser::Serialize;
// #[macro_use]
// extern crate serde;
// extern crate async_graphql;

#[derive(Debug)]
pub struct Shared {
    pub senders: RwLock<HashMap<i32, Vec<mpsc::Sender<Message>>>>
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct Message {
    sender_id: i32,
    message: String,
    created_at: String,
}



mod models;
mod mutation;
mod query;
mod subscription;

pub use models::*;
pub use query::*;
pub use subscription::*;
pub use mutation::*;