use rust_decimal::prelude::*;
use std::{
  convert::{TryFrom, TryInto},
  fmt,
};

#[derive(Debug)]
pub struct Percent(Decimal);

impl Percent {
  pub fn from_decimal(value: Decimal) -> Percent {
    Percent(value)
  }

  pub fn from_decimal_max100(value: Decimal) -> Percent {
    Percent(value / Decimal::new(100, 0))
  }
}
#[derive(Debug)]
pub enum ExchangeError {
  UnsupportedTickerSymbol(String),
  HttpError(String),
  Unknown(String),
}

#[derive(Debug)]
pub enum TickerSymbol {
  ETH,
  BTC,
}
impl fmt::Display for TickerSymbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
impl TryFrom<&str> for TickerSymbol {
  type Error = ExchangeError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "ETH" => Ok(TickerSymbol::ETH),
      "BTC" => Ok(TickerSymbol::BTC),
      x => Err(ExchangeError::UnsupportedTickerSymbol(String::from(x))),
    }
  }
}

#[derive(Debug)]
pub struct TradingPair {
  left: TickerSymbol,
  right: TickerSymbol,
}

impl fmt::Display for TradingPair {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.left, self.right)
  }
}

impl TryFrom<&str> for TradingPair {
  type Error = ExchangeError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let (left, right) = value.split_at(value.len() / 2);

    Ok(TradingPair {
      left: left.try_into()?,
      right: right.try_into()?,
    })
  }
}

impl TradingPair {
  pub fn create(left: TickerSymbol, right: TickerSymbol) -> TradingPair {
    TradingPair { left, right }
  }
}

#[derive(Debug)]
pub struct TradeFee {
  pub pair: TradingPair,
  pub maker_commission: Percent,
  pub taker_commission: Percent,
}

pub trait Exchange {
  type Error;
  fn get_trade_fee(&self, pair: &TradingPair) -> Result<TradeFee, Self::Error>;
}
