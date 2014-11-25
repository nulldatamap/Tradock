use action::Action;
use market::Failure;
use agent::Agent;
use market_data::MarketData;

pub use interface::console::ConsoleInterface;

mod console;

pub type Response = Result<(), Failure>;

// TODO: When they fix associated types, change IError to be a associated type
pub trait Interface<E> {
  fn render_market_data( &mut self, Vec<&MarketData>, &Agent ) -> Result<(), E>;
  fn get_user_action( &mut self, Vec<&MarketData>, &Agent ) -> Result<Vec<Action>, E>;
  fn handle_response( &mut self, Vec<(&str, Response)> ) -> Result<bool, E>;
}

