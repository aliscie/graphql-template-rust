use async_graphql::*;


use std::io;
use actix_web::web::Data;
use migration::{Migrator, MigratorTrait};

#[macro_use] extern crate serde;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, guard};
use dotenv::dotenv;
mod handlers;
mod db;
mod schema_graphql;
mod models;

use self::handlers::{index, index_graphiql};

use self::schema_graphql::{QueryRoot, MutationRoot};
use self::db::establish_connection;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // load .env variables
    dotenv().ok();
    let host = "0.0.0.0";
    let port = "5000";

    // configure logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();


    // database connection pool
    let db_pool = establish_connection().await;
    // running all the pending migrations
    let _ = Migrator::up(&db_pool, None).await;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db_pool)
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
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

