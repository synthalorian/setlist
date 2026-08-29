# Setlist — Agent Brief

Praise team command center: setlists, chord charts (ChordPro), one-tap transposition, PDF export for the team. Built by a worship guitarist for worship guitarists.

## Your mission: v0 core engine (no UI yet)
- [ ] Scaffold Tauri 2 + Svelte app (`npm create tauri-app` pattern), plain dark default theme
- [ ] ChordPro parser (Rust): directives (title, key, capo), chord-over-lyric lines
- [ ] Transposition engine: shift all chords ±N semitones, prefer flats/sharps per target key, handle slash chords
- [ ] Rust unit tests: transposition correctness across all 12 keys, edge cases (C#↔Db, slash chords, minors)
- [ ] CLI harness `setlist transpose <file> <semitones>` to prove the engine end-to-end

## Stack
Tauri 2 + Svelte frontend, Rust core. Local-first, no accounts.

## House Rules (non-negotiable)

- **Authorship credit:** README/docs footer is `Made by synth with synthclaw 🎹🦞` — synth first, always. Never "heavy lifting by synthclaw", never sole-author credit.
- **Theme:** plain dark theme is the DEFAULT for anything other people use. Synthwave '84 (#240037, #8F00FF, #FF7EDB, #FF00FF, #F3E70F) is strictly opt-in via config.
- **CLI naming:** the binary is the bare project name. Never a `-cli` suffix.
- **Commits:** conventional commits (`feat:`, `fix:`, `chore:`...). Local commits are fine.
- **NEVER:** push to a remote, create GitHub remotes, force-push, rewrite/delete tags, or touch `.env`/credential files.
- **No support/donation links** (BuyMeACoffee etc.) — the user removed those deliberately.
- **Done means verified:** build it AND run it before claiming completion. No stubs-as-deliverables.
