use std::io::stdio::{StdReader, stdin};
use std::io::BufferedReader;
use std::ascii::AsciiExt;

use market::{Count, Failure, Market};
use market_data::MarketData;
use agent::Agent;
use circularbuf::CircularBuf;
use game::{starting_funds, Game};

pub struct ConsoleInterface {
  input : BufferedReader<StdReader>
}

fn render_market_data( player : &Agent, market : &Market ) {
  println!( "Stats for: {}", market.name );
  println!( "Day: {}", market.data.day_count );
  println!( "Price per. asset: {:.2} DKK", market.price );
  let total_price = market.assets as f64 * market.price;
  println!( "Total market value ( {} assets ): {:.2} DKK", market.assets
                                                         , total_price );
  // Find the closest disable price point:
  
  
  // <GRAPH HERE>
  let assets = player.get_assets( &market.name );
  let total_gain = assets as f64 * market.price;
  println!( "You own: {} assets of {} ( worth {:.2} DKK )", assets
                                                          , market.name
                                                          , total_gain );
}

impl ConsoleInterface {
  pub fn new() -> ConsoleInterface {
      ConsoleInterface {
        input: stdin()
      }
  }

  pub fn print_overview( &self, game : &Game ) {
    print!( "Available markets:" );
  }

  // This function checks if the user typed in a valid market
  // to buy or sell from
  fn find_market<'a>( &self, name : &str, markets : &'a mut Vec<Market> )
                      -> Option<&'a mut Market> {
    for market in markets.iter_mut() {
      if name.eq_ignore_ascii_case( market.name.as_slice() ) {
        return Some( market )
      }
    }
    None
  }

  // This function handles all the functionality
  // of the interface
  pub fn user_turn( &mut self, game : &mut Game )
                   -> Result<bool, String> {
    println!( "\n" )
    for market in game.markets.iter() {
      print!( "{}: {:.2} DKK per. asset ", market.name
                                         , market.price );
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
    println!( "\nYour current funds: {} DKK", game.player.funds);
    loop {
      let mut user_input = self.input.read_line().unwrap();
      user_input.pop();
      let slices : Vec<&str> = user_input.as_slice()
                                         .trim()
                                         .split( ' ' )
                                         .filter( |s| s.len() > 0 )
                                         .collect();
      match slices.as_slice() {
        [ "buy", amount, name ] => {
          if let Some( x ) = from_str::<Count>( amount ) {
            println!( "You buy {} things!", x );  
          } else {
            println!( "'{}' is not a number you dummy!", amount );
          }
          if let Some( buy_market ) = self.find_market( name, &mut game.markets ) {
            buy_market.buy_assets( &mut game.player, 10);
            println!( "Wow such market named: {}", buy_market.name );
            println!( "Your current funds: {} DKK", game.player.funds);
          } else {
            println!( "No market named: {}", name );
          }
        },
        [ "sell", amount, name ] => {
          if let Some( x ) = from_str::<Count>( amount ) {
            println!( "You sell {} things!", x);
          } else {
            println!( "'{}' is not a number you dummy!", amount);
          }
          if let Some( sell_market ) = self.find_market( name, &mut game.markets) {
            sell_market.sell_assets( &mut game.player, 10);
            println!( "Wow such market named: {}", sell_market.name );
            println!( "Your current funds: {} DKK", game.player.funds);
          } else {
            println!( "No market named: {}", name );
          }
        },
        [ "inventory" ] => {
          // do this after we finished buy/sell 
        }
        [ "quit" ] => {
          println!( "Exiting game...");
          return Ok( false );
        },
        [ "help" ] => {
          println!( "Type 'buy' to buy something from the market, remember to do it in this order: buy, amount, market name");
          println!( "Type 'sell' to sell something from the market, remember to do it in this order: sell, amount, market name");
          println!( "Type 'inventory' to see what kind of assets you have");
          println!( "Type 'done' to continue to the next day");
          println!( "Type 'quit' to exit the game");
        },
        [ "done" ] => {
          return Ok( true );
          break
        },
        [] => {},
        _ => { 
          println!( "Invalid command!" )
        }
      }
    }

    Ok( true )
  }

  pub fn print_outcome( &self, game : &Game ) {
    let funds = game.player.funds;
    println!( "\n\nYou've retired from trading." );
    if funds < starting_funds() {
      println!( "You've not been lucky on the ruthless trading market and has lost:" );
      println!( "{:7.2} DKK", starting_funds() - funds );
      println!( "Maybe better luck next time?" );
    } else if funds > starting_funds() {
      println!( "You've succeed on the stock market, earning:" );
      println!( "{:7.2} DKK", funds - starting_funds() );
    } else {
      println!( "You've accomplished absolutely nothing.." );
    }

    println!( "\n\nGoodbye, and thanks for playing!" );
  }

}
