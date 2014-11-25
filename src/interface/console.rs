
use action::Action;
use market::{Count, Failure};
use market_data::MarketData;
use agent::Agent;
use interface::{Interface, Response};

/*
  
  NEEDS REFACTORING BADLY!

*/

use self::ffi::{CActionKind, CResponse, CAction
               , CPair, CMarketData, CAgent, CVec};

mod ffi {
  use market_data::MarketData;
  use agent::Agent;
  use interface::Response;
  use market::Failure;
  use circularbuf::CircularBuf;
  use libc::{c_int, c_double, uint32_t};

  pub type RawInterface = *mut ();
  type RawString = *const i8;
    
  #[repr(C)]
  pub struct CVec<T> {
    start : *mut T,
    end : *mut T,
    storage : *mut T
  }

  impl<T> CVec<T> {
    pub fn from_vec( mut vec : Vec<T> ) -> CVec<T> {
      let datas;
      let datae;
      unsafe {
        datas = vec.as_mut_ptr();
        datae = datas.offset( vec.len() as int );
      }
      CVec {
        start: datas,
        end: datae,
        storage: datae
      }
    }

    pub fn to_vec( &self ) -> Vec<T> {
      let len = self.end.to_uint() - self.start.to_uint();
      unsafe {
        Vec::from_raw_parts( self.start, len, len )
      }
    }
  }

  #[deriving(Show)]
  #[repr(C)]
  pub enum CActionKind {
    Ok,
    Retry,
    Failed
  }
  
  #[repr(C)]
  pub struct CAction {
    pub kind : CActionKind,
    pub amounts : CVec<i32>,
    pub failure : Option<&'static str>
  }

  impl CAction {
    pub fn as_error( &self ) -> String {
      self.failure.expect( "A non-null error message" ).to_string()
    }
  }
  
  #[repr(C)]
  pub enum CResponse {
    Success,
    InsufficientAgentFunds,
    InsufficientMarketAssets,
    InsufficientAgentAssets
  }

  impl CResponse {
    pub fn from_response( resp : Response ) -> CResponse {
      match resp {
        Ok( () ) => CResponse::Success,
        Err( fl ) => {
          match fl {
            Failure::InsufficientAgentAssets => CResponse::InsufficientAgentAssets,
            Failure::InsufficientMarketAssets => CResponse::InsufficientMarketAssets,
            Failure::InsufficientAgentFunds => CResponse::InsufficientAgentFunds
          }
        }
      }
    }
  }

  #[repr(C)]
  pub struct CPair<T> {
    pub key : RawString,
    pub value : T
  }

  #[repr(C)]
  pub struct CAgent {
    pub name : RawString,
    pub funds : c_double,
    pub assets : CVec<CPair<u32>>
  }

  impl CAgent {
    pub fn from_agent( agent : &Agent ) -> CAgent {
      let mut assets = Vec::with_capacity( agent.assets.len() );

      for (market, &invst) in agent.assets.iter() {
        assets.push( CPair{ key: market.to_c_str().as_ptr(), value: invst } );
      }

      CAgent{ name: agent.name.to_c_str().as_ptr()
            , funds: agent.funds
            , assets: CVec::from_vec( assets ) }
    } 
  }

  #[repr(C)]
  pub struct CMarketData {
    pub name : RawString,
    pub time_frame : uint32_t,
    pub day_count : uint32_t,
    pub asset_history : CircularBuf<uint32_t>,
    pub price_history : CircularBuf<c_double>,
    pub holders_history : CircularBuf<uint32_t>
  }

  impl CMarketData {
    pub fn from_market_data( data : &MarketData ) -> CMarketData {
      CMarketData{ name: data.name.to_c_str().as_ptr()
                 , time_frame: data.time_frame
                 , day_count: data.day_count
                 , asset_history: data.asset_history.clone()
                 , price_history: data.price_history.clone()
                 , holders_history: data.holders_history.clone() }
    }
  }
  
  extern "C" {
    pub fn create_interface() -> RawInterface;
    pub fn destroy_interface( ri : RawInterface );
    pub fn render_market_data( ri : RawInterface, md : CVec<CMarketData>, ag : CAgent )
                               -> CAction;
    pub fn get_user_action( ri : RawInterface, md : CVec<CMarketData>, ag : CAgent )
                            -> CAction;
    pub fn handle_response( ri : RawInterface, rs : CVec<CPair<CResponse>> ) -> CAction;
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

  fn render_market_data( &mut self, data : Vec<&MarketData>, agent : &Agent )
                         -> Result<(), String> {
    let result;
    unsafe {
      let cdata = data.iter()
                      .map( |&md| CMarketData::from_market_data( md ) )
                      .collect();
      result = ffi::render_market_data( self.raw_interface
                                      , CVec::from_vec( cdata )
                                      , CAgent::from_agent( agent ) );
    }
    match result.kind {
      CActionKind::Ok => {
        Ok( () )
      },
      CActionKind::Failed => {
        Err( result.as_error() )
      },
      k => {
        panic!( "[render_market_data] Got unexpected kind back: {}", k );
      }
    }
  }

  fn get_user_action( &mut self, data : Vec<&MarketData>, agent : &Agent )
                      -> Result<Vec<Action>, String> {
    let result;
    unsafe {
      let cdata = data.iter()
                      .map( |&md| CMarketData::from_market_data( md ) )
                      .collect();
      result = ffi::get_user_action( self.raw_interface
                                   , CVec::from_vec( cdata )
                                   , CAgent::from_agent( agent ) );
    }
    match result.kind {
      CActionKind::Ok => {
        Ok( result.amounts.to_vec().iter().map( |&x| {
          if x > 0 {
            Action::Buy( x as u32 )
          } else if x < 0 {
            Action::Sell( -x as u32 )
          } else {
            Action::Pass
          }
        } ).collect() )
      },
      CActionKind::Failed => {
        Err( result.as_error() )
      },
      k => {
        panic!( "[get_user_action] Got unexpected kind back: {}", k );
      }
    }
  }

  fn handle_response( &mut self, res : Vec<(&str, Response)> ) -> Result<bool, String> {
    let result;
    let responses = res.iter()
                       .map( |&(k, v)| CPair{
                          key: k.to_c_str().as_ptr(),
                          value: CResponse::from_response( v )
                        } ).collect();
    unsafe {
      result = ffi::handle_response( self.raw_interface
                                   , CVec::from_vec( responses ) );
    }
    match result.kind {
      CActionKind::Ok => {
        Ok( true )
      },
      CActionKind::Retry => {
        Ok( false )
      },
      CActionKind::Failed => {
        Err( result.as_error() )
      },
    }
  }

}
