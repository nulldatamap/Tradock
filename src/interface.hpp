#ifndef __INTERFACE_H__
#define __INTERFACE_H__
#include <vector>
#include <stdint.h>

typedef unsigned int uint;

template<typename T>
struct CircularBuffer {
  std::vector<T> items;
  uint head;
  uint capacity;
};

template<typename T>
uint length( CircularBuffer<T> buf ) {
  return buf.items.size();
}

template<typename T>
uint capacity( CircularBuffer<T> buf ) {
  return buf.capacity;
}

template<typename T>
const T& get( CircularBuffer<T> buf, uint i ) {
  if( length( buf ) == capacity( buf ) ) {
    uint reli = ( buf.head + 1 + i ) % capacity( buf );
    return buf.items[reli];
  } else {
    return buf.items[i];
  }
}

template<typename T>
const T& front( CircularBuffer<T> buf ) {
  if( length( buf ) == 0 ) {
    return (const T&)0;
  }
  if( length( buf ) == capacity( buf ) ) {
    return get( buf, buf.head );
  } else {
    return get( buf, length( buf ) - 1 );
  }
}

enum ActionKind {
  // If we didn't crash
  Ok,
  // Used in `handle_response` when we want to retry `get_user_action`
  Retry,
  // If an error happens
  Failed
};

struct Action {
  ActionKind kind;
  // Only used if the kind is `Ok` and we're in `get_user_action`
  std::vector<int32_t> amounts;
  // Only used if we failed, this should be the error message
  const char* failure;
};

enum Response {
  Success,
  InsufficientAgentFunds,
  InsufficientMarketAssets,
  InsufficientAgentAssets
};

struct Pair {
  // The name of the market
  const char* key;
  // How much we got invested in that market
  uint32_t value;
};

struct Agent {
  // The name of our agent
  const char* name;
  double funds;
  // His investments
  std::vector<Pair> assets; 
};

struct MarketData {
  // The name of the goods being traded in this market
  const char* name;
  // How long we're recording data for
  uint32_t time_frame;
  // How many days we're in
  uint32_t day_count;
  // The history of asset count
  CircularBuffer<uint32_t> asset_history;
  // The history of the worth of the stock
  CircularBuffer<double> price_history;
  // The histroy of the amount of investors
  CircularBuffer<uint32_t> holders_history;
};

#endif
