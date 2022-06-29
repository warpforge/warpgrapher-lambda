use lambda_runtime::handler_fn;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use warpgrapher::engine::context::RequestContext;
use warpgrapher::engine::database::cypher::CypherEndpoint;
use warpgrapher::engine::database::DatabaseEndpoint;
use warpgrapher::{Configuration, Engine, Error};

static CONFIG: &str = "
version: 1
model:
  - name: User
    props:
      - name: name
        type: String
  - name: Team
    props: 
      - name: name
        type: String
";

#[derive(Clone, Debug)]
pub struct Rctx {}

impl Rctx {}

#[derive(Clone, Debug, Deserialize)]
pub struct AwsLambdaProxyRequest {
    pub body: String,
}

impl RequestContext for Rctx {
    type DBEndpointType = CypherEndpoint;

    fn new() -> Self {
        Rctx {}
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    pub variables: Option<serde_json::Value>,
}

pub async fn create_app_engine() -> Result<Engine<Rctx>, Error> {
    // parse config
    let config = Configuration::try_from(CONFIG.to_string())?;

    // define database endpoint
    let db = CypherEndpoint::from_env()?.pool().await?;

    // create warpgrapher engine
    let engine: Engine<Rctx> = Engine::new(config, db).build()?;

    Ok(engine)
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
async fn handler(event: Value, _: lambda_runtime::Context) -> Result<Value, Error> {
    let engine = create_app_engine().await?;

    let proxy_request: AwsLambdaProxyRequest = serde_json::from_value(event).unwrap();
    let gql_request: GraphqlRequest = serde_json::from_str(&proxy_request.body).unwrap();
    let metadata = HashMap::new();

    let result = engine
        .execute(
            gql_request.query.to_string(),
            gql_request.variables,
            metadata,
        )
        .await?;

    let response = format_proxy_response(result)?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
