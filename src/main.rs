use std::io;
use actix_web::web::Data;

#[macro_use] extern crate serde;
#[macro_use] extern crate juniper;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
mod handlers;
mod db;
mod schema_graphql;
mod models;

use self::handlers::{graphql, playground, graphiql};
use self::schema_graphql::create_schema;

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


    println!("Starting GraphQL server at http://{}:{}", host, port);

    // start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(create_schema()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive()) // allow all cross origin requests
            .service(graphql)
            .service(playground)
            .service(graphiql)
              .default_service(web::route().to(|| async {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
