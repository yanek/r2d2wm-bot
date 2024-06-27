# 🤖 **R2D2**WM

This **Rust**-based bot is designed to help manage and dispatch **cron** tasks within a **Discord** server, making it
perfect for automating repetitive tasks. Originally developed to assist with reminders in school related Discord server.

> [!IMPORTANT]
> This crate is in *very* early developpement, and things may be incomplete, break at any moment, or just be poorly (if
> at all) documented. Clone at your own risk.

## Installation

This bot being self-hosted only, you'll need to create an application on
the [Discord Developper Portal](https://discord.com/developers/applications). Look it up, it's really not complicated.

Then you can copy your bot's token somewhere safe -- and I mean *really safe*, it's supposed to stay secret. You'll have
to put it in your configuration file later, so everything can work.

### 1a. In a docker container

An image is available on [Docker Hub](https://hub.docker.com/repository/docker/yanekosaurus/r2d2wm/general).
Please note that as I'm deploying on a Raspberry Pi 4, it is built for the `linux/arm64` architecture **only**, at least
for now.

```yaml
# Example docker-compose.yml:
services:
  r2d2wm:
    image: yanekosaurus/r2d2wm
    container_name: r2d2wm
    hostname: r2d2wm
    restart: unless-stopped
    volumes:
      - /home/user/r2d2wm-bot/config:/config
    environment:
      R2D2WM_CONFIG_PATH: "/config"
```

### 1b. Build from sources

```bash
# Clone the repo
$ git clone https://github.com/yanek/r2d2wm-bot.git

# Change the current directory
$ cd r2d2wm-bot

# Build and run
$ cargo build --release
$ cargo run
```

### 2. Configuration

```bash
# TODO
```