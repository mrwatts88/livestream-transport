# Livestream Transport

[![Format](https://github.com/mrwatts88/livestream-transport/actions/workflows/fmt.yml/badge.svg)](https://github.com/mrwatts88/livestream-transport/actions/workflows/fmt.yml)
[![Clippy](https://github.com/mrwatts88/livestream-transport/actions/workflows/clippy.yml/badge.svg)](https://github.com/mrwatts88/livestream-transport/actions/workflows/clippy.yml)
[![Test](https://github.com/mrwatts88/livestream-transport/actions/workflows/test.yml/badge.svg)](https://github.com/mrwatts88/livestream-transport/actions/workflows/test.yml)
[![Check](https://github.com/mrwatts88/livestream-transport/actions/workflows/check.yml/badge.svg)](https://github.com/mrwatts88/livestream-transport/actions/workflows/check.yml)

A custom livestreaming pipeline built from scratch in Rust. This project implements a WebRTC-inspired transport layer for streaming video from a camera to multiple clients across different networks.

### Components

- **Producer**: Captures video from camera using FFmpeg and sends encoded frames to the signaling server
- **Consumer**: Receives frames from signaling and plays them via FFplay
- **Signaling Server**: Handles ICE candidate exchange and SDP negotiation between peers
- **STUN/TURN**: Self-hosted via [coturn](https://github.com/coturn/coturn) for NAT traversal

### Key Features

- Custom transport protocol (not using GStreamer)
- Multi-client streaming across different networks
- NAT traversal via STUN/TURN
- WebRTC-style signaling for peer discovery

## Prerequisites

- Rust (latest stable)
- FFmpeg
- coturn (for STUN/TURN server)

## Development

### Setup

```bash
# Clone the repository
git clone https://github.com/mrwatts88/livestream-transport.git
cd livestream-transport

# Build the project
cargo build
```

### Running

```bash
# Start the producer
make p

# In another terminal, start a consumer in signaling mode
make s

# Or start a consumer in consumer mode
make c

# Run tests
make test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy

# Type check without building
cargo check

# Run all checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

## TURN/STUN Server Setup

Deploy coturn on your server:

```bash
# Install coturn
sudo apt install coturn

# Configure /etc/turnserver.conf
# See docs/coturn-config.md for recommended settings

# Start the service
sudo systemctl start coturn
```

## License

MIT
