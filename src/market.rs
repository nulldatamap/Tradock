
use agent::Agent;

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
  pub price_factor : f64
}

#[deriving(Eq, PartialEq, Show)]
pub enum Failure {
  InsufficientAgentFunds,
  InsufficientMarketAssets,
  InsufficientAgentAssets
}

impl Market {
  
  pub fn new( name : String, starting_price : Money
            , starting_asset_count : Count, price_factor : f64 )
              -> Market {
    
    Market{ name: name, price: starting_price
          , assets: starting_asset_count
          , buys: 0, sells: 0, holders: 1
          , price_factor: price_factor }
  }

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

  pub fn recalculate_price( &mut self ) {
    let prev_holders = self.holders as f64;

    self.holders += self.buys - self.sells;

    let holder_growth_rate = ( self.holders as f64 ) / prev_holders;
    self.price *= holder_growth_rate ;

    self.buys = 0;
    self.sells = 0;
  }

}
