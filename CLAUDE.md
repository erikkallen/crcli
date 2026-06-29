# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

`crcli` is a commandline CRC calculator. It computes any of ~100 predefined CRC variants over a file, a hex string, or text.

## Commands

```shell
cargo build                       # build
cargo test                        # run tests
cargo test it_works               # run a single test by name
cargo fmt --all -- --check        # CI gate: formatting
cargo clippy -- -D warnings       # CI gate: lints (warnings are errors)
cargo run -- -t crc16 file.txt    # run locally
```

CI (`.github/workflows/ci.yml`) runs build/test/fmt/clippy across stable, beta, nightly, and MSRV **1.74.0** (bumped from 1.60 for clap 4.6). Keep changes MSRV-compatible. Releases build cross-platform binaries on GitHub release creation (`build.yml`).

## Architecture

Two files:

- **`src/lib.rs`** — `ALGO_LIST`, a `const [CrcType; 101]` table mapping each `algo_name` (e.g. `"CRC16_MODBUS"`) to a `crc_func: fn() -> CRC` from the `crc-any` crate. Entries are either a named constructor (`CRC::crc16modbus`) or an inline closure calling `CRC::create_crc(poly, width, init, xorout, reflect)` for variants `crc-any` doesn't expose directly. **This table is the source of truth** for both the computation and the clap `--type` value list — adding a CRC variant means adding one entry here, nothing else. Also holds `hex_to_bytes` (separated-hex parsing) and `find_matches` (the `--find` reverse lookup that brute-forces the whole table over the data), with unit tests.
- **`src/main.rs`** — clap arg parsing (`Opts`) and the compute loop. `--type`'s `possible_values` is generated from `ALGO_LIST`, so the table and CLI stay in sync automatically. Files are digested in 0x4000-byte chunks; hex strings are normalized (strip `0x`, left-pad each token to 2 chars, join, then `decode_to_slice`). Output prints LE/BE in both hex and decimal.

The array length in `ALGO_LIST: [CrcType; N]` is fixed — bump `N` when adding/removing entries or it won't compile. Commented-out lines (e.g. `CRC32_AIXM`) are variants `crc-any` doesn't yet provide.

## Gotchas

- Some `algo_name` typos are load-bearing as CLI values: `CRC16_AUG_CCIT` (missing T) and the `seperator` flag spelling. Changing these is a user-facing breaking change.
