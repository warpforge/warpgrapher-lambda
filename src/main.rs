use lambda::handler_fn;
use serde_json;
use serde_json::{json, Value};
use std::collections::HashMap;
use warpgrapher::juniper::http::GraphQLRequest;
use warpgrapher::Error;

mod app;

/// parses the event object generated by the AWS Lambda proxy integration
/// and returns graphql request and metadata ready to be consumed by
/// the warpgrapher engine.
fn parse_proxy_request(event: Value) -> Result<(GraphQLRequest, HashMap<String, String>), Error> {
    let body_str = event.get("body").unwrap().as_str().unwrap();
    let request: GraphQLRequest = serde_json::from_str(body_str)
        .map_err(|e| Error::SerializationFailed { source: e})?;
    let metadata = HashMap::new();
    Ok((request, metadata))
}

/// formats the output of a warpgrapher engine execution and into the output
/// that the lambda proxy integration expects.
fn format_proxy_response(result: Value) -> Result<Value, Error> {
    Ok(json!({
      "body": serde_json::to_string(&result)
        .map_err(|e| Error::SerializationFailed { source: e})?,
      "headers": json!({}),
      "isBase64Encoded": false,
      "statusCode": 200
    }))
}

/// this function handles invocation of the lambda function.
async fn handler(event: Value, _: lambda::Context) -> Result<Value, Error> {
    let engine = app::create_app_engine().await?;
    let (graphql_request, metadata) = parse_proxy_request(event).unwrap();
    let result = engine.execute(&graphql_request, &metadata)?;
    let response = format_proxy_response(result)?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(handler);
    lambda::run(func).await?;
    Ok(())
}
