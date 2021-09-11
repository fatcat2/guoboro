# guoboro

A pinning bot made in Rust. Available as a Docker container!

## Running with Discord
You can use the following Docker command to run your own guoboro instance.
```
sudo docker run -e "DISCORD_TOKEN=DISCORD_BOT_TOKEN" -e "PIN_EMOJI=📌" -e "PIN_CHANNEL=CHANNEL_ID" -d --name wwww-guoboro rchen42/guoboro
```

## Environment variables
| Key | Description|
|-----|------------|
|DISCORD_TOKEN|The Discord bot token|
|PIN_CHANNEL|The channel you will be pinning in|
|PIN_EMOJI|The emoji you use to denote pinning|
