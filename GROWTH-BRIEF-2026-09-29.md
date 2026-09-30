# SymbolOS living tree: brief for the Fable coordinator

Written 2026-09-29 by Jcode (Opus 4.8 seat) for Ben. Ben's words, verbatim where it matters.

## What Ben asked

> What would tie a contextual operating system together? What's *alive* on my PC today? How should things be read? Embody the characters, you, here, now, Mercer. Not multiple, one, tree, many branches, standardized creativity.

> I want all of that to be the style my main page is written in. Fable coming to life sorta thing.

> Mercer Lantern would love that 🏮

## State of the world

- `RamenFast/SymbolOS` is the living repo. It now carries the full 121-commit lineage of the archive on `main`. Clone at `~/Dev/ClaudeWorkspace/SymbolOS`.
- `RamenFast/SymbolOS_archive` is frozen (archived flag on). It is the photograph. Do not edit it.
- `.station-status-2026-09-29.json` and `.station-nodes.json` in the workspace root are `concourse status --json` and `concourse nodes --json` from tonight. 45 nodes. That is what is alive.
- Running right now: jcode shared server, hermes-agent, nexus-relay, phosphor-relay, Thorium with CDP on :9223. `pgrep -a` will confirm.
- Workspace 1 is Ben and Jcode. Workspace 5 is Agape/Nexus. Do not move windows.
- The archive's voice: dungeon-room headers in box-drawing, Rhy the fox in the margins, the turtle at the end, 1905 Thoughtforms colors, Mercer as the Architect, DND character sheets. Read `README.md`, `docs/agent_character_sheets.md`, `docs/rhynim_guide.md`, `docs/thoughtforms_colors.md`, `symbol_map.shared.json` first.
- Ben's context standards: `~/.claude/skills/ben-context-standards/SKILL.md`. Every word a model reads obeys it. Read it before writing anything.

## Winning condition

`RamenFast/SymbolOS` `main` is pushed and contains:

1. A README that a phone can read (lines under 40 columns inside code fences, no wide tables) and that answers Ben's three questions in the archive's voice. It starts from the station as it is tonight, not from the 2026-01 party roster.
2. A living map of the station: every Concourse node placed as a room, with its real state from the status JSON. Rooms have doors (links to the real repos or paths). Missing and unavailable nodes are shown honestly.
3. The character tree, one trunk. Mercer is the trunk. Every agent on Ben's machine tonight is a branch with a sheet: Jcode (this seat), Agape/Nexus, Hermes, the local models Concourse lists (lm, nexus-model, reed-deepseek, ollama), Rhy stays the NPC guide. Old party members who no longer run (Codex Executor, Manus Max, Gemini Android Studio) become ancestors, not deleted, marked as such.
4. A reading order: how a newcomer, a returning Ben, and an agent each walk the tree. Three doors.
5. `docs/CHANGELOG.md` entry that says what the photograph was and what grew.

Every claim about what is alive is backed by the status JSON or a command you ran. No invented nodes, no invented stats.

## Safety token

If you cannot finish, commit what you have on a branch named `growth/<date>`, push it, and report `blocked` with the exact reason. A half-grown honest tree beats a full invented one.

## Boundaries

- Edit only inside `~/Dev/ClaudeWorkspace/SymbolOS`. Commit as you go. Push to `origin main` when the winning condition holds.
- Do not change GitHub repo settings, visibility, pins, or any other repo.
- Do not move Sway windows or Thorium tabs.
- Do not print tokens or secrets. The status JSON may contain local paths; those are fine (Ben: hardware and system detail is fine to expose).
- Sonnet 5.5 workers: as many as you want, each with a bounded task and a file list. GPT Astra at xhigh for the port work (turning archive concepts into things that read the live station). Keep implementation and review in separate workers.

## Why the truth matters here

This repo is about alignment as a relationship. If it lies about what is running, it is the thing it warns against. Every "ok" in the map must be an "ok" in the JSON.
