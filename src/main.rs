extern crate libc;

mod circularbuf;
mod market;
mod market_data;
mod agent;
mod action;
mod ai;
mod interface;
mod game;
#[cfg(test)]
mod test;

fn version() -> String {
  format!( "{}.{}.{}", env!( "CARGO_PKG_VERSION_MAJOR" )
                     , env!( "CARGO_PKG_VERSION_MINOR" )
                     , env!( "CARGO_PKG_VERSION_PATCH" ) )
}

fn main() {
  println!( "Tradock v{}", version() );
  game::start_game();
}

