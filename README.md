# Welcome

This is a personal project not intended to make money. This is based on the game Lost Cities. https://boardgamegeek.com/boardgame/50/lost-cities

## Board Game Design

Just putting this here, for now.

```
The board:
+-----------+-----------+-----------+-----------+-----------+
| Opponent's score: 12                                      |
+-----------+-----------+-----------+-----------+-----------+
|           |           |           |           |           |
|  +-----+  |           |           |           |           |
|  |  5  |  |           |           |           |           |
|  +-----+  |           |           |           |           |
|  |  4  |  |           |           |           |           |
|  +-----+  |           |           |           |           |
|  |  3  |  |           |           |           |           |
|  +-----+  |           |           |           |  +-----+  |
|  | wgr |  |           |           |           |  | 10  |  |
|  +-----+  |           |           |           |  +-----+  |
|  | wgr |  |           |           |           |  |  9  |  |
|  | Red |  |           |           |           |  | Ylw |  |
|  +-----+  |           |           |           |  +-----+  |
+-----------+-----------+-----------+-----------+-----------+
|    Red    |   Green   |   White   |   Blue    |  Yellow   |
|           |           |           |           |           |
|  +-----+  |  +-----+  |  +-----+  |  +-----+  |  +-----+  |
|  |  8  |  |  |     |  |  |     |  |  |     |  |  |     |  |
|  | Red |  |  |     |  |  |     |  |  |     |  |  |     |  |
|  +-----+  |  +-----+  |  +-----+  |  +-----+  |  +-----+  |
| 3 in pile |           |           |           |           |
|           |           |           |           |           |
|    Red    |   Green   |   White   |   Blue    |  Yellow   |
+-----------+-----------+-----------+-----------+-----------+
|  +-----+  |  +-----+  |           |           |           |
|  |  8  |  |  | wgr |  |           |           |           |
|  | Red |  |  +-----+  |           |           |           |
|  +-----+  |  |  3  |  |           |           |           |
|           |  +-----+  |           |           |           |
|           |  |  4  |  |           |           |           |
|           |  +-----+  |           |           |           |
|           |  |  5  |  |           |           |           |
|           |  | Red |  |           |           |           |
|           |  +-----+  |           |           |           |
|           |           |           |           |           |
+-----------+-----------+-----------+-----------+-----------+
| Your score: -30                                           |
+-----------+-----------+-----------+-----------+-----------+
Main draw pile: 42 cards remaining

Your hand:
+-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+
|  8  | |  8  | |  8  | |  8  | |  10 | | wgr | | wgr | | wgr |
| Red | | Grn | | Wht | | Blu | | Ylw | | Red | | Red | | Red |
+-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+
```

## Worklog

### Planned

**Backend**
1. Rules: handle end game
1. Implement backend with file storage
1. Figure out nio task model for backend
1. See if it's easily possible to separate storage types from API types
1. See if it's easily possible to separate CLI/FE logic from backend logic via separate repos?

**Frontend**
1. Implement main.rs state machine (turns, end game)
1. Improve turn selection UI
  * Sort cards in hand
  * Add review section
  * Add better selection mechanism
  * Add turn indicator
1. Add duplicate rule checks on the FE
1. Improve board drawer

**General**
1. Split game into separate backend and frontend processes.
1. Figure out best way to model error propagation to top level.
1. Figure out how to make sub-crates tests run during top-level cargo build
1. Add automated test of a game
1. Test coverage?

### Complete (to some extent)

1. Implement basics to start a game
1. Implement main.rs CLI i/o
1. Implement "dumb" storage
1. Integrate API into storage
1. GetState for viewing player
1. Fully implement GetGameState API
1. Add rules engine
1. Rules: updating based on a turn
1. Implement main.rs user turn selection I/O
1. Rules: scoring
1. Implement main.rs board drawer
