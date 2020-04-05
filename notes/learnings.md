These are my learnings which I will apply on the next ascii-board game I make.

== General ==

* UI and backend should have different GameApi APIs and type definitions
  * UI sometimes has subset of data or formatted differently

== UI ==

* State is difficult, can be more easily managed if everything is strictly modeled as a screen
* "Smart" CLI should be inside each screen mod
* Everything should run from a screen, and screens should redirect to other screens
* Having my own draw trait might be easier than `impl Display`

== Backend ==

* "Dumb" storage layer was good

== Layers of Abstraction ==

```
+---------------+   +---------------+   +---------------+   +---------------+   +---------------+
|               |   |               |   |               |   |               |   |               |
|   Frontend    |-->|     Game      |-->|    Proto      |-->|    Backend    |-->|    Backend    |
|  Application  |<--|    Client     |<--|    Server     |<--|  Application  |<--|    Storage    |
|               |   |               |   |               |   |               |   |               |
+---------------+   +---------------+   +---------------+   +---------------+   +---------------+

```

* **App** - game-independent "platform" logic
* **Game** - game-specific logic
* **Frontend** - UI specific logic
* **Client/Server** - Network interfaces
* **Backend** - Business logic
* **Storage** - Dumb persistence layer
* **Protbuf** - Object definition

```
+---------------+   +---------------+   +---------------+   +---------------+   +---------------+
|               |   |               |   |               |   |               |   |               |
|   Frontend    |-->|   Proto App   |   |   Proto App   |-->|    Backend    |-->|    Backend    |
|   App Engine  |   |    Client     |   |    Server     |   |  Application  |   |    Storage    |
|               |   |               |   |               |   |               |   |               |
+---------------+   +---------------+   +---------------+   +---------------+   +---------------+
        |                       \              /                |
        |                        \            /                 |
        |                         V          V                  |
        |                         +----------+                  |
        |                         | Protobuf |                  |
        |                         |   def    |                  |
        |                         +----------+        +---------+
        V                        ^ ^       ^ ^        |
  +----------|                  / /         \ \       |
  | +----------|               / /           \ \      V
  | | +----------+    +----------|            +----------|
  +-| | Frontend |--->| +----------|          | +----------|
    +-|   Game   |--->| | +----------+        | | +----------+
      +----------+    +-| | ProtoApp |        +-| | Backend  |
                        +-|  Client  |          +-|   Game   |
                          +----------+            +----------+
```

Proto data model should be more closely coupled to the client's representation.

1. Frontend Application
    * Holds state of which screen user is on
    * Displays data
    * Takes input from user
2. Game Client
    * Defines **FrontendGameApi**
    * Depends on `generated` proto client
3. Proto Server
    * Adapts BackendGameApi to proto model
    * Also TODO need backend "engine" in here for routing layer
4. Backend Application
    * Defines **BackendGameApi**
    * Defines rules
    * Applies rules
        * There are some rules which *must* be applied on the backend.
        * There are some rules which *can* be applied on the backend or frontend. For these, we prefer the backend to simplify code duplication, since in most cases, implementing it on only the frontend isn't possible.
    * Has superset of data model than client
