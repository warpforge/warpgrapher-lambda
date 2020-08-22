use std::convert::TryFrom;
use warpgrapher::{Engine, Error};
use warpgrapher::engine::config::{Configuration};
use warpgrapher::engine::database::DatabaseEndpoint;
use warpgrapher::engine::database::cosmos::CosmosEndpoint;

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

pub async fn create_app_engine() -> Result<Engine, Error> {

   // parse config
   let config = Configuration::try_from(CONFIG.to_string())?;

   // define database endpoint
   let db = CosmosEndpoint::from_env()?.pool().await?;

   // create warpgrapher engine
   let engine: Engine<(), ()> = Engine::new(config, db).build()?;

   Ok(engine)
}