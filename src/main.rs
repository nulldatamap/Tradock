extern crate sdl2;

use std::rand::random;

use market::{Market, Count};
use agent::Agent;

mod market;
mod agent;
#[cfg(test)]
mod agent_market_tests;

fn version() -> String {
  format!( "{}.{}.{}", env!( "CARGO_PKG_VERSION_MAJOR" )
                     , env!( "CARGO_PKG_VERSION_MINOR" )
                     , env!( "CARGO_PKG_VERSION_PATCH" ) )
} 

fn main() {
  println!( "Tradock v{}", version() );
  
}

