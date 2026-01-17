# aura-bot

[![ci](https://github.com/AnnaThorne/aura-bot/actions/workflows/ci.yml/badge.svg)](https://github.com/AnnaThorne/aura-bot/actions/workflows/ci.yml)  
Aura farming discord bot written in Rust.

## How to run
Create a compose file:
```
services:
  aura-bot:
    image: ghcr.io/annathorne/aura-bot:main
    restart: unless-stopped
    env_file:
      - .env
```
Create a .env file with the values as seen in .env.sample.  
Run
```
docker compose up
```
