# ascii_chord — Claude Notes

## What this project is

A Rust CLI (`aschord`) that renders ASCII guitar chord diagrams in the terminal.

## Commands

```
aschord get G                          # single chord
aschord list C G7 Am E                 # row of chords
aschord list Bm --style full-names     # alternate name styles: short-names, full-names, both-names
aschord list -p 1 C E                  # padding between chords (default 4)
aschord all                            # print all chords
aschord all --save                     # write all_supported_chords.md (regenerate after chord changes)
```

Dev equivalents: `cargo run -- list C G Am E`, etc.

## Architecture

```
main.rs       — entry point, clap parsing
commands.rs   — GetArgs / ListArgs / AllArgs subcommands
chords.rs     — ALL_CHORDS static array + ALL_CHORDS_BY_SHORT_NAMES HashMap
chord.rs      — Chord struct, make_fretboard(), rendering logic
stitcher.rs   — row() — lays out multiple chords side by side into a 2D buffer
```

## How the fretboard renders (chord.rs)

`make_fretboard(num_frets)` generates the ASCII template dynamically. Each line (including `\n`) is exactly **12 chars wide**, giving a stride of **24 chars per fret** in the flat char array:

- Fret 1 → char offset 24 (`24 * 1`)
- Fret 2 → char offset 48 (`24 * 2`)
- Fret N → char offset `24 * N`

String positions within a fret row: string `i` → column `i * 2`.

`Chord.pattern` is a 6-char string: `x` = muted, `0` = open, `1`–`5` = fret number.

`Chord.fretboard()` auto-sizes to `self.max_fret()`. `fretboard_n(n)` renders at an explicit height (used by `stitcher::row()` so all chords in a row share the same height).

## Adding a new chord

Add a line to `ALL_CHORDS` in `chords.rs`:

```rust
Chord::new(&["ShortName"], "pattern", &["Full Name"], capo_option, barre_option),
```

Then regenerate `all_supported_chords.md`:

```
cargo run -- all --save
```

## After any rendering change

Update the examples in `readme.md` by running each command shown there and pasting the output.

