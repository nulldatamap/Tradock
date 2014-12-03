
// This is an incomplete datastructure where only the needed 
// methods will be implemented for the time being.

#[deriving(Show, Clone)]
#[repr(C)]
pub struct CircularBuf<T> {
  items : Vec<T>,
  head : uint,
  capacity : uint
}

pub struct CirItems<'a, T: 'a> {
  buf : &'a CircularBuf<T>,
  idx : uint
}

impl<'a, T: 'a> Iterator<&'a T> for CirItems<'a, T> {
  fn next( &mut self ) -> Option<&'a T> {
    // We've reached the end
    if self.idx == self.buf.len() {
      None
    } else {
      // Get the next element
      let r = &self.buf[self.idx];
      self.idx += 1;
      Some( r )
    }
  }

  fn size_hint( &self ) -> (uint, Option<uint>) {
    (self.buf.len(), Some( self.buf.len() ))
  }

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

  pub fn front( &self ) -> Option<&T> {
    if self.items.len() == 0 {
      return None
    }
    Some( if self.items.len() == self.capacity {
      &self.items[self.head]
    } else {
      &self.items[self.items.len() - 1]
    } )
  }

  pub fn iter<'a>( &'a self ) -> CirItems<'a, T> {
    CirItems{ buf: self, idx: 0 }
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
