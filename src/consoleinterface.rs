use std::io::stdio::{StdReader, stdin};
use std::io::BufferedReader;
use std::ascii::AsciiExt;
use std::cmp::max;
use std::num::Float;

use market::Failure::{InsufficientAgentFunds
                    , InsufficientMarketAssets
                    , InsufficientAgentAssets};
use market::{Money, Count, Failure, Market};
use market_data::MarketData;
use agent::Agent;
use circularbuf::CircularBuf;
use game::{starting_funds, Game};

pub struct ConsoleInterface {
  input : BufferedReader<StdReader>
}

// The maximum width of the embedded graph
fn graph_max_width() -> uint {
  69
}

// The height of the embedded graph
fn graph_height() -> uint {
  12
}

fn render_market_data( player : &Agent, market : &mut Market ) {
  let price_history = &market.data.price_history;

  println!( "Statistics for: {}", market.name );
  println!( "Day: {}", market.data.day_count );

  // How much has the market grown?
  let growth = if price_history.len() == 1 {
    // We haven't grown if it's the first day
    0.
  } else {
    // Get the previous price
    let previous_price = price_history[price_history.len() - 2];
    // Calculate the price in %
    (market.price - previous_price) / previous_price * 100.
  };
  println!( "Price per. asset: {:.2} DKK ( {:+.2}% growth )", market.price
                                                            , growth );

  let total_price = market.assets as f64 * market.price;
  println!( "Total market value ( {} assets ): {:.2} DKK", market.assets
                                                         , total_price );
  // Find the closest disable price point:

  println!( "\n       Price\n         ^" );
  // Find out how many element we are skipping to fit the graph
  // into the limited terminal window.
  let skip_amount;
  let graph_width;
  // How wide does the graph need to be, and do we need to skip any data in the
  // buffer in order to see the newewst data?
  if price_history.len() > graph_max_width() {
    skip_amount = price_history.len() - graph_max_width();
    graph_width = graph_max_width();
  } else {
    skip_amount = 0;
    graph_width = price_history.len();
  };
  // Find the highest price over the given time period
  let roof : Money = price_history.iter()
                                  .skip( skip_amount )
                                  // Find the highest value
                                  .fold( 0., |mx, &vl| {
                                    if vl > mx { vl }
                                    else { mx }
                                  } );
  // Scale the step value to the highest price
  let graph_step = roof / graph_height() as f64;
  // Go through each line of the graph
  for y in range( 0u, graph_height() ).rev() {
    let price_point = y as f64 * graph_step;
    // Our string buffer that holds a horizontal slice of the graph
    let mut graph_slice = String::new();
    for x in range( 0u, graph_width ) {
      let x_price = price_history[skip_amount + x];
      // Relative price to the current price point
      let rel_price = (x_price - price_point + 1.) / graph_step;
      // Build each cell if the right character that corresponds the it's value
      graph_slice.push( if rel_price <= 0. {
        ' '
      } else if rel_price <= 0.25 {
        '_'
      } else if rel_price <= 0.50 {
        '.'
      } else if rel_price <= 0.75 {
        '='
      } else {
        '#'
      } );
    }
    // Print the axis and graph slice
    println!( "{:9.2}|{}", price_point, graph_slice );
  }
  println!( "Time:    +{}>\n", String::from_char( graph_max_width(), '-' ) );
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
    println!( "\nYour current funds: {:.2} DKK", game.player.funds);
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
         if let Some( buy_market ) = self.find_market( name, &mut game.markets ) {
           if let Some( x ) = from_str::<Count>( amount ) {
              match buy_market.buy_assets( &mut game.player, x ) {
                Err( InsufficientAgentFunds) => {
                  println!( "Not enough money" );
                },
                Err( InsufficientMarketAssets) => {
                  println!( "Not enough market assets" );
                },
                _ => {
                  println!( "You buy {} {} for {:.2} DKK", x, name, x as f64 * buy_market.price);
                  println!( "Your current funds: {:.2} DKK", game.player.funds );
                }
               } 
           } else {
            println!( "'{}' is not a number you dummy!", amount );
          }
        } else {
          println!( "No market named: {}", name );
        }
        },
        [ "sell", amount, name ] => {
          if let Some( sell_market ) = self.find_market( name, &mut game.markets) {
            if let Some( x ) = from_str::<Count>( amount ) {
               match sell_market.sell_assets( &mut game.player, x ) {
                Err( InsufficientAgentAssets ) => {
                  println!( "You don't have {} assets of {} to sell", x, name);
                },
                _ => {
                  println!( "You sell {} assets of {} for {:.2} DKK", x, name, x as f64 * sell_market.price);
                  println!( "Your current funds: {:.2} DKK", game.player.funds );
                }
               }
            } else {
             println!( "'{}' is not a number you dummy!", amount);
          }
        } else {
          println!( "No market named: {}", name );
          }
        },
        [ "assets" ] => {
          for market in game.markets.iter() {
            println!( "{}: {}", market.name, game.player.get_assets( &market.name ) );
          }
        },
        [ "show", name ] => {
          if let Some( market ) = self.find_market( name, &mut game.markets) {
            render_market_data( &game.player, market );
          }
        }
        [ "quit" ] => {
          return Ok( false );
        },
        [ "help" ] => {
          println!( "'buy' <amount> <market>");
          println!( "'sell' <amount> <market>");
          println!( "'show' <market name>");
          println!( "Type 'assets' to see what kind of assets you have");
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
