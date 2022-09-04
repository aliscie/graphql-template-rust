use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, route, get};
use juniper::http::{playground::playground_source, GraphQLRequest};
use sea_orm::DatabaseConnection;

use crate::schema_graphql::{create_context, Schema};
use juniper_actix::{graphiql_handler, playground_handler};

#[get("/playground")]
pub async fn playground() -> impl Responder {
    playground_handler("/graphql", None).await
}

#[get("/graphiql")]
pub async fn graphiql() -> impl Responder {
    graphiql_handler("/graphql", None).await
}


#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    pool: web::Data<DatabaseConnection>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = create_context(pool.as_ref().clone());
    let res = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}
