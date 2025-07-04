use blueprint_sdk::prelude::*;

#[derive(Clone, TangleClientContext, ServicesContext)]
pub struct MyContext {
  #[config]
  pub env: BlueprintEnvironment,
}

impl MyContext {
  pub async fn new(env: BlueprintEnvironment) -> Result<Self> {
    Ok(Self { env })
  }
}
