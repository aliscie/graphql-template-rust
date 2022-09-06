use actix_web::web::Data;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SimpleObject};
use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use tokio::sync::{mpsc, RwLock};
use std::collections::HashMap;
use std::io;
use std::sync::Arc;

#[macro_use]
extern crate serde;

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
mod db;
mod handlers;
mod models;
mod schema_graphql;

use self::handlers::{index, index_graphiql, index_ws};

use self::db::establish_connection;
use self::schema_graphql::{MutationRoot, QueryRoot, SubscriptionRoot};

#[derive(Debug)]
pub struct Shared{
    senders : RwLock<HashMap<i32, Vec<mpsc::Sender<Message>>>>
}

#[derive(Serialize, Clone, SimpleObject)]
pub struct Message {
    sender_id : i32,
    message: String,
    created_at: String
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // load .env variables
    dotenv().ok();
    let host = "127.0.0.1";
    let port = "5000";

    // configure logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // database connection pool
    let db_pool = establish_connection().await;
    let shared = Arc::new(Shared{ senders : RwLock::new(HashMap::new())});
    // running all the pending migrations
    let _ = Migrator::up(&db_pool, None).await;

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(db_pool)
        .data(shared)
        .finish();

    println!("Starting GraphQL server at http://{}:{}", host, port);

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive()) // allow all cross origin requests
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
