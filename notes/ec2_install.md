## Connect

```sh
HOSTNAME=ec2-54-189-144-219.us-west-2.compute.amazonaws.com
ssh -o IdentitiesOnly=yes -i ~/.aws/private-keys/firstattemptatgameserver20200327.pem ec2-user@$HOSTNAME
```

## Install

```sh
echo "export PATH=\"\$PATH:$HOME/lost-cities-game/bin\"" >> ~/.bash_profile && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source ~/.bash_profile && \
sudo yum install git && \
sudo yum install gcc gcc-c++ make && \
git clone https://github.com/fridge-dev/lost-cities-game
cd ~/lost-cities-game
cargo build --release --bin lost-cities-game-server
```

## Run

```sh
cd ~/lost-cities-game
async.sh ./target/release/lost-cities-game-server
```
