use std::collections::HashMap;
use market::{Money, Count};

#[deriving(Show)]
pub struct Agent {
  pub name   : String,
  pub funds  : Money,
  pub assets : HashMap<String, Count>
}

impl Agent {

  pub fn new( name : String, starting_funds : Money ) -> Agent {

    Agent{ name: name, funds: starting_funds, assets: HashMap::new() }
  }

  pub fn add_assets( &mut self, market : &String, amount : Count ) {

    match self.assets.find_mut( market ) {
      Some( asset_count ) => {
        *asset_count += amount;
        // return early so the the None case can work out without
        // breaking the borrowing rules.
        return
      },
      // Here we do nothing, and go out of the lifetime of our
      // mutable borrow: `self.assets.find_mut( market )`
      None => {}
    }
    // Now that `self.assets` isn't mutuably borrowed we can
    // do an `.insert` on it without making the borrow checker sad.
    self.assets.insert( market.clone(), amount );
  }

  pub fn remove_assets( &mut self, market : &String, amount : Count ) -> bool {

    match self.assets.find_mut( market ) {
      Some( asset_count ) => {

        if *asset_count < amount {
          return false
        } else {
          *asset_count -= amount
        }

      },
      None => return false
    }

    true
  }
}
