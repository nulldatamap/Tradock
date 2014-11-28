
use agent::Agent;
use market_data::MarketData;
use self::Failure::{InsufficientAgentAssets
                   , InsufficientAgentFunds
                   , InsufficientMarketAssets};

pub type Money = f64;
pub type Count = u32;

#[deriving(Show)]
pub struct Market {
  pub name    : String,
  pub price   : Money,
  pub assets  : Count,
  pub buys    : Count,
  pub sells   : Count,
  pub holders : Count,
  pub data    : MarketData
}

#[deriving(Eq, PartialEq, Show)]
pub enum Failure {
  InsufficientAgentFunds,
  InsufficientMarketAssets,
  InsufficientAgentAssets
}

impl Market {
  
  pub fn new( name : String, starting_price : Money
            , starting_asset_count : Count )
              -> Market {
    
    Market{ name: name.clone(), price: starting_price
          , data: MarketData::new( name, default_time_frame() )
          , assets: starting_asset_count
          , buys: 0, sells: 0, holders: 1 }
  }


  // Takes an agent, and the amount of assets to buy, and tries to buy
  // then for the agent. This might fail though in variaous ways
  // see `market::Failure` ( in this file ) if you want to see them.
  pub fn buy_assets( &mut self, agent : &mut Agent, amount : Count )
      -> Result<(), Failure> {
    
    let price = self.price * (amount as f64);

    if agent.funds < price {
      return Err( InsufficientAgentFunds )
    }

    if self.assets < amount {
      return Err( InsufficientMarketAssets )
    }

    self.assets -= amount;
    self.buys += amount;

    agent.add_assets( &self.name, amount );
    agent.funds -= price;

    Ok( () )
  }


  // Takes an agent, the amount of assets to sell, and tries to sell
  // them from the agent. This might fail though if the agent doesn't
  // have the assets they are trying to sell.
  pub fn sell_assets( &mut self, agent : &mut Agent, amount : Count )
      -> Result<(), Failure> {
    
    let price = self.price * (amount as f64);

    if !agent.remove_assets( &self.name, amount ) {
      return Err( InsufficientAgentAssets )
    }

    self.assets += amount;
    self.sells += amount;

    agent.funds += price;

    Ok( () )
  }

  // Recalculates the price for the day and resets it's counting mechanisms
  // so it can calculate the growth acurately for next time.
  pub fn recalculate_price( &mut self ) {
    let prev_holders = self.holders as f64;

    self.holders += self.buys - self.sells;

    let holder_growth_rate = ( self.holders as f64 ) / prev_holders;
    self.price *= holder_growth_rate;

    self.buys = 0;
    self.sells = 0;
  }

  // Get the current data from the market as a 3-tuple of
  // the asset count, price of the stock and the amount of holders
  pub fn current_data( &self ) -> ( Count, Money, Count ) {
    ( self.assets, self.price, self.holders )
  }

  // Recalculates the price and collects the data for the day
  pub fn next_day( &mut self ) {
    self.recalculate_price();
    let cd = self.current_data();
    self.data.collect( cd );
  }

}

fn default_time_frame() -> Count {
  365
}
