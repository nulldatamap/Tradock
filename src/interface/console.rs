
use action::Action;
use market::{Count, Failure};
use market_data::MarketData;
use agent::Agent;
use interface::Interface;
use circularbuf::CircularBuf;
use game::Game;

pub struct ConsoleInterface;

impl ConsoleInterface {
  pub fn new() -> ConsoleInterface {
      ConsoleInterface
  }
}

impl Interface<&'static str> for ConsoleInterface {

  fn user_turn( &mut self, game : &Game )
                      -> Result<bool, &'static str> {
    Ok( true )
  }

}
