use market::{Money};
use market::Market;
use agent::Agent;
use std::rand::random;
use randorditer::random_order;

pub struct AI {
  // The lowest we sell
  low_threshhold : f64,
  // The highest we buy
  high_threshhold : f64,
  invested : Money,
  inital_funds : Money
}

// Takes the given procentage and generates a boolean
// with the specified chance of being true.
fn random_chance( procentage : f64 ) -> bool {
  random::<f64>() <= procentage
}

impl AI {

  pub fn make_random_ai( inital_funds : Money ) -> AI {
    AI{ low_threshhold: 2.2 * random()
      , high_threshhold: 0.75 * random()
      , invested: 0.
      , inital_funds: inital_funds }
  }

  // Problems with the AI:
  // * It's still quite dump
  // * They only buy and sell one asset at a time
  // * They don't buy and sell in a random order

  // The only buying and selling one asset at a time thing might not be a
  // problem when we got so many bots trading at the same time.
  pub fn make_decision( &mut self, agent : &mut Agent, markets : &mut Vec<Market> ) {
    // A 10% chance that we won't trade today at all
    if random_chance( 0.10 ) {
      return
    }
    for market in random_order( markets ) {
      // A 10% chance that we won't even consider trading on this market today
      if random_chance( 0.10 ) {
        continue
      }
      // Get the how many assets we have in the current market
      let assets = agent.assets.get( &market.name.to_string() )
                               .map( |&v| v )
                               // Default to 0 if it's not an entry
                               .unwrap_or( 0 ) as f64;
      // If we have assets, consider selling them
      if assets > 0. {
        // Find out how much gain in % we would get
        let gain = assets * market.price / self.invested;
        // How likely are we to buy it ( in % ), do to our threshhold?
        let sell_chance = self.low_threshhold / gain;
        // Roll the dice
        if random_chance( sell_chance ) {
          // Re-adjust our investment stats
          self.invested -= market.price;
          // And sell the assets
          let _ = market.sell_assets( agent, 1 );
          println!( "{}: Sold some {}", agent.name, market.name );
          // We'll just skip to the next market, because it doesn't
          // make sense to buy from a market we just sold from.
          continue;
        }
      }
      // If there's no assets to buy on the market, just skip to the next
      if market.assets == 0 {
        continue
      }
      // Find out in % what a single asset corresponds to based on our funds
      let price_scale = market.price / agent.funds;
      // Find out how likely we are to buy it, based our threshhold
      let buy_chance = self.high_threshhold / price_scale;
      // Roll the dice once more
      if random_chance( buy_chance ) {
        // Remember our investment
        self.invested += market.price;
        // And buy the actual stock
        let _ = market.buy_assets( agent, 1 );
        println!( "{}: Bough some {}", agent.name, market.name );
      }
    }
  }
}
