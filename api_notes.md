```c++
// API notes
/* High level functionality
   * Convey information to the user
     - Market data / history
     - Other things?
   * Get actions from the user
     - Buy, sell and pass
     - Other actions?
   * Communicate validity of actions
     - Can't do that ( with a reason )
     - Succeeded
*/

// &Interface - a pointer to the state of your interface
// Action - an action by the user ( buy, sell, pass, etc. )
// Response - a reponse from the game about how it handled the user's action
// vector<&const MarketData> - A list of the collected data from the different markets
// &const Agent - information about the user's agent ( the one they're playing as )

/// Defined by the interface, called by the game

// Called by the game in order to render the information the user needs each turn
// Returns: true if the function succeeded, false if it failed
bool render_market_data( &Interface );

// Called by the game in order to get input from the user
// Returns: an action for the user to do
Action get_user_action( &Interface );

// Called by the game to make the interface to handle the game's response
// Returns: true if it accepts it, false if it wants to redo the user action
bool handle_response( &Interface, Response );

/// Defined by the game, called by the interface

// Creates the interface
&Interface create_interface();

// Destroys the interface
void destroy_interface();

// Get's all the data from all the markets ( should be called each turn )
vector<&const MarketData> get_market_data();

// Get's the players agent data ( Should be called each turn )
&const Agent get_agent();

```