use coward_binance::*;
use coward_exchange::*;
use serde::{Deserialize, Serialize};

mod bot;

#[derive(Serialize, Deserialize, Default)]
struct BinanceConfig {
  api_key: String,
  api_secret: String,
}

fn main() -> Result<(), confy::ConfyError> {
  let config: BinanceConfig = confy::load("coward-bot")?;
  let exchange = Binance::create(
    &config.api_key,
    &config.api_secret,
    BinanceEnvironment::Live,
  );

  let bot = bot::CowardBot::create(exchange);

  bot.run();

  Ok(())
}
