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
  // Create markets
  let mut markets = vec![ Market::new( "Coal".to_string(), 0.5, 10, 1. )
                        , Market::new( "Icecream".to_string(), 0.1, 200, 1. ) ];
  // Create agents
  let mut agents = vec![ Agent::new( "John".to_string(), 1000000. )
                       , Agent::new( "Zoey".to_string(),  500000. )
                       , Agent::new( "Olaf".to_string(),      10. )
                       , Agent::new( "Mark".to_string(),    1000. ) ];
  let mut user = Agent::new( "You".to_string(), 100. );
  // Get user input
  // Make AI bid
  for _ in range( 0, 100u ) {
    for agent in agents.iter_mut() {
      println!( "Agent {}:", agent.name );
      let choice = random::<uint>() % markets.len();
      let market = &mut markets.get_mut( choice );
      if random() {
        let amount = if agent.funds as Count == 0 {
          0
        } else {
          random::<Count>() % market.assets as Count
        };
        println!( "  Bought {} from {}", amount, market.name );
        println!( "  {}", market.buy_assets( agent, amount ) );
      } else {
        let amount = agent.assets.find( &market.name )
                                 .map( |val| random::<Count>() % ( *val + 1 ) )
                                 .unwrap_or( 0 );
        println!( "  Sold {} to {}", amount, market.name );
        println!( "  {}", market.sell_assets( agent, amount ) );
      }
    }
    // Progress day
    for market in markets.iter_mut() {
      market.recalculate_price();
      println!( "| {}", market );
    }

    for agent in agents.iter() {
      println!( ": {}", agent );
    }

    println!( "====================" );
  }
}

