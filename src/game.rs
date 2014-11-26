use std::rand::random;
use std::io::timer::sleep;
use std::time::Duration;

use market::{Market, Count, Money};
use market_data::MarketData;
use agent::Agent;
use action::Action::{Buy, Sell, Pass};
use ai;
use interface::{Interface, ConsoleInterface};

struct Game {
  market : Vec<Market>,
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
  let mut agents_and_ai = Vec::new();
  for i in range( 0, 100u ) {
    agents_and_ai.push( ( Agent::new( format!( "agent#{}", i ), 100. )
                        , make_random_ai() ) );
  }
  let mut markets = vec![ Market::new( "Coal".to_string(), 1.5, 1000 )
                        , Market::new( "Icecream".to_string(), 0.3, 400 )
                        , Market::new( "Foodball players".to_string(), 23., 20 ) ];
  loop {
    {
      let mds : Vec<&MarketData> = markets.iter().map( |x| &x.data ).collect();
      interface.render_market_data( mds.clone(), agents_and_ai[0].ref0() );
      println!( "=> {}", interface.get_user_action( mds, agents_and_ai[0].ref0() ) );
      println!( "<= {}", interface.handle_response( vec![] ) );
    }
    for market in markets.iter_mut() {
      for &(ref mut agent, ref ai) in agents_and_ai.iter_mut() {
        match ai.make_decision( &market.name, agent, &market.data ) {
          Buy( amount ) => { market.buy_assets( agent, amount ); },
          Sell( amount ) => { market.sell_assets( agent, amount ); },
          _ => {}
        };
      }
      market.next_day();
      sleep( Duration::milliseconds( 500 ) );
    }
    println!( "=======================" );
    sleep( Duration::milliseconds( 1000 ) );
  }
}
