# Rustify Telegram

**Rustify Telegram** is a lightweight backend service that receives Gotify-style notifications and forwards them to a Telegram chat. It is built in Rust using **Actix Web** and **Reqwest**, with optional TLS via **Rustls**.

This project is designed to replace outdated Gotify plugins, providing a reliable and modern alternative for forwarding network device notifications or other alerts to Telegram.

## TODO
- [x] Use MUSL to build statically linked binary (multi arch)

## Table of Contents
 
<!-- TOC -->
* [Rustify Telegram](#rustify-telegram)
  * [TODO](#todo)
  * [Features](#features)
  * [Getting Started](#getting-started)
    * [Requirements](#requirements)
    * [Environment Variables](#environment-variables)
    * [Running with Docker](#running-with-docker)
<!-- TOC -->
---

## Features

- Receives JSON payloads from Gotify or compatible clients
- Forwards notifications to a Telegram bot
- Supports dynamic linking (no static musl required)
- Lightweight Docker deployment with multi-stage builds
- Configurable environment variables for security and flexibility
- Cross-architecture support: Added separate build stages for x86_64 and ARM64
---

## Getting Started

### Requirements

- Rust 1.90+
- Docker & Docker Compose (optional)
- Telegram Bot token and Chat ID

### Environment Variables

| Variable                | Description                                      |
|-------------------------|--------------------------------------------------|
| `TELEGRAM_BOT_TOKEN`    | Telegram bot token for sending messages         |
| `TELEGRAM_CHAT_ID`      | Chat ID where notifications will be sent        |
| `GOTIFY_CLIENT_TOKEN`   | Token expected for incoming Gotify requests     |
| `PORT`                  | Port for the Actix Web server (default: 8396)  |

---

### Running with Docker

1. Build the Docker image:

```bash
# For x86_64 (Intel/AMD servers)
docker build --platform linux/amd64 -t rustify-telegram:amd64 .

# For ARM64 (Apple Silicon, Raspberry Pi)
docker build --platform linux/arm64 -t rustify-telegram:arm64 .
```

2. Run the container
```bash
   docker run -d \
   -e TELEGRAM_BOT_TOKEN="your_bot_token" \
   -e TELEGRAM_CHAT_ID="your_chat_id" \
   -e GOTIFY_CLIENT_TOKEN="your_gotify_token" \
   -e PORT="8396" \
   -p 8396:8396 \
   rustify-telegram
```

3. Usage

Send a POST request to the /message endpoint:
```bash
curl -X POST "http://<host>:<port>/message?token=<GOTIFY_CLIENT_TOKEN>" \
-H "Content-Type: application/json" \
-d '{
"title": "Unknown Device Connected",
"message": "Device XYZ joined the network"
}'
```

The service will forward the notification to the configured Telegram chat.