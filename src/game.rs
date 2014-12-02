use std::rand::random;
use std::io::timer::sleep;
use std::time::Duration;
use std::io::fs::File;
use std::path::posix::Path;

use market::{Market, Count, Money};
use market_data::MarketData;
use agent::Agent;
use action::Action::{Buy, Sell, Pass};
use ai::AI;
use consoleinterface::ConsoleInterface;

pub type ContextHandle<'a> = &'a Game;

pub struct Game {
  pub agents_and_ai : Vec<( Agent, AI )>,
  pub player : Agent,
  pub markets : Vec<Market>,
}

impl Game {
  fn new() -> Game {
    let mut agents_and_ai = Vec::new();
    let mut player = Agent::new( "You".to_string(), 100. );
    for i in range( 0, 100u ) {
      let inital_funds = 100.; // Should be randomized
      agents_and_ai.push( ( Agent::new( format!( "agent#{}", i ), inital_funds )
                          , AI::make_random_ai( inital_funds ) ) );
    }
    let mut markets = vec![ Market::new( "Coal".to_string(), 1.5, 1000 )
                          , Market::new( "Icecream".to_string(), 0.3, 400 )
                          , Market::new( "Foodball players".to_string(), 23., 20 ) ];
    Game {
      agents_and_ai: agents_and_ai,
      player: player,
      markets: markets
    }
  }

  // Shows the user the information needed to make a decision
  // Get input from the user
  // Try to perform the action, and report back if it fails
  // Update the markets and make the AI trade
  // Repeat.
  fn start( mut self, interface : ConsoleInterface ) {
    // Collect inital market data
    for market in self.markets.iter_mut() {
      market.next_day();
    }
        
    while interface.user_turn( &self ).unwrap() {

      for &(ref mut agent, ref mut ai) in self.agents_and_ai.iter_mut() {
        ai.make_decision( agent, &mut self.markets );
      }

      for market in self.markets.iter_mut() {
        market.next_day();
      }
    }
  } 

}

fn display_market( m : &Market ) {
  println!( "{}:\n{} x ${} ( by {} )\n${}", m.name, m.assets
                                          , m.price, m.holders
                                          , m.assets as Money * m.price );
}

pub fn start_game() {
  Game::new().start( ConsoleInterface::new() );
}
