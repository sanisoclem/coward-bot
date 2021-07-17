use coward_binance::*;
use coward_exchange::*;
fn main() {
  // get trading fees
  let api_key = "";
  let api_secret = "";
  let exchange = Binance::create(api_key, api_secret, BinanceEnvironment::Live);

  let test = exchange.get_trade_fees(&TradingPair::create(TickerSymbol::ETH, TickerSymbol::BTC));

  match test {
    Ok(x) => print!("It worked: {:?}", x),
    Err(e) => print!("What a failure {:?}", e),
  }
}
