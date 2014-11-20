
use action::Action;
use market::{Count, Failure};
use interface::{Interface, Response};

/*
  
  NEEDS REFACTORING BADLY!

*/

mod ffi {
  use libc::{c_int};

  pub type RawInterface = *mut ();
  
  #[deriving(Show)]
  #[repr(C)]
  pub enum CActionKind {
    Ok,
    Buy,
    Sell,
    Pass,
    Retry,
    Failed
  }
  
  #[repr(C)]
  pub struct CAction {
    pub kind : CActionKind,
    pub amount : c_int,
    pub failure : Option<&'static str>
  }
  
  #[repr(C)]
  pub enum CResponse {
    Ok,
    InsufficientAgentFunds,
    InsufficientMarketAssets,
    InsufficientAgentAssets
  }
  
  extern "C" {
    pub fn create_interface() -> RawInterface;
    pub fn destroy_interface( ri : RawInterface );
    pub fn render_market_data( ri : RawInterface ) -> CAction;
    pub fn get_user_action( ri : RawInterface) -> CAction;
    pub fn handle_response( ri : RawInterface, rs : CResponse ) -> CAction;
  }
}

pub struct ConsoleInterface {
  raw_interface : ffi::RawInterface
}

impl ConsoleInterface {
  pub fn new() -> ConsoleInterface {
    unsafe {
      ConsoleInterface{ raw_interface: ffi::create_interface() }
    }
  }
}

impl Drop for ConsoleInterface {
  fn drop( &mut self ) {
    unsafe {
      ffi::destroy_interface( self.raw_interface );
    }
  }
}

impl Interface<String> for ConsoleInterface {

  fn render_market_data( &mut self ) -> Result<(), String> {
    let result;
    unsafe {
      result = ffi::render_market_data( self.raw_interface );
    }
    match result.kind {
      ffi::CActionKind::Ok => {
        Ok( () )
      },
      ffi::CActionKind::Failed => {
        Err( result.failure.unwrap().to_string() )
      },
      k => {
        panic!( "[render_market_data] Got unexpected kind back: {}", k );
      }
    }
  }

  fn get_user_action( &mut self ) -> Result<Action, String> {
    let result;
    unsafe {
      result = ffi::get_user_action( self.raw_interface );
    }
    match result.kind {
      ffi::CActionKind::Buy => {
        Ok( Action::Buy( result.amount as Count ) )
      },
      ffi::CActionKind::Sell => {
        Ok( Action::Sell( result.amount as Count ) )
      },
      ffi::CActionKind::Pass => {
        Ok( Action::Pass )
      },
      ffi::CActionKind::Failed => {
        Err( result.failure.unwrap().to_string() )
      },
      k => {
        panic!( "[get_user_action] Got unexpected kind back: {}", k );
      }
    }
  }

  fn handle_response( &mut self, res : Response ) -> Result<bool, String> {
    let result;
    let response = match res {
      Ok( () ) => ffi::CResponse::Ok,
      Err( fl ) => {
        match fl {
          Failure::InsufficientAgentAssets => ffi::CResponse::InsufficientAgentAssets,
          Failure::InsufficientMarketAssets => ffi::CResponse::InsufficientMarketAssets,
          Failure::InsufficientAgentFunds => ffi::CResponse::InsufficientAgentFunds
        }
      }
    };
    unsafe {
      result = ffi::handle_response( self.raw_interface, response );
    }
    match result.kind {
      ffi::CActionKind::Ok => {
        Ok( true )
      },
      ffi::CActionKind::Retry => {
        Ok( false )
      },
      ffi::CActionKind::Failed => {
        Err( result.failure.unwrap().to_string() )
      },
      k => {
        panic!( "[handle_response] Got unexpected kind back: {}", k );
      }
    }
  }

}
