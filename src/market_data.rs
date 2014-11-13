
use circularbuf::CircularBuf;
use market::{Count, Money, Market};

#[deriving(Show)]
pub struct MarketData {
  name : String,
  time_frame : Count,
  day_count : Count,
  asset_history : CircularBuf<Count>,
  price_history : CircularBuf<Money>,
  holders_history : CircularBuf<Count>
}

impl MarketData {

  pub fn new( name : String, time_frame : Count ) -> MarketData {
    let tf = time_frame as uint;
    MarketData{ name: name, time_frame: time_frame, day_count: 0
              , asset_history: CircularBuf::new( tf )
              , price_history: CircularBuf::new( tf )
              , holders_history: CircularBuf::new( tf ) }
  }

  pub fn collect( &mut self, data : ( Count, Money, Count ) ) {
    let ( assets, price, holders ) = data;
    self.day_count += 1;
    self.asset_history.push( assets );
    self.price_history.push( price );
    self.holders_history.push( holders );
  }

}
