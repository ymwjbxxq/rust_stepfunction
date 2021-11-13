use serde::{Serialize};

#[derive(Serialize, Debug, Default)]
pub struct PayLoad {
  pub is_hello_world_example: String,
}