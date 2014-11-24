#include "Dropbox/git/Tradock/src/interface.h"
#include <stdio.h>

// Empty placeholder
class Interface {
};

extern Interface* create_interface() {
  return new Interface();
}

extern void destroy_interface( Interface* iface ) {
  delete iface;
}


extern Action render_market_data( Interface* self
                                , vector<MarketData> data
                                , Agent agent ) {
  // If the rendering goes well
  if( 1 == 1 ) {
    // Return Ok
    return { Ok };
  } else {
    return { Failed, {}, "What went wrong!" };
  } 
}

extern Action get_user_action( Interface* self
                             , vector<MarketData> data
                             , Agent agent ) {
  // If everything goes well, return a vector of
  // buys/sells/passes in the same order as the
  // market-data was supplied in.
  // A buy is a positive value ( the amount you buy )
  // A sell is a negative value ( the amount you sell )
  // Or 0, since you buy or sell nothing 
  if( true ) {
    return { Ok, { 10, -3, 0 } };
  } else {
    return { Failed, {}, "Something went wrong :(" };
  }
}

extern Action handle_response( Interface* self, Response resp ) {
  if( resp == Success ) {
    return { Ok };
  } else {
    // Let the user try that again
    return { Retry };
  }
  // If we fail while trying to tell the user they didn't something
  // they couldn't:
  return { Failed, {}, "First you break the rules, and now the code!" };
}


