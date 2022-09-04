use std::io;

#[macro_use] extern crate serde;
#[macro_use] extern crate juniper;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
mod handlers;
mod schema;
mod db;
mod schema_graphql;
mod models;

use self::db::establish_connection;
use self::handlers::{graphql, playground, graphiql_route};
use self::schema_graphql::create_schema;

use std::sync::Arc;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env variables
    dotenv().ok();
    let host = "127.0.0.1";
    let port = "5000";

    // configure logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // create Juniper schema
    let schema = Arc::new(create_schema());

    // database connection pool
    let db_pool = establish_connection();


    println!("Starting GraphQL server at http://{}:{}", host, port);

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .app_data(schema.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive()) // allow all cross origin requests
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
              .default_service(web::route().to(|| async {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
