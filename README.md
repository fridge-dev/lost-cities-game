# Welcome

This is a personal project not intended to make money. This is based on the game Lost Cities. https://boardgamegeek.com/boardgame/50/lost-cities

## Board Game Design

Just putting this here, for now.

```
+------------+------------+------------+------------+------------+
|    -14     |    -45     |     1      |     20     |            |
+------------+------------+------------+------------+------------+
|            |            |            |            |            |
|            |      6     |            |     10     |            |
|            |      6     |            |     8      |            |
|            |      6     |            |     5      |            |
|            |      6     |            |     2      |            |
|            |      4     |            |     x4     |            |
+------------+------------+------------+------------+------------+
|    Red     |   Green    |   White    |    Blue    |   Yellow   |
|            |            |            |            |            | Deck: [?, 42]
|  [__, XY]  |  [__, XY]  |  [__, XY]  |  [__, XY]  |  [__, XY]  |
+------------+------------+------------+------------+------------+
|     x2     |     x3     |     4      |            |            |
|     3      |     5      |     7      |            |            |
|     4      |            |     10     |            |            |
|     6      |            |            |            |            |
|            |            |            |            |            |
|            |            |            |            |            |
+------------+------------+------------+------------+------------+
|    -14     |    -45     |     1      |            |            |
+------------+------------+------------+------------+------------+

+-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+
| Red | | Grn | | Wht | | Blu | | Ylw | | Red | | Red | | Red |
|  8  | |  8  | |  8  | |  8  | |  10 | |  x  | |  x  | |  x  |
+-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+ +-----+
```

## Worklog

### Planned

**Backend**
1. Rules: scoring
1. Rules: handle end game
1. Implement backend with file storage
1. Figure out nio task model for backend
1. See if it's easily possible to separate storage types from API types
1. See if it's easily possible to separate CLI/FE logic from backend logic via separate repos?

**Frontend**
1. Implement main.rs state machine (turns, end game)
1. Implement main.rs board drawer
1. Add duplicate rule checks on the FE

**General**
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
