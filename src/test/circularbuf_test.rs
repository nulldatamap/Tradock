use circularbuf::CircularBuf;

#[test]
fn valid_usage() {
  let mut cbuf = CircularBuf::new( 10 );
  // Test front on an empty buffer
  assert_eq!( cbuf.front(), None );

  // Make sure we can push on a sparse buffer
  for i in range( 0u, 5 ) {
    cbuf.push( i );
  }
  assert_eq!( cbuf.len(), 5 );

  let mut counter = 0;
  // Make sure iteration works
  for elm in cbuf.iter() {
    assert_eq!( *elm, counter );
    counter += 1;
  }
  
  // Check that indexing on a sparse buffer works
  for i in range( 0u, 5 ) {
    assert_eq!( cbuf[i], i );
  }
  // Check that front refers to the right element
  assert_eq!( cbuf.front().map( |&v| v ), Some( 4 ) );
  // Check that we can assign indexes properly
  cbuf[0] = 1337;
  assert_eq!( cbuf[0], 1337 );
  // Make sure we can fill it 
  for i in range( 5u, 10 ) {
    cbuf.push( i );
  }
  assert_eq!( cbuf.len(), cbuf.capacity() );
  assert_eq!( cbuf.front().map( |&v| v ), Some( 9 ) );
  // Check if pushing on a filled buffer works
  cbuf.push( 420 ); // blæze it
  assert_eq!( cbuf.len(), cbuf.capacity() );
  assert_eq!( cbuf.front().map( |&v| v ), Some( 420 ) );
  // Make sure that the circular indexing works
  assert_eq!( cbuf[0], 1 );
  cbuf.push( 42 );
  assert_eq!( cbuf[0], 2 );
}
