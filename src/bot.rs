use coward_exchange::*;
use rust_decimal::prelude::*;
use std::{fmt::Debug, time::Instant};

pub struct OpenPosition {
  amount: Decimal,
  entry_price: Decimal,
  entry_fee: Decimal,
  entry_date: Instant,
}

impl OpenPosition {
  pub fn close(self, exit_price: Decimal, exit_fee: Decimal) -> ClosedPosition {
    ClosedPosition {
      amount: self.amount,
      entry_price: self.entry_price,
      entry_date: self.entry_date,
      entry_fee: self.entry_fee,
      exit_price,
      exit_date: Instant::now(),
      exit_fee
    }
  }
}

pub struct ClosedPosition {
  amount: Decimal,
  entry_price: Decimal,
  entry_date: Instant,
  entry_fee: Decimal,
  exit_price: Decimal,
  exit_date: Instant,
  exit_fee: Decimal
}

impl ClosedPosition {
  pub fn was_profitable(&self) -> bool {
    let zero = Decimal::new(0, 0);
    zero - self.entry_fee - (self.entry_price * self.amount) + (self.exit_price * self.amount) - self.entry_fee > zero
  }
}

pub enum Position {
  PendingOpen, // todo add pending open fields
  Open(OpenPosition),
  Partial,
  PendingClose(OpenPosition), // todo: add closing fields
  Closed(ClosedPosition),
  Error, // TODO: position is not covered (if any trades/withdrawals were made manually)
}

// impl Position {
//   pub fn is_open(&self) -> bool {
//     match self {
//       Position::Open(_) => true,
//       Position::Closed(_) => false,
//     }
//   }
// }

pub struct CowardBot<TExchange: Exchange> {
  exchange: TExchange,
  positions: Vec<Position>
}

impl<TExchange: Exchange> CowardBot<TExchange>
where
  <TExchange as Exchange>::Error: Debug,
{
  pub fn create(exchange: TExchange) -> Self {
    CowardBot { exchange,
    positions: Vec::new() }
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

  pub fn signal_buy() {
    // let right_side_currency = BTC
    // let left_side_currency = ETH
    // get available BTC
    // get available ETH
    // get required ETH to cover all open positions
    // mark uncovered positions as error (we cannot close these positions since we don't have the ETH)
    // get equivalent BTC of positions based on entry_price - this is the amount of BTC used to enter those positions

    // if available BTC < trade_size, do nothing - we can't trade anymore
    // create buy order
    // if available BTC < trade_size, notify
  }

  pub fn signal_sell() {
    // let currency_right = BTC;
    // let currency_left = ETH;
    // let right_balance = get_balance(currency_right);
    // let left_Balance = get_balance(currency_left);
    // let left_cover_required = positions.get_cover_required();
    // positions.mark_uncovered(left_balance) // we cannot close these positions since we don't have the ETH
    // let right_spent = positions.get_spent(); // get equivalent BTC of positions based on entry_price - this is the amount of BTC used to enter those positions
    // let open_position_count = positions.get_open().count();
    // let possible_positions = get_possible_positions_count(right_balance);
    // let profitable_open_positions = positions.get_open_profitable(rate);


    // if available BTC < trade_size, do nothing - we can't trade anymore
    // create buy order
    // if available BTC < trade_size, notify
  }
}
