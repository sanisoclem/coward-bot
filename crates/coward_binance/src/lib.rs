mod models;
mod util;

use coward_exchange::*;
use std::convert::TryInto;
use util::*;

pub enum BinanceEnvironment {
  TestNet,
  Live,
}

#[derive(Debug)]
pub enum BinanceError {
  Http(reqwest::Error),
  Exchange(ExchangeError),
}
impl From<reqwest::Error> for BinanceError {
  fn from(err: reqwest::Error) -> Self {
    BinanceError::Http(err)
  }
}
impl From<ExchangeError> for BinanceError {
  fn from(err: ExchangeError) -> Self {
    BinanceError::Exchange(err)
  }
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
    sign_bytes(self.api_secret.as_bytes(), bytes)
  }

  pub fn sign(&self, value: &str) -> String {
    let result = self.sign_bytes(value.as_bytes());
    encode_hex(&result)
  }
}

impl Exchange for Binance {
  type Error = BinanceError;

  fn get_trade_fee(&self, pair: &TradingPair) -> Result<TradeFee, Self::Error> {
    let client = reqwest::blocking::Client::new();
    let query_string = format!("symbol={}&timestamp={}", pair, get_timestamp());
    let signature = self.sign(&query_string);
    let results = client
      .get(format!(
        "{}/sapi/v1/asset/tradeFee?{}&signature={}",
        self.url, query_string, signature
      ))
      .header("X-MBX-APIKEY", &self.api_key)
      .send()?
      .error_for_status()?
      .json::<Vec<models::BinanceTradeFee>>()?;

    let result = &results[0];

    Ok(TradeFee {
      pair: result.symbol.as_str().try_into()?,
      maker_commission: Percent::from_decimal(result.maker_commission),
      taker_commission: Percent::from_decimal(result.taker_commission),
    })
  }
}
