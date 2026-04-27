# tsunagi

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical specification at [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md). The Compounding Directive (operational rules: solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in the org-level pleme-io/CLAUDE.md ★★★ section. Read both before non-trivial changes.


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
