use actix_web::{web,  HttpRequest, HttpResponse, Result};
use async_graphql_actix_web::*;
use async_graphql::http::GraphiQLSource;

use crate::schema_graphql::QuerySchema;

pub async fn index(schema: web::Data<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:5000")
                .subscription_endpoint("ws://localhost:8000")
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<QuerySchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(QuerySchema::clone(&*schema)).start(&req, payload)
}
