## Connect

```sh
HOSTNAME=ec2-54-189-144-219.us-west-2.compute.amazonaws.com
ssh -o IdentitiesOnly=yes -i ~/.aws/private-keys/firstattemptatgameserver20200327.pem ec2-user@$HOSTNAME
```

## Install and run

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
source ~/.bash_profile && \
sudo yum install git && \
sudo yum install gcc gcc-c++ make && \
git clone https://github.com/fridge-dev/lost-cities-game
cd ~/lost-cities-game
cargo run --release --bin lost-cities-game-server
```

## Update

```sh
source ~/.bash_profile
cd ~/lost-cities-game
git pull --rebase
cargo run --release --bin lost-cities-game-server
```
