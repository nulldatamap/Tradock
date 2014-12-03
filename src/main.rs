#![feature(if_let, macro_rules)]

extern crate libc;

mod randorditer;
mod circularbuf;
mod market;
mod market_data;
mod agent;
mod ai;
mod consoleinterface;
mod game;
#[cfg(test)]
mod test;

fn version() -> String {
  format!( "{}.{}.{}", env!( "CARGO_PKG_VERSION_MAJOR" )
                     , env!( "CARGO_PKG_VERSION_MINOR" )
                     , env!( "CARGO_PKG_VERSION_PATCH" ) )
}

fn print_welcome_message() {
  println!( "========= Welcome to Tradock version {}! =========", version() );
  println!( "- By Marco 'Nulldata' Persson & Andreas W. Gustavsen.\n" );
  println!( "Type 'help' to see the list of commands available." );
  println!( "--------------------------------------------------" );
}

fn main() {
  print_welcome_message();
  game::start_game();
}

