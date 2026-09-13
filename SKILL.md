---
name: guitar-chord
description: Show ASCII guitar chord diagrams using the ascii_chord CLI tool. Use when asked how to play a guitar chord, or to show chord charts/diagrams for any chord name (e.g. E, B7, Am, C, G, Dm, etc.). Requires cargo (Rust toolchain) to be installed. The source code is bundled with this skill — build it with cargo from the skill directory.
metadata:
  openclaw:
    requires:
      bins:
        - cargo
    install:
      - type: shell
        label: "Post-install init (gitignore + pre-warm build)"
        script: "scripts/init.sh"
        note: "Creates .gitignore for /target and pre-builds the binary. Run once after install."
    sideEffects:
      note: >
        cargo build creates a target/ directory under the skill's source folder. These are normal
        Rust toolchain side effects and persist beyond a single invocation. If the Rust toolchain
        is not installed, installing rustup will modify the user's home directory (~/.cargo, ~/.rustup)
        and may update PATH.
---

# guitar-chord

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

## Post-Install Setup

After installing, run the init script once:

```bash
bash ~/.openclaw/workspace/skills/ascii-chord/scripts/init.sh
```

This will:
1. Create a `.gitignore` in the skill directory to exclude cargo build artifacts (`/target`) from git
2. Pre-warm the cargo build cache so the first chord lookup is fast

## First Run Warning

If you skip the init script, the **first `cargo run` will take 30–60 seconds** while cargo compiles the binary. This is normal — subsequent runs are fast (binary is cached in `target/`). Do not interrupt the first build.

## Diagram Legend

```
✕  = mute this string (don't play)
◯  = open string (play unfretted)
●  = fret this position (filled dot)
═  = nut (top of fretboard)
─  = fret wire
│  = string
```

Numbers on the left (when shown) indicate the starting fret position for higher-up chords.

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

# Full list of all supported chord names
cd ~/.openclaw/workspace/skills/ascii-chord && cargo run -- all 2>/dev/null
```

## Discovering Supported Chords

Not all chord voicings are supported. If a user asks for a chord that fails, use `all` to list every supported chord name and find the closest match:

```bash
cd ~/.openclaw/workspace/skills/ascii-chord && cargo run -- all 2>/dev/null
```

## Notes

- Suppress build warnings with `2>/dev/null`
- Chord names are case-sensitive (`Am` not `am`, `B7` not `b7`)
- After first build, subsequent runs are fast (binary cached by cargo in `target/`)
- Source repo: https://github.com/ascii-music/ascii_chord (MIT licensed)
