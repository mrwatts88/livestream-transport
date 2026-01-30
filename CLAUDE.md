# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/claude-code) when working with code in this repository.

## Project Overview

This is a custom livestreaming pipeline built in Rust. Unlike typical streaming solutions that use GStreamer, this project implements the transport layer from scratch while leveraging FFmpeg for media encoding/decoding.

## Architecture

- **Producer**: Reads MPEG-TS stream from stdin (piped from FFmpeg), sends frames over UDP to consumers
- **Consumer**: Receives frames via UDP, writes to stdout (piped to FFplay for playback)
- **Signaling Server**: QUIC-based server (via s2n-quic) for ICE candidate exchange and SDP negotiation (WIP)

The project uses STUN/TURN (self-hosted coturn) for NAT traversal to enable streaming across different networks.

## Build Commands

```bash
make p          # Capture from camera via FFmpeg, pipe to producer, send over UDP
make s          # Build and run a consumer in signaling mode
make c          # Receive UDP stream, pipe to FFplay for playback
make test       # Run the test suite
make ci         # Run all CI checks (fmt, clippy, check, test)
cargo build     # Build all targets
cargo fmt       # Format code
cargo clippy    # Run lints
```

## Project Structure

```
src/
├── lib.rs          # Library root
└── bin/
    ├── producer.rs # Producer binary
    ├── consumer.rs # Consumer binary
    └── signaling.rs # Signaling server binary (WIP)
```

## Key Design Decisions

1. **Custom Transport**: Building transport from scratch rather than using GStreamer-rs for learning and control
2. **FFmpeg Integration**: Using FFmpeg CLI for input (camera capture) and output (playback via FFplay)
3. **QUIC Signaling**: Using s2n-quic for signaling server with ICE candidate exchange and SDP negotiation
4. **Self-hosted TURN**: Using coturn for NAT traversal rather than relying on third-party TURN servers

## Dependencies

- `tokio`: Async runtime
- `s2n-quic`: QUIC protocol implementation for signaling
- `bytes`: Efficient byte handling for frame data
- FFmpeg (external): Media encoding/decoding

## Testing

Tests are run via `cargo test` or `make test`. Integration tests may require FFmpeg to be installed.

## Git Commits

Do not add Co-Authored-By lines to commit messages.
