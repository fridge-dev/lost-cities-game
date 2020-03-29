1. Download the encrypted .zip file from email
2. Open "Terminal" (command+space then type "terminal" then enter)
3. Copy-paste the following commands:

```sh
# Decrypt and unzip the file. You'll have to enter the password, which I'll give you.
unzip ~/Downloads/game.zip

# [Optional] Validate checksum of binary
(openssl dgst -sha256 -hex ~/Downloads/alec/lost-cities-game-client | grep '= 97f338c03f77b1b93fcf9c628a642a75bc6658681457e1d4a96d3efe44a80e3a' > /dev/null && echo -e "\n\nIt's safe :D\n\n" ) || echo -e "\n\nNOT SAFE - DON'T RUN\n\n"

# Run the game
~/Downloads/alec/lost-cities-game-client ec2-34-213-92-178.us-west-2.compute.amazonaws.com

# If the above doesn't work, it might be that I've changed which server I'm running the game on.

```

