use action::Action;
use market::Failure;
use game::ContextHandle;

pub use interface::console::ConsoleInterface;

mod console;

// TODO: When they fix associated types, change IError to be a associated type
pub trait Interface<E> {
  fn user_turn( &mut self, ContextHandle ) -> Result<(), E>;
}

