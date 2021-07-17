use coward_exchange::*;
use hmac::{Hmac, Mac, NewMac};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{
  convert::TryInto,
  fmt::Write,
  time::{SystemTime, UNIX_EPOCH},
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BinanceTradeFee {
  pub symbol: String,
  pub maker_commission: Decimal,
  pub taker_commission: Decimal,
}

pub enum BinanceEnvironment {
  TestNet,
  Live,
}

fn get_timestamp() -> u128 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis()
}
fn encode_hex(bytes: &[u8]) -> String {
  let mut s = String::with_capacity(bytes.len() * 2);
  for &b in bytes {
    write!(&mut s, "{:02x}", b).unwrap();
  }
  s
}

pub struct Binance {
  url: String,
  api_key: String,
  api_secret: String,
}

impl Binance {
  pub fn create(api_key: &str, api_secret: &str, env: BinanceEnvironment) -> Binance {
    Binance {
      url: match env {
        BinanceEnvironment::TestNet => "https://testnet.binance.vision".to_owned(),
        BinanceEnvironment::Live => "https://api.binance.com".to_owned(),
      },
      api_key: String::from(api_key),
      api_secret: String::from(api_secret),
    }
  }

  pub fn sign_bytes(&self, bytes: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
      .expect("HMAC can take key of any size");
    mac.update(bytes);
    mac.finalize().into_bytes().to_vec()
  }

  pub fn sign(&self, value: &str) -> String {
    let result = self.sign_bytes(value.as_bytes());
    encode_hex(&result)
  }
}

impl Exchange for Binance {
  fn get_trade_fees(&self, pair: &TradingPair) -> Result<TradeFee, ExchangeError> {
    let client = reqwest::blocking::Client::new();
    let query_string = format!("symbol={}&timestamp={}", pair, get_timestamp());
    let signature = self.sign(&query_string);
    let results = client
      .get(format!(
        "{}/sapi/v1/asset/tradeFee?{}&signature={}",
        self.url, query_string, signature
      ))
      .header("X-MBX-APIKEY", &self.api_key)
      .send()
      .unwrap()
      .json::<Vec<BinanceTradeFee>>()
      .unwrap();

    let result = &results[0];

    Ok(TradeFee {
      pair: result.symbol.as_str().try_into()?,
      maker_commission: Percent::from_decimal(result.maker_commission),
      taker_commission: Percent::from_decimal(result.taker_commission),
    })
  }
}
