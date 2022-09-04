
use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::db::DbPool;
use crate::schema_graphql::{create_context, Schema};
use juniper_actix::{graphql_handler, playground_handler, graphiql_handler};


pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", Some("/subscriptions")).await
}

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    db_pool : web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let db_pool = (*db_pool).clone();
    let context = create_context(db_pool);
    graphql_handler(&schema, &context, req, payload).await
}

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}
