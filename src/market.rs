
use agent::Agent;

pub type Money = f64;
pub type Count = i32;

pub struct Market {
  pub name   : String,
  pub price  : Money,
  pub assets : Count,
  pub buys   : Count,
  pub sells  : Count,
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
          , buys: 0, sells: 0
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

    self.buys += amount;
    self.assets -= amount;

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
    self.sells += 1;

    agent.funds += price;

    Ok( () )
  }

  pub fn recalculate_price( &mut self ) {

    let buy_to_sell_ratio = (self.buys as f64 + 1.)
                          / (self.sells as f64 + 1.);
    self.price = buy_to_sell_ratio * self.price;

    self.buys = 0;
    self.sells = 0;
  }

}
