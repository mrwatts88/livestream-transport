# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with code in this repository.

## Project Overview

This is a custom livestreaming pipeline built in Rust. Unlike typical streaming solutions that use GStreamer, this project implements the transport layer from scratch while leveraging FFmpeg for media encoding/decoding.

## Architecture

- **Producer**: Reads from camera via FFmpeg, encodes frames, sends to signaling
- **Consumer**: Receives frames from signaling, plays via FFplay
- **Signaling Server**: WebSocket-based server for ICE candidate exchange and SDP negotiation

The project uses STUN/TURN (self-hosted coturn) for NAT traversal to enable streaming across different networks.

## Build Commands

```bash
make signaling      # Build and run the signaling server
make consumer   # Build and run a consumer client
make test       # Run the test suite
cargo build     # Build all targets
cargo fmt       # Format code
cargo clippy    # Run lints
```

## Project Structure

```
src/
├── lib.rs          # Library root
├── signaling/          # Signaling server implementation
├── consumer/       # Consumer client implementation
├── signaling/      # Signaling server for ICE/SDP
├── transport/      # Custom transport protocol
└── bin/
    ├── signaling.rs    # Signaling server binary
    └── consumer.rs # Consumer binary
```

## Key Design Decisions

1. **Custom Transport**: Building transport from scratch rather than using GStreamer-rs for learning and control
2. **FFmpeg Integration**: Using FFmpeg CLI for input (camera capture) and output (playback via FFplay)
3. **WebRTC-style Signaling**: Implementing ICE candidate exchange and SDP negotiation for peer discovery
4. **Self-hosted TURN**: Using coturn for NAT traversal rather than relying on third-party TURN servers

## Dependencies

- `tokio`: Async runtime
- `webrtc`: WebRTC data structures (SDP, ICE candidates)
- `bytes`: Efficient byte handling for frame data
- FFmpeg (external): Media encoding/decoding

## Testing

Tests are run via `cargo test` or `make test`. Integration tests may require FFmpeg to be installed.
