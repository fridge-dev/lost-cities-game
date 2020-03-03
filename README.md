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
1. Implement "dumb" storage
1. Integrate API into storage
1. Add automated test of a game
1. Add rules engine

**Frontend**
1. Implement main.rs state machine
1. Implement main.rs user turn selection
1. Implement main.rs board drawer

### Complete (to some extent)

1. Implement basics to start a game
1. Implement main.rs CLI i/o
