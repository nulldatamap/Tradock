
use market::{Count, Money};
use action::Action;
use action::Action::{Buy, Sell, Pass};
use market::Market;
use agent::Agent;
use std::rand::random;

pub struct AI {
  // The lowest we sell
  low_threshhold : f64,
  // The highest we buy
  high_threshhold : f64,
  invested : Money,
  inital_funds : Money
}

impl AI {

  pub fn make_random_ai( inital_funds : Money ) -> AI {
    AI{ low_threshhold: 2.2 * random()
      , high_threshhold: 0.75 * random()
      , invested: 0.
      , inital_funds: inital_funds }
  }

  pub fn make_decision( &mut self, agent : &mut Agent, markets : &mut Vec<Market> ) {
    // A 10% chance that we won't trade today at all
    if random::<f64>() > 0.90 {
      return
    }
    for market in markets.iter_mut() {
      // A 10% chance that we won't even consider trading on this market today
      if random::<f64>() > 0.90 {
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
        if random::<f64>() <= sell_chance {
          // Re-adjust our investment stats
          self.invested -= market.price;
          // And sell the assets
          market.sell_assets( agent, 1 );
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
      if random::<f64>() <= buy_chance {
        // Remember our investment
        self.invested += market.price;
        // And buy the actual stock
        market.buy_assets( agent, 1 );
        println!( "{}: Bough some {}", agent.name, market.name );
      }
    }
  }
}
