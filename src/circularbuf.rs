
// This is an incomplete datastructure where only the needed 
// methods will be implemented for the time being.

#[deriving(Show)]
pub struct CircularBuf<T> {
  items : Vec<T>,
  head : uint,
  capacity : uint
}

impl<T> CircularBuf<T> {

  pub fn new( capacity: uint ) -> CircularBuf<T> {
    CircularBuf{ items: Vec::with_capacity( capacity )
               , head: capacity - 1, capacity: capacity }
  }

  pub fn push( &mut self, value : T ) {
    // If we've started to wrap around
    if self.items.len() == self.capacity {
      self.head = ( self.head + 1 ) % self.capacity;
      // Put the value in the desired spot
      self.items[ self.head ] = value;
      // And move the head along and make sure it wraps around
    } else {
      // Otherwise if we're stil working with a sparse buffer
      // Just push the value onto the vector
      self.items.push( value );
    }
  }

  pub fn len( &self ) -> uint {
    self.items.len()
  }

  pub fn capacity( &self ) -> uint {
    self.capacity
  }

  pub fn front( &self ) -> &T {
    if self.items.len() == self.capacity {
      &self.items[self.head]
    } else {
      &self.items[self.items.len() - 1]
    }
  }

}

impl<T> Index<uint, T> for CircularBuf<T> {

  fn index( &self, i : &uint ) -> &T {
    if self.items.len() == self.capacity {
      let reli = ( self.head + 1 + *i ) % self.capacity;
      &self.items[reli]
    } else {
      &self.items[*i]
    }
  }

}

impl<T> IndexMut<uint, T> for CircularBuf<T> {

  fn index_mut( &mut self, i : &uint ) -> &mut T {
    if self.items.len() == self.capacity {
      let reli = ( self.head + 1 + *i ) % self.capacity;
      self.items.index_mut( &reli )
    } else {
      self.items.index_mut( i )
    }
  }

}
