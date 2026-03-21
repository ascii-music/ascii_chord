---
name: ascii-chord
description: Show ASCII guitar chord diagrams using the ascii_chord CLI tool. Use when asked how to play a guitar chord, or to show chord charts/diagrams for any chord name (e.g. E, B7, Am, C, G, Dm, etc.). Requires git and cargo (Rust toolchain) to be installed. First use clones https://github.com/yzhong52/ascii_chord into /tmp and builds it with cargo — review the source before running if desired.
metadata:
  openclaw:
    requires:
      bins:
        - git
        - cargo
    sideEffects:
      note: >
        If the Rust toolchain is not installed, the SKILL.md suggests installing rustup — this modifies
        the user's home directory (~/.cargo, ~/.rustup) and may update PATH. cargo build also creates
        a target/ directory under /tmp/ascii_chord. These are normal Rust toolchain side effects
        and persist beyond a single invocation.
    thirdPartyCode:
      note: >
        This skill clones https://github.com/yzhong52/ascii_chord (MIT, authored by the same person as
        this skill) and builds it locally with `cargo run`. You are compiling and executing code fetched
        from GitHub. Review the repository before building if you have concerns. The repo is open-source
        and MIT licensed.
---

# ascii-chord

Display ASCII guitar chord diagrams using [ascii_chord](https://github.com/yzhong52/ascii_chord) — an open-source Rust CLI (MIT license, authored by the same person as this skill).

## Required Tools

| Tool | Purpose | Check |
|---|---|---|
| **git** | Clone the source repo | `git --version` |
| **cargo / Rust** | Build and run the CLI | `cargo --version` |

## Before First Use

This skill clones `https://github.com/yzhong52/ascii_chord` into `/tmp` and builds it with `cargo run`. This executes third-party code on your machine. The repository is open-source (MIT) and authored by the same person as this skill — you can review it at https://github.com/yzhong52/ascii_chord before proceeding.

### Installing Rust (if not already installed)

```bash
# macOS (Homebrew — recommended)
brew install rustup-init && rustup-init
```

Or download the installer from [rustup.rs](https://rustup.rs) and follow the instructions there.

> **Note:** Installing Rust via rustup will create `~/.cargo` and `~/.rustup` in your home directory and may modify your shell `PATH`. This is standard Rust toolchain behavior and persists after the skill runs.

### Cloning the repo

Check if already cloned; clone if not:

```bash
[ -d /tmp/ascii_chord ] || git clone https://github.com/yzhong52/ascii_chord /tmp/ascii_chord
```

No further installation step is needed — `cargo run` builds and runs in one step. The compiled binary is cached in `/tmp/ascii_chord/target/` and reused on subsequent runs.

## Usage

**Single chord:**
```bash
cd /tmp/ascii_chord && cargo run -- get <CHORD> 2>/dev/null
```

**Multiple chords side by side:**
```bash
cd /tmp/ascii_chord && cargo run -- list <CHORD1> <CHORD2> ... 2>/dev/null
```

**List all supported chords:**
```bash
cd /tmp/ascii_chord && cargo run -- all 2>/dev/null
```

## Examples

```bash
# Single chord
cd /tmp/ascii_chord && cargo run -- get Am 2>/dev/null

# Multiple side by side (great for progressions)
cd /tmp/ascii_chord && cargo run -- list C G Am F 2>/dev/null

# Full list of supported chord names
cd /tmp/ascii_chord && cargo run -- all 2>/dev/null
```

## Notes

- Suppress build warnings with `2>/dev/null`
- Chord names are case-sensitive (`Am` not `am`, `B7` not `b7`)
- After first build, subsequent runs are fast (binary is cached by cargo in `/tmp/ascii_chord/target/`)
- Source repo: https://github.com/yzhong52/ascii_chord (MIT licensed)
