use blueprint_sdk::prelude::*;
use square_blueprint_lib::{
  context::MyContext,
  jobs::square::{square_number, SQUARE_JOB_ID},
};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
  // Load environment (from env vars / .env)
  let env = BlueprintEnvironment::load()?;

  // Signer & client
  let signer = env.keystore().first_local::<SpSr25519>()?;
  let pair   = env.keystore().get_secret::<SpSr25519>(&signer)?;
  let signer = TanglePairSigner::new(pair.0);
  let client = env.tangle_client().await?;

  // Streams for finalized blocks
  let producer = TangleProducer::finalized_blocks(client.rpc_client.clone()).await?;
  let consumer = TangleConsumer::new(client.rpc_client.clone(), signer);

  // Application context
  let ctx = MyContext::new(env.clone()).await?;

  // Run blueprint
  BlueprintRunner::builder(TangleConfig::default(), env)
    .router(
      Router::new()
        .route(SQUARE_JOB_ID, square_number.layer(TangleLayer))
        .with_context(ctx),
    )
    .producer(producer)
    .consumer(consumer)
    .run()
    .await
}
