mod auth;
mod context;
mod error;
mod schema;

use crate::{
    auth::extract_bearer_token,
    context::{AuthContext, MyContext},
    schema::{mutation_root::MutationRoot, query_root::QueryRoot},
};
use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::Utc;
use schema::MySchema;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

const PORT: u16 = 8000;

async fn index(
    schema: web::Data<MySchema>,
    graphql_request: GraphQLRequest,
    request: HttpRequest,
) -> GraphQLResponse {
    let inner_req = graphql_request.into_inner();

    let auth_context = {
        let bearer_token = {
            if let Some(authorization) = request
                .headers()
                .get(actix_web::http::header::AUTHORIZATION)
            {
                extract_bearer_token(authorization)
            } else {
                None
            }
        };

        AuthContext { bearer_token }
    };

    let inner_req = inner_req.data(auth_context);
    info!(query = inner_req.query, "Handling GraphQL query.");
    schema.execute(inner_req).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let up_since = Utc::now();
    let context = MyContext { up_since };

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(context)
        .finish();

    info!("GraphiQL IDE: http://localhost:{PORT}");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
