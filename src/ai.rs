use std::rand::random;
use std::num::Float;

use randorditer::random_order;
use market::{Money};
use market::Market;
use agent::Agent;

pub struct AI {
  // The lowest we sell
  low_threshold : f64,
  // The highest we buy
  high_threshold : f64,
  riskiness : f64,
  invested : Money,
  inital_funds : Money
}

// Takes the given percentage and generates a boolean
// with the specified chance of being true.
fn random_chance( procentage : f64 ) -> bool {
  random::<f64>() <= procentage
}

impl AI {

  pub fn make_random_ai() -> AI {
    AI{ low_threshold: 2.2 * random()
      , high_threshold: 0.75 * random()
      , invested: 0.
      , riskiness: 1. + 10. * random() }
  }

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
      let assets = agent.get_assets( &market.name.to_string() ) as f64;
      // If we have assets, consider selling them
      if assets > 0. {
        // Find out how much gain in % we would get
        let gain = assets * market.price / self.invested;
        // How likely are we to buy it ( in % ), do to our threshold?
        let sell_chance = self.low_threshold / gain;
        // Roll the dice
        if random_chance( sell_chance ) {
          // Re-adjust our investment stats
          self.invested -= market.price;
          // And sell at least 1 asset
          let amount = (random::<f64>() * assets).floor() as u32 + 1;
          let _ = market.sell_assets( agent, amount );
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
      // Find out how likely we are to buy it, based our threshold
      let buy_chance = self.high_threshold / price_scale;
      // Roll the dice once more
      if random_chance( buy_chance ) {
        // Remember our investment
        self.invested += market.price;
        let investment_price_limit = agent.funds * self.high_threshold;
        // And buy at least between 1 and the amount of assets
        // our investment limit price can buy us times the AIs riskiness
        let amount = ( market.price / investment_price_limit * random::<f64>()
                                    * self.riskiness ).floor() as u32 + 1;
        let _ = market.buy_assets( agent, amount );
      }
    }
  }
}
