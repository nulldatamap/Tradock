use action::Action;
use market::Failure;
use game::ContextHandle;

pub use interface::console::ConsoleInterface;

mod console;

pub type Response = Result<(), Failure>;

// TODO: When they fix associated types, change IError to be a associated type
pub trait Interface<E> {
  fn render_market_data( &mut self, ContextHandle ) -> Result<(), E>;
  fn get_user_action( &mut self, ContextHandle ) -> Result<Vec<Action>, E>;
  fn handle_response( &mut self, ContextHandle ) -> Result<bool, E>;
}

