# PassVault — Documentation

PassVault is a terminal-based password toolkit written in Rust.
It generates, strengthens, and evaluates passwords entirely in the terminal — no storage, no network, no external accounts.

This documentation focuses on **why** each design decision was made — not just how things work.

---

## Index

### Design Decisions

High-level choices that shape the entire project.

- [Why a TUI](#why-a-tui) — No GUI overhead, works over SSH, stays out of the way
- [Why no storage](#why-no-storage) — Passwords that never touch disk can never leak from disk
- [Why two generation modes](#why-two-generation-modes) — Memorability and entropy solve different problems

### Internals

How each module works and why it was designed that way.

- [Generator](#generator) — Memorable (word-based) and Random (character-based) strategies
- [Strengthener](#strengthener) — Padding, capitalization, shuffle — without replacing the original
- [Evaluator](#evaluator) — Ten-point scoring system and what each criterion means
- [UI Architecture](#ui-architecture) — Screen state machine, event loop, theming

---

## Quick Start

```bash
# Clone
git clone https://github.com/viincnt/PassVault
cd PassVault

# Build & run
cargo run

# Or build a release binary
cargo build --release
./target/release/passvault
```

**Requirements:** Rust 1.85+ (edition 2024), a terminal with Unicode support.

---

## Features

| Feature        | What it does                                                      |
| -------------- | ----------------------------------------------------------------- |
| **Generate**   | Creates a brand-new password from scratch                         |
| **Strengthen** | Takes a weak password and pads/mutates it into something stronger |
| **Evaluate**   | Scores any password on a 0–10 scale with per-criterion feedback   |

---

## Keybindings

### Global

| Key   | Action                   |
| ----- | ------------------------ |
| `← →` | Navigate between options |
| `↵`   | Confirm / select         |
| `Esc` | Go back one screen       |
| `q`   | Quit (main menu only)    |

### Result screen

| Key                   | Action                   |
| --------------------- | ------------------------ |
| `c`                   | Copy result to clipboard |
| `↵` / `Esc` / `Space` | Return to main menu      |

---

## Design Decisions

### Why a TUI

A graphical interface would add a window manager dependency and make the tool unusable over SSH or inside scripts.
`ratatui` + `crossterm` give a polished, keyboard-driven experience that runs anywhere a terminal does.

### Why no storage

PassVault deliberately stores nothing. There is no vault file, no master password, no encrypted database.
The tool is a _generator and evaluator_, not a manager — passwords live in your clipboard or password manager of choice.
A tool that never writes secrets to disk cannot leak them from disk.

### Why two generation modes

Entropy and memorability pull in opposite directions.

- **Random** mode maximises entropy per character — a 20-character random password is harder to brute-force.
- **Memorable** mode maximises recallability — a word-based password with leet-speak mutations is still typeable from memory and clears the bar for most services.

Offering both means the tool is useful whether you're locking a KeePass vault (random, long) or setting a recovery code you might need to recite aloud (memorable).

---

## Internals

### Generator

**Memorable** (`generate_memorable`): picks N words at random from `src/assets/words.txt`, joins them with `-`, then optionally runs each word through `mutate_word` and appends a random special character.

`mutate_word` applies probabilistic leet-speak substitutions (`a→@`, `e→3`, `o→0`, etc.) and random capitalisation. The probabilities are tuned low enough that the word stays recognisable but the charset expands significantly.

**Random** (`generate_random`): builds a charset of lowercase + uppercase + digits (+ specials if requested), then samples it uniformly. Before shuffling, it guarantees at least one character from each required class — so the result always passes its own evaluator.

### Strengthener

`strengthen` takes an existing password and:

1. Pads it to a minimum length of 8 if it's shorter.
2. Appends 4 random characters unconditionally (increasing entropy even for already-long passwords).
3. Randomly uppercases a random subset of alphabetic characters.
4. Shuffles the entire result.

The original password characters are always preserved — the user's muscle memory for their base password remains intact.

### Evaluator

`evaluate` returns a score from **0 to 10** by checking ten independent criteria, each covering a different attack vector:

| Points | Criterion                                         | Attack vector                    |
| ------ | ------------------------------------------------- | -------------------------------- |
| +1     | Contains lowercase letters                        | Charset diversity                |
| +1     | Contains uppercase letters                        | Charset diversity                |
| +1     | Contains digits                                   | Charset diversity                |
| +1     | Contains symbols                                  | Charset diversity                |
| +1     | Length ≥ 8                                        | Brute force                      |
| +1     | Length ≥ 12                                       | Brute force                      |
| +1     | Length ≥ 16                                       | Brute force (gold standard)      |
| +1     | No character repeated 3× in a row                 | Pattern-based attacks            |
| +1     | No sequential runs (`123`, `abc`, `qwerty`, etc.) | Keyboard walk / sequence attacks |
| +1     | Doesn't contain a known common password           | Dictionary attacks               |

The UI maps this to three labels: **Weak** (0–7), **Fair** (8–9), **Strong** (10).
The score is displayed as a large number with a fill bar so the result is readable at a glance.

Passwords shorter than 5 characters are rejected before evaluation with an error — they are too short to produce a meaningful score.

### UI Architecture

The app is a simple state machine. `Screen` is an enum; `App` holds all runtime state.

```
Screen::MainMenu
  ├── Screen::Generator
  │     ├── Screen::GenWordCount  (memorable path)
  │     │     └── Screen::GenSpecials → Screen::Result
  │     └── Screen::GenLength     (random path)
  │           └── Screen::GenSpecials → Screen::Result
  ├── Screen::Strengthener → Screen::Result
  └── Screen::Evaluator    → Screen::Result
```

Each screen has a dedicated `draw_*` function in `src/ui/screens/` and a corresponding `on_*` key handler in `App`. The event loop polls at 100 ms, keeping CPU use negligible.

**Theme** lives in `src/ui/screens/theme.rs` as a set of `Color` constants and `Style` helper functions. The accent colour is a burnt-orange (`#C9462A`) chosen to be distinctive on both dark and light backgrounds.

---

## Project Structure

```
PassVault/
├── src/
│   ├── main.rs                     — entry point, terminal setup and teardown
│   ├── modules.rs                  — module declarations
│   ├── modules/
│   │   ├── generator.rs            — memorable and random password generation
│   │   ├── strengthener.rs         — password padding and mutation
│   │   └── evaluator.rs            — ten-criterion scoring
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── app.rs                  — App state, Screen enum, event loop, key handlers
│   │   └── screens/
│   │       ├── mod.rs              — shared widgets: draw_header, draw_help, draw_choice
│   │       ├── theme.rs            — colour palette and style helpers
│   │       ├── main_menu.rs        — logo + four-option navigation
│   │       ├── generator.rs        — generation type selector
│   │       ├── input.rs            — generic text input screen (word count, length, etc.)
│   │       └── result.rs           — password display, evaluation score, copy shortcut
│   └── assets/
│       └── words.txt               — word list for memorable generation
└── Cargo.toml
```

---

## Dependencies

| Crate          | Version | Purpose                                              |
| -------------- | ------- | ---------------------------------------------------- |
| `ratatui`      | 0.29    | Terminal UI framework                                |
| `crossterm`    | 0.28    | Cross-platform terminal input/output                 |
| `tui-big-text` | 0.7     | Large pixel-font text for the logo and score display |
| `rand`         | 0.8     | Cryptographically seeded RNG for password generation |
| `arboard`      | 3       | Cross-platform clipboard access                      |

---

## License

MIT — see [LICENSE](LICENSE) for details.
