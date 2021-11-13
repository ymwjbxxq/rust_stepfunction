use step_function::dtos::sf_payload::PayLoad;
use step_function::dtos::request::Request;
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{handler_fn, Error, Context};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use step_function::aws::client::{AWSClient, AWSConfig};
use std::sync::Arc;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Initialize service
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();
  
  let config = aws_config::load_from_env().await;
  let aws_client = AWSConfig::set_config(config);
  let sf_client = aws_client.sf_client();
  let aws_client = AWSClient {
    sf_client
  };

  lambda_runtime::run(handler_fn(|event: SqsEvent, ctx: Context| {
        execute(&aws_client, event, ctx)
    })) 
    .await?;

  Ok(())
}

pub async fn execute(client: &AWSClient, event: SqsEvent, _ctx: Context) -> Result<(), Error> {
  log::info!("EVENT {:?}", event);
  let mut tasks = Vec::with_capacity(event.records.len());
    let shared_client = Arc::from(client.clone());
    for record in event.records.into_iter() {
      let _shared_client = Arc::clone(&shared_client);
      tasks.push(tokio::spawn(async move {
        if let Some(body) = &record.body {
          let request: Request = serde_json::from_str(&body).unwrap();
          let result = start_stepfunction(&_shared_client, request).await;
          if result.is_err() {
            log::error!("Error: {:?}", result.err().unwrap());
          }
        } else {
          log::error!("Empty body {:?}", record);
        }
      }))
    }

    join_all(tasks).await;

  Ok(())
}

async fn start_stepfunction(client: &AWSClient, request: Request) -> Result<(), Error> { 
  let pay_load = PayLoad {
    is_hello_world_example: request.id
  };
  let state_machine_arn = std::env::var("STATE_MACHINE_ARN").expect("STATE_MACHINE_ARN must be set");

  let result = client.sf_client
    .start_execution()
    .set_state_machine_arn(Some(state_machine_arn))
    .input(serde_json::to_string(&pay_load).unwrap())
    .send()
    .await?;
  
  log::info!("RESULT {:?}", result);

  Ok(())
}


