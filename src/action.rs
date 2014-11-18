use market::Count;

pub enum Action {
  Buy( Count ),
  Sell( Count ),
  Pass
}
