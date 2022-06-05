use std::convert::Infallible;

use crate::schema;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Schema,
};
use async_graphql_warp::GraphQLResponse;
use serde_json::json;
use warp::{filters::BoxedFilter, http::Response, reply::json, Filter, Rejection, Reply};

// We are going to use Result cause its a type that warp can figure out and sand a OK or throw an Error
// This func will be a health check for the API so if its all ok its going to give us someting that impl Reply else a Rejection from Warp

/// Check if server is alive.
async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(
        &json!({"Rust API": "Vdbj+LpYb45tK3DmU1enKbB4OOB3iHarl8+ub7UPPpqHyN57dJzGL4rD3G9QGdBVb0jfpPdAmn+WtHjVQzYj28UOq3upoFderVJr3gNFQUdAc9oc1FBuQKfn58+YKbUl5xvutMQaSrDFT15DcWN9QYjgg+Z9xHNHaI9p6DhA71s="}),
    ))
}

// Boxed Filter is a Struct that is going to filter anything that comes into our routes as long as it have the type we are filtering
// warp::path::end() is going to define the response that our end of the rout will give us

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    // Build the Graphql Schema
    let schema = schema::build_graphql_schema().finish();

    let health = warp::path::end().and_then(health);

    //Graphql query and subscription handler.
    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        ),
    ));

    let graphql_playground = warp::path("playground").map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    // Now we have to bind all roots together
    // And we decide witch route our app will be getting
    health.or(graphql_handler).or(graphql_playground).boxed()
}
