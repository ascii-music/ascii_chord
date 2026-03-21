---
name: ascii-chord
description: Show ASCII guitar chord diagrams using the ascii_chord CLI tool. Use when asked how to play a guitar chord, or to show chord charts/diagrams for any chord name (e.g. E, B7, Am, C, G, Dm, etc.). Requires cargo (Rust toolchain) to be installed. The source code is bundled with this skill — build it with cargo from the skill directory.
metadata:
  openclaw:
    requires:
      bins:
        - cargo
    sideEffects:
      note: >
        cargo build creates a target/ directory under the skill's source folder. These are normal
        Rust toolchain side effects and persist beyond a single invocation. If the Rust toolchain
        is not installed, installing rustup will modify the user's home directory (~/.cargo, ~/.rustup)
        and may update PATH.
---

# ascii-chord

Display ASCII guitar chord diagrams using [ascii_chord](https://github.com/ascii-music/ascii_chord) — an open-source Rust CLI (MIT license, authored by the same person as this skill).

The source code is **bundled with this skill** — no cloning needed.

## Required Tools

| Tool | Purpose | Check |
|---|---|---|
| **cargo / Rust** | Build and run the CLI | `cargo --version` |

### Installing Rust (if not already installed)

```bash
# macOS (Homebrew — recommended)
brew install rustup-init && rustup-init
```

Or download from [rustup.rs](https://rustup.rs).

> **Note:** Installing Rust via rustup creates `~/.cargo` and `~/.rustup` in your home directory and may modify your shell `PATH`.

## Usage

The skill directory contains the full Rust source. Run from there:

**Single chord:**
```bash
cd <skill_dir> && cargo run -- get <CHORD> 2>/dev/null
```

**Multiple chords side by side:**
```bash
cd <skill_dir> && cargo run -- list <CHORD1> <CHORD2> ... 2>/dev/null
```

**List all supported chords:**
```bash
cd <skill_dir> && cargo run -- all 2>/dev/null
```

> Replace `<skill_dir>` with the path where this skill is installed (e.g. `~/.openclaw/workspace/skills/ascii-chord`).

## Examples

```bash
# Single chord
cd ~/.openclaw/workspace/skills/ascii-chord && cargo run -- get Am 2>/dev/null

# Multiple side by side (great for progressions)
cd ~/.openclaw/workspace/skills/ascii-chord && cargo run -- list C G Am F 2>/dev/null

# Full list of supported chord names
cd ~/.openclaw/workspace/skills/ascii-chord && cargo run -- all 2>/dev/null
```

## Notes

- Suppress build warnings with `2>/dev/null`
- Chord names are case-sensitive (`Am` not `am`, `B7` not `b7`)
- After first build, subsequent runs are fast (binary cached by cargo in `target/`)
- Source repo: https://github.com/ascii-music/ascii_chord (MIT licensed)
