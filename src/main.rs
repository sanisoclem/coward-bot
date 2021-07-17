use coward_binance::*;
use coward_exchange::*;
fn main() {
  // get trading fees
  let api_key = "9MvoV5LEbiRc6Kt045iHnwUbiwfved9pLHwblmPEB2qdUc0I905bGSY2wOCcS53P";
  let api_secret = "51jUmQLfvzbUzEwCRY8AHqZyrKOcLZbVg5XL0MS88ZjVWDDInQj79sc41jMdIhlm";
  let exchange = Binance::create(api_key, api_secret, BinanceEnvironment::Live);

  let test = exchange.get_trade_fees(&TradingPair::create(TickerSymbol::ETH, TickerSymbol::BTC));

  match test {
    Ok(x) => print!("It worked: {:?}", x),
    Err(e) => print!("What a failure {:?}", e),
  }
}
