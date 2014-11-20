
use market::{Count, Money};
use action::Action;
use action::Action::{Buy, Sell, Pass};
use market_data::MarketData;
use agent::Agent;

pub struct AI {
  pub highest_buy : Money,
  pub lowest_sell : Money
}

impl AI {
  pub fn make_decision( &self, market: &String, agent: &Agent
                  , data: &MarketData ) -> Action {
    // Buy if you can
    // Sell if you can
    let price = match data.price_history.front() {
      Some( &p ) => p,
      None => return Pass
    };
    if agent.funds >= price && price <= self.highest_buy {
      return Buy( 1 )
    } else if price >= self.lowest_sell {
      match agent.assets.find( market ) {
        Some( sc ) => {
          return Sell( *sc )
        },
        None => return Pass
      }
    }
    Pass
  }
}
