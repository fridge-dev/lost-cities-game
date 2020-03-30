```
cargo build --release --bin lost-cities-game-client
cp ./target/release/lost-cities-game-client ~/tmp/lost-cities-game-client
rm ~/tmp/game.zip
zip -er ~/tmp/game.zip ~/tmp/lost-cities-game-client
```