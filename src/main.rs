use binance::{
  account::*,
  api::*,
  config::*,
  errors::ErrorKind as BinanceLibErrorKind,
  general::*,
  market::*,
  model::{DayTickerEvent, KlineSummary},
  savings::*,
  userstream::*,
  websockets::*,
};
use serde::{Deserialize, Serialize};
use std::{
  error::Error,
  sync::atomic::{AtomicBool, Ordering},
};
mod bot;

#[derive(Serialize, Deserialize, Default)]
struct BinanceConfig {
  api_key: String,
  api_secret: String,
}

fn main() -> Result<(), confy::ConfyError> {
  let config: BinanceConfig = confy::load("coward-bot")?;

  // let api_key = Some((&config.api_key).into());
  // let api_secret = Some((&config.api_secret).into());

  // let savings: Savings = Binance::new(api_key, api_secret);

  // match savings.get_all_coins() {
  //   Ok(answer) => println!(
  //     "{:#?}",
  //     answer
  //       .into_iter()
  //       .filter(|p| p.coin == "ETH")
  //       .collect::<Vec<binance::model::CoinInfo>>()
  //   ),
  //   Err(e) => println!("Error: {}", e),
  // }

  // user_stream_websocket(&config.api_key);
  Ok(())
}

fn user_stream_websocket(key: &str) {
  let keep_running = AtomicBool::new(true); // Used to control the event loop
  let api_key_user = Some(key.into());
  let user_stream: UserStream = Binance::new(api_key_user, None);

  if let Ok(answer) = user_stream.start() {
    let listen_key = answer.listen_key;

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
      match event {
        WebsocketEvent::AccountUpdate(account_update) => {
          for balance in &account_update.balance {
            println!(
              "Asset: {}, free: {}, locked: {}",
              balance.asset, balance.free, balance.locked
            );
          }
        }
        WebsocketEvent::OrderTrade(trade) => {
          println!("{:?}", trade);
        }
        _ => (),
      };

      Ok(())
    });

    web_socket.connect(&listen_key).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
      println!("Error: {}", e);
    }
    user_stream.close(&listen_key).unwrap();
    web_socket.disconnect().unwrap();
    println!("Userstrem closed and disconnected");
  } else {
    println!("Not able to start an User Stream (Check your API_KEY)");
  }
}
