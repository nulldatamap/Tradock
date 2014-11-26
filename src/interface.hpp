#ifndef __INTERFACE_H__
#define __INTERFACE_H__
#include <vector>
#include <stdint.h>

template<typename T>
class CircularBuf {
public:
  T* items;
private:
  uint64_t cursor;
  uint64_t _allocated;
  uint64_t head;
  uint64_t allocated;
public:
  uint64_t length() {
    return this->cursor;
  }

  uint64_t capacity() {
    return this->allocated;
  }
  
  bool empty() {
    return this->cursor == 0;
  }

  T get( uint64_t i ) {
    if( this->length() == this->capacity() ) {
      uint64_t reli = ( this->head + 1 + i ) % this->capacity();
      return this->items[reli];
    } else {
      return this->items[i];
    }
  }
  
  T front() {
    if( this->length() == this->capacity() ) {
      return this->get( this->head );
    } else {
      return this->get( this->length() - 1 );
    }
  }
};



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

template<typename T>
struct Pair {
  // The name of the market
  const char* key;
  // How much we got invested in that market
  T value;
};

struct Agent {
  // The name of our agent
  const char* name;
  double funds;
  // His investments
  std::vector<Pair<uint32_t>> assets;
};

struct MarketData {
  // The name of the goods being traded in this market
  const char* name;
  // How long we're recording data for
  uint32_t time_frame;
  // How many days we're in
  uint32_t day_count;
  // The history of asset count
  CircularBuf<uint32_t> asset_history;
  // The history of the worth of the stock
  CircularBuf<double> price_history;
  // The histroy of the amount of investors
  CircularBuf<uint32_t> holders_history;
};

#endif
