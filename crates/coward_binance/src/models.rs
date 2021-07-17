use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceTradeFee {
  pub symbol: String,
  pub maker_commission: Decimal,
  pub taker_commission: Decimal,
}
