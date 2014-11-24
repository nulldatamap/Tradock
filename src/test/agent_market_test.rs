
use market::{Count, Money, Market};
use market::Failure::{InsufficientAgentFunds
                     , InsufficientMarketAssets
                     , InsufficientAgentAssets};
use agent::{Agent};

fn setup( m_assets : Count, m_price : Money ) -> (Market, Agent) {
  ( Market::new( "Test Market".to_string(), m_price, m_assets )
  , Agent::new( "Test Agent".to_string(), 100. ) )
}

#[test]
// Make sure that buying works as intended.
fn enough_funds_enough_assets_buy() {
  let (mut ma, mut ag) = setup( 10, 10. );
  // A normal buy ( with an initial insert in the agents assets )
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 9 );
  assert_eq!( ma.buys, 1 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 90. );
  assert_eq!( ag.assets[ma.name], 1 );

  // Another normal buy ( this time with a modify in the agents assets ) 
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 8 );
  assert_eq!( ma.buys, 2 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 80. );
  assert_eq!( ag.assets[ma.name], 2 );
}

#[test]
// Make sure that the user can't buy assets they can't afford.
fn enough_funds_not_enough_assets_buy() {
  let (mut ma, mut ag) = setup( 1, 10. );
  // Make sure that the user can buy even if they're broke afterwards
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 0 );
  assert_eq!( ma.buys, 1 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 90. );
  assert_eq!( ag.assets[ma.name], 1 );

  // And make sure the user get an error when they try 
  // to buy something now that they're broke.
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Err( InsufficientMarketAssets ) );
  assert_eq!( ma.assets, 0 );
  assert_eq!( ma.buys, 1 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 90. );
  assert_eq!( ag.assets[ma.name], 1 );
}

#[test]
// Make sure the user can't buy more assets than available
fn not_enough_funds_enough_assets_buy() {
  let (mut ma, mut ag) = setup( 20, 10. );
  // Make sure the user can buy all remaining assets 
  assert_eq!( ma.buy_assets( &mut ag, 10 ), Ok( () ) );
  assert_eq!( ma.assets, 10 );
  assert_eq!( ma.buys, 10 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 0. );
  assert_eq!( ag.assets[ma.name], 10 );

  // And make sure the user isn't able to buy more than that.
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Err( InsufficientAgentFunds ) );
  assert_eq!( ma.assets, 10 );
  assert_eq!( ma.buys, 10 );
  assert_eq!( ma.sells, 0 );

  assert_eq!( ag.funds, 0. );
  assert_eq!( ag.assets[ma.name], 10 );
}

#[test]
// Make sure selling works as intended
fn enough_assets_sell() {
  let (mut ma, mut ag) = setup( 10, 10. );
  assert_eq!( ma.buy_assets( &mut ag, 2 ), Ok( () ) );
  // Make sure the user can sell, when they have the assets
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 9 );
  assert_eq!( ma.buys, 2 );
  assert_eq!( ma.sells, 1 );

  assert_eq!( ag.funds, 90. );
  assert_eq!( ag.assets[ma.name], 1 );

  // Make sure that the user can sell their last assets
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 10 );
  assert_eq!( ma.buys, 2 );
  assert_eq!( ma.sells, 2 );

  assert_eq!( ag.funds, 100. );
  assert_eq!( ag.assets[ma.name], 0 );
}

#[test]
// Make sure the user can't sell more assets than they have
fn not_enough_assets_sell() {
  let (mut ma, mut ag) = setup( 1, 10. );
  assert_eq!( ma.buy_assets( &mut ag, 1 ), Ok( () ) );

  // Make sure the user can sell the rest of their assets
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.assets, 1 );
  assert_eq!( ma.buys, 1 );
  assert_eq!( ma.sells, 1 );

  assert_eq!( ag.funds, 100. );
  assert_eq!( ag.assets[ma.name], 0 );

  // And make sure they can't sell assets they don't have
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Err( InsufficientAgentAssets ) );
  assert_eq!( ma.assets, 1 );
  assert_eq!( ma.buys, 1 );
  assert_eq!( ma.sells, 1 );

  assert_eq!( ag.funds, 100. );
  assert_eq!( ag.assets[ma.name], 0 );
}

#[test]
// Make sure the recalculation of the price behaves as intended
fn test_recalculation() {
  let (mut ma, mut ag) = setup( 10, 10. );
  assert_eq!( ma.buy_assets( &mut ag, 3 ), Ok( () ) );
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Ok( () ) );
  assert_eq!( ma.sell_assets( &mut ag, 1 ), Ok( () ) );
  
  // Make sure that the market has grown by a 100%
  ma.recalculate_price();

  assert_eq!( ma.buys, 0 );
  assert_eq!( ma.sells, 0 );
  assert_eq!( ma.price, 20. );
  assert_eq!( ma.holders, 2 );
}
