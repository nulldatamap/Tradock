
use market::{Count, Failure, Market};
use market_data::MarketData;
use agent::Agent;
use circularbuf::CircularBuf;
use game::Game;
use std::io::stdio::{StdReader, stdin};
use std::io::BufferedReader;
use std::ascii::AsciiExt;

pub struct ConsoleInterface {
  input : BufferedReader<StdReader>
}

impl ConsoleInterface {
  pub fn new() -> ConsoleInterface {
      ConsoleInterface {
        input: stdin()
      }
  }

  fn find_market<'a>( &self, name : &str, markets : &'a Vec<Market> )
                      -> Option<&'a Market> {
    for market in markets.iter() {
      if name.eq_ignore_ascii_case( market.name.as_slice() ) {
        return Some( market )
      }
    }
    None
  }

  pub fn user_turn( &mut self, game : &Game )
                   -> Result<bool, String> {
    println!( "Your current funds: {}", game.player.funds);
    for market in game.markets.iter() {
      print!( "{}: {} ", market.name, market.price ); 
      if market.data.day_count > 1 {
        let prices = &market.data.price_history;
        let previous_day = prices.len() - 2;
        let previous_price = prices[previous_day];
        let procentage = ( market.price - previous_price ) / previous_price;
        println!( "({:+.2}%)", procentage * 100. );
      } else {
        println!( "" );
      }
    }
    
    let mut user_input = self.input.read_line().unwrap();
    user_input.pop();
    let slices : Vec<&str> = user_input.as_slice()
                                       .split( ' ' )
                                       .filter( |s| s.len() > 0 )
                                       .collect();
    match slices.as_slice() {
      [ "buy", name, v ] => {
        if let Some( x ) = from_str::<Count>( v ) {
          println!( "You buy {} things!", x );
        } else {
          println!( "'{}' is not a number you dummy!", v );
        }
        if let Some( buy_market ) = self.find_market( name, &game.markets ) {
          println!( "Wow such market named: {}", buy_market.name );
        } else {
          println!( "No market named: {}", name );
        }
      },
      [ "sell", name, v ] => {
        if let Some( x ) = from_str::<Count>( v ) {
          println!( "You sell {} things!", x);
        } else {
          println!( "'{}' is not a number you dummy!", v);
        }
        if let Some( buy_market ) = self.find_market( name, &game.markets) {
          println!( "Wow such market named: {}", buy_market.name );
        } else {
          println!( "No market named: {}", name );
        }
      },
      [ "done" ] => {
        return Ok( true );
      }
      _ => println!( "Invalid command! {}", slices.len() )
    }

    Ok( true )
  }

}
