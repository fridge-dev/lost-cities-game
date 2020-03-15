# Crates

* **bin-client** - The binary for client (Frontend) application
* **bin-server** - The binary for server (Backend) application
* **types** - The structs used by the client-side to model the game in memory.
* **api** - The internal API definition and implementations for how Frontend and Backend support the game.
* **wire_types** - The structs used by client/server to communicate over the wire.
* **rules** - The rules engine for enforcing various rules (e.g. what plays are allowed, how to calculate the score of a board)
* **storage** - The APIs and implementation of storage engine

# Layers of abstraction

The major difference between data modeled by Frontend and Backend is Frontend will only have data for a specific player, whereas Backend will have all of the information. An example of data not present in Frontend is the player's opponent's cards in hand.

The different layers of abstraction require different types for modeling the data. Here is the current and planned future types.

## Current

```
+----------------------------------------+   +---------------------------------------------------+
|  Frontend                              |   |  Backend                                          |
|                                        |   |                                                   |
| UI <-> GameApi types <-> Wire types <---------> Wire types <-> GameApi types <-> Storage types |
|                                        |   |                                                   |
+----------------------------------------+   +---------------------------------------------------+
```

Pros:
* The rules engine can be shared between Frontend and Backend.

Cons:
* Can't implement std lib Traits (From, Into) on GameApi types when they're defined outside of crate
    * Could be solved by generating the protobuf types in each bin crate.
* The kind of errors are different in Frontend/Backend
    * Could be solved with generics.

## Ideal end state

Layers:
1. Main event loop
    * Types: cli (strings)
    * Used by: -
    * Uses: (2) Client app API
1. Client app API
    * Types: api (client app types)
        * Implements Display
    * Used by: (1)
    * Uses: (3) tonic client
1. Tonic client
    * Types: wire_types (protobuf)
    * Used by: (2)
    * Uses: -
1. Tonic server
    * Types: wire_types (protobuf)
    * Used by: (5)
    * Uses: -
1. Server app
    * Types: api (server app types)
    * Used by: -
    * Uses: (6) storage
1. Storage
    * Types: storage
    * Used by: (5)
    * Uses: - (memory?, disk?, DDB?)

Rules: applies to (2) types and (5) types

```
+----------------------------------------+   +----------------------------------+
|  Frontend                              |   |  Backend                         |
|                                        |   |                                  |
| UI <-> Frontend types <-> Wire types <---------> Wire types <-> Backend types |
|                                        |   |                                  |
+----------------------------------------+   +----------------------------------+
```

* `Frontend types` = GameApi
* `Backend types` = Storage types
