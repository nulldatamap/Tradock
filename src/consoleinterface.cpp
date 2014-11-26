// This is the source code for the console interface for Tradock


// Preprocessor directives
#include "interface.hpp"
#include <iostream>
#include <string>
#include <vector>
#include <sstream>

using namespace std;

// Empty placeholder
class Interface {
};

extern "C"
Interface* create_interface() {
  return new Interface();
}

extern "C"
void destroy_interface( Interface* iface ) {
  delete iface;
}

extern "C"
Action render_market_data( Interface* self
                         , std::vector<MarketData> data
                         , Agent _agent ) {
  Agent agent = *(Agent*) (&data + 1); // U WOT M8?! Just leave it be

  cout << agent.name << "\n";
  return { Ok };
}


/*
and fix strings ( Storing CString for longer )
*/


extern "C"
Action get_user_action( Interface* self
                      , std::vector<MarketData> data
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

extern "C"
Action handle_response( Interface* self, std::vector<Pair<Response>> resp ) {
  if( 1 == 1 ) {
    return { Ok };
  } else {
    // Let the user try that again
    return { Retry };
  }
  // If we fail while trying to tell the user they didn't something
  // they couldn't:
  return { Failed, {}, "First you break the rules, and now the code!" };
}




