
use circularbuf::CircularBuf;
use market::{Count, Money, Market};

#[deriving(Show)]
pub struct MarketData {
  pub name : String,
  pub time_frame : Count,
  pub day_count : Count,
  pub asset_history : CircularBuf<Count>,
  pub price_history : CircularBuf<Money>,
  pub holders_history : CircularBuf<Count>
}

impl MarketData {

  pub fn new( name : String, time_frame : Count ) -> MarketData {
    let tf = time_frame as uint;
    MarketData{ name: name, time_frame: time_frame, day_count: 0
              , asset_history: CircularBuf::new( tf )
              , price_history: CircularBuf::new( tf )
              , holders_history: CircularBuf::new( tf ) }
  }

  // Collects a set of data, asset count, price and holder count
  // All boundled up in one 3-tuple.
  pub fn collect( &mut self, data : ( Count, Money, Count ) ) {
    let ( assets, price, holders ) = data;
    self.day_count += 1;
    self.asset_history.push( assets );
    self.price_history.push( price );
    self.holders_history.push( holders );
  }

}
