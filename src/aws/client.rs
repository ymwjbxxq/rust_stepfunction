pub struct AWSConfig {
  config: aws_types::config::Config,
}

impl AWSConfig {
   pub fn set_config(config: aws_types::config::Config) -> Self {
    Self { 
      config: config 
    }
  }

  pub fn sf_client(&self) -> aws_sdk_sfn::Client {
    let sf_client = aws_sdk_sfn::Client::new(&self.config);
    return sf_client;
  }

}

#[derive(Clone)]
pub struct AWSClient {
  pub sf_client: aws_sdk_sfn::Client,
}
