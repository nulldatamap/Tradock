use market::Count;

#[deriving(Show)]
pub enum Action {
  Buy( Count ),
  Sell( Count ),
  Pass
}
