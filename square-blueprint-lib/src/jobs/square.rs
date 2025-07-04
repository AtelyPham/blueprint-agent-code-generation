use blueprint_sdk::{
  prelude::*,
  tangle::{TangleArgs1, TangleLayer, TangleResult},
};

pub const SQUARE_JOB_ID: u64 = 0;

#[debug_job]                       // automatically logs entry/exit + timing
pub async fn square_number(
  Context(_ctx): Context<crate::context::MyContext>,
  TangleArgs1(value): TangleArgs1<i64>,
) -> Result<TangleResult<i64>> {
  let squared = value * value;
  info!(original = value, squared, "Squared number");
  Ok(TangleResult::ok(squared))
}
