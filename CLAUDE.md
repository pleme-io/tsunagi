# tsunagi

macOS-Android bridge: clipboard sync, notification mirroring, file transfer. Consumes AdbTransport, UsbEnumerator. Uses hasami (clipboard), tsuuchi (notifications), tsunagu (daemon lifecycle).

## Build

```bash
nix build
nix run .#tsunagi
cargo build
```

## Architecture

- Binary: `tsunagi`
- Language: Rust (edition 2024, rust-version 1.91.0)
- License: MIT
- Nix: substrate `rust-tool-release-flake.nix` pattern
