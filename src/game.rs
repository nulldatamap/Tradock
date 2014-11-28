use std::rand::random;
use std::io::timer::sleep;
use std::time::Duration;
use std::io::fs::File;
use std::path::posix::Path;

use market::{Market, Count, Money};
use market_data::MarketData;
use agent::Agent;
use action::Action::{Buy, Sell, Pass};
use ai;
use interface::{Interface, ConsoleInterface};

pub type ContextHandle<'a> = &'a Game;

pub struct Game {
  pub agents_and_ai : Vec<( Agent, ai::AI )>,
  pub player : Agent,
  pub markets : Vec<Market>,
}

impl Game {
  fn new() -> Game {
    let mut agents_and_ai = Vec::new();
    let mut player = Agent::new( "You".to_string(), 100. );
    for i in range( 0, 100u ) {
      agents_and_ai.push( ( Agent::new( format!( "agent#{}", i ), 100. )
                          , make_random_ai() ) );
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

  fn start( mut self, interface : &mut Interface<&'static str> ) {
    // Collect inital market data
    for market in self.markets.iter_mut() {
      market.next_day();
    }
    
    // Main game loop
    loop {
      // We unwrap all the interface cases, since we assume that the
      // error is unrecoverable and therefor there's nothing left to do
      // other than report the error and start the panic process.
      interface.render_market_data( &self ).unwrap();
    }
  } 

}

fn display_market( m : &Market ) {
  println!( "{}:\n{} x ${} ( by {} )\n${}", m.name, m.assets
                                          , m.price, m.holders
                                          , m.assets as Money * m.price );
}

fn make_random_ai() -> ai::AI {
  let low_bound = 1. + ( random::<Money>() * 50. );
  let high_bound = low_bound + ( random::<Money>() * 100000. );
  ai::AI{ lowest_sell: low_bound
        , highest_buy: high_bound }
}



pub fn start_game() {
  let mut interface = ConsoleInterface::new();
  Game::new().start( &mut interface );
}
