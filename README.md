a brief Discord bot that sends daily embedded messages of Neon Genesis Evangelion characters that I wrote as a birthday present to a friend. Took this opportunity to learn the Rust programming language and its Discord API crate [serenity](https://github.com/serenity-rs/serenity).

to run on your respective Discord server channel, clone the repo:
```
git clone https://github.com/ronydahdal/evabot.git
```

cd into the project directory and create a .env file with your bot token and channel ID:
```
cd evabot
touch .env file
```
example .env file:
```
DISCORD_TOKEN:MDSA7899jdMDSUDNSA&1DUSDHDUIS8471
CHANNEL_ID:138291231278938
```

build the rust environment:
```
cargo build 
```

run the bot:
```
cargo run 
```
