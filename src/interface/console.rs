
use action::Action;
use market::{Count, Failure};
use market_data::MarketData;
use agent::Agent;
use interface::{Interface, Response};
use circularbuf::CircularBuf;
use game::Game;

pub struct ConsoleInterface;

impl ConsoleInterface {
  pub fn new() -> ConsoleInterface {
      ConsoleInterface
  }
}

impl Interface<&'static str> for ConsoleInterface {

  fn render_market_data( &mut self, handle : &Game )
                         -> Result<(), &'static str> {
    Err( "NÆH" )
  }

  fn get_user_action( &mut self, handle : &Game )
                      -> Result<Vec<Action>, &'static str> {
    Err( "NÆH" )
  }

  fn handle_response( &mut self, handle : &Game ) -> Result<bool, &'static str> {
    Err( "NÆH" )
  }

}
