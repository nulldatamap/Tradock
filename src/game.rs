use std::rand::{task_rng, Rng, random};

use market::{Money, Market};
use agent::Agent;
use ai::AI;
use consoleinterface::ConsoleInterface;
use randorditer::random_order;

pub type ContextHandle<'a> = &'a Game;

pub struct Game {
  pub agents_and_ai : Vec<( Agent, AI )>,
  pub player : Agent,
  pub markets : Vec<Market>,
}

// Both a shorthand, and also makes me not have to
// write .to_string() after each string.
macro_rules! market(
  ( $name:expr, $price:expr, $amount:expr ) => (
    Market::new( ($name).to_string(), $price, $amount )
  )
)

// The amount of money the player starts with
pub fn starting_funds() -> Money {
  10000.
}

fn make_player() -> Agent {
  Agent::new( "You".to_string()
            , starting_funds() ) // Starting funds
}

// Gets us 3 random markets
fn get_random_markets() -> Vec<Market> {
  let mut all_markets = vec![ market!( "Google"     , 0.2   , 1000000 )
                            , market!( "Microsoft"  , 0.15  , 1000000 )
                            , market!( "Apple"      , 0.19  , 1000000 )
                            , market!( "Maersk"     , 0.5   , 1000000 )
                            , market!( "Sony"       , 0.12  , 1000000 )
                            , market!( "Tesla"      , 0.11  , 1000000 )
                            , market!( "Bacon"      , 1.3   , 7000 )
                            , market!( "Icecream"   , 1.0   , 7000 )
                            , market!( "Apples"     , 0.7   , 7000 )
                            , market!( "Oranges"    , 0.8   , 7000 )
                            , market!( "Turkey"     , 0.9   , 7000 )
                            , market!( "Burgers"    , 0.95  , 7000 )
                            , market!( "Gold"       , 5.    , 8500 )
                            , market!( "Silver"     , 3.5   , 8500 )
                            , market!( "Copper"     , 3.7   , 8500 )
                            , market!( "Iron"       , 2.2   , 8500 )
                            , market!( "Aluminium"  , 1.9   , 8500 )
                            , market!( "Guns"       , 50.5  , 3000 )
                            , market!( "Oil"        , 105.  , 3750 )
                            , market!( "Gas"        , 100.  , 3500 )
                            , market!( "Coal"       , 95.   , 3500 )
                            , market!( "Nukes"      , 10000., 10 )
                            , market!( "Electronics", 7.5   , 7000 ) ];
  let mut rng = task_rng();
  // Shuffle the markets
  rng.shuffle( all_markets.as_mut_slice() );
  // Truncate the vector so we only got 3 markets
  all_markets.truncate( 3 );

  all_markets
}

impl Game {
  fn new() -> Game {
    let mut agents_and_ai = Vec::new();
    for i in range( 0, 100u ) {
      // Bots start with between 100 DKK to 1000000 DKK
      let inital_funds = 100. + 999900. * random();
      agents_and_ai.push( ( Agent::new( format!( "agent#{}", i ), inital_funds )
                          , AI::make_random_ai( inital_funds ) ) );
    }
    Game {
      agents_and_ai: agents_and_ai,
      player: make_player(),
      markets: get_random_markets()
    }
  }

  // Shows the user the information needed to make a decision
  // Get input from the user
  // Try to perform the action, and report back if it fails
  // Update the markets and make the AI trade
  // Repeat.
  fn start( mut self, mut interface : ConsoleInterface ) {
    // Collect initial market data
    for market in self.markets.iter_mut() {
      market.next_day();
    }

    // Give the player some starting information
    interface.print_overview( &self );
        
    while interface.user_turn( &mut self ).unwrap() {

      for &(ref mut agent, ref mut ai) in random_order( &mut self.agents_and_ai ) {
        ai.make_decision( agent, &mut self.markets );
      }

      for market in self.markets.iter_mut() {
        market.next_day();
      }
    }

    // Tell the player what they've accomplished
    interface.print_outcome( &self );

    // And that's it, game over.
  }

}

pub fn start_game() {
  Game::new().start( ConsoleInterface::new() );
}
