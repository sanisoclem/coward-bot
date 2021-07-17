use coward_exchange::*;
use std::fmt::Debug;

pub struct CowardBot<TExchange: Exchange> {
  exchange: TExchange,
}

impl<TExchange: Exchange> CowardBot<TExchange>
where
  <TExchange as Exchange>::Error: Debug,
{
  pub fn create(exchange: TExchange) -> Self {
    CowardBot { exchange }
  }

  pub fn run(&self) {
    let test = self
      .exchange
      .get_trade_fee(&TradingPair::create(TickerSymbol::ETH, TickerSymbol::BTC));

    match test {
      Ok(x) => print!("It worked: {:?}", x),
      Err(e) => print!("What a failure {:?}", e),
    }
  }
}
