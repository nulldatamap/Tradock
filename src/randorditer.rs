use std::rand::{task_rng, random, Rng};
use std::mem::{transmute, size_of};

// This is safe since the iterator still is bound to the lifetime of it's vector.
pub struct RandomOrderIterator<'a, T: 'a> {
  ptr : *mut T,
  width : uint,
  indencies : Vec<uint>
}

impl<'a, T> RandomOrderIterator<'a, T> {
  pub fn new( items : &'a mut Vec<T> ) -> RandomOrderIterator<'a, T> {
    // Get a (pseudo) random number generator
    let mut rng = task_rng();
    // Generate a list of all the indencies for `items`
    let mut indencies = Vec::from_fn( items.len(), |i| i );
    // Shuffle them
    rng.shuffle( indencies.as_mut_slice() );
    unsafe {
                            // Turn the reference into a pointer
      RandomOrderIterator { ptr: items.as_mut_ptr()
                          , width: items.len()
                          , indencies: indencies }
    }
  } 
}

impl<'a, T> Iterator<&'a mut T> for RandomOrderIterator<'a, T> {
  fn next( &mut self ) -> Option<&'a mut T> {
    // Normally you want special case code for zero sized types
    // but I chose to just now allow them for now.
    if size_of::<T>() == 0 {
      panic!( "Why would you iterate over zero-sized data..?" );
    }
    unsafe {
      // Get the next randomized index if any and return a pointer to that element
      self.indencies.pop()
                    .and_then( |idx| {
                      // Get the idx'th's location, and turn it into a reference.
                      transmute( self.ptr.offset( idx as int ) )
                    } )
    }
  }

  fn size_hint( &self ) -> (uint, Option<uint>) {
    (self.width, Some(self.width))
  }
}

// More readable than RandomOrderIterator::new( .. )
pub fn random_order<T>( items : &mut Vec<T> ) -> RandomOrderIterator<T> {
  RandomOrderIterator::new( items )
}
