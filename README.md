# Setlist

Praise team command center: build setlists, manage chord charts, transpose keys in one tap, share with the team before Sunday.

## Why
Setlists live in group chats, chord charts in PDFs, and transposition in the worship leader's head. Setlist puts it all in one place — built by someone who actually stands on that stage.

## Features (v0.1)
- Song library with chord charts (ChordPro format)
- Setlist builder: drag-order songs, notes per slot
- One-tap key transposition across the whole set
- Offline-first; export setlist as PDF for the team
- Blackshield (steel+blood) theme by default; synthwave stays opt-in

## CLI (v0 engine harness)
The bare `setlist` binary doubles as a CLI harness for the core engine:

```console
$ setlist transpose examples/way-maker.chordpro 2
{title: Way Maker}
{key: F#}
...
[F#]Way maker, [C#]miracle worker
[D#m]Promise keeper, [B]light in the darkness
```

With no arguments it launches the Tauri app. Run the engine tests with
`cargo test` inside `src-tauri/`.

## Stack
Tauri 2 + Svelte. Local SQLite. PDF export for sharing.

## Monetization
Free core; premium tier for team sync + cloud library later.

## Roadmap
- [ ] Song library + ChordPro parser/renderer
- [ ] Transposition engine
- [ ] Setlist builder + ordering
- [ ] PDF export
- [ ] Mobile shell (Android)

---
Made by [synth](https://github.com/synthalorian) with synthclaw 🎹🦞
