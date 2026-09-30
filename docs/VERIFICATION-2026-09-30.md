# Publication Receipt

```text
╔═══════════════════════════════
║ ⚔️  ROOM  The Receipt
║ 📍  Instruments
║ 🎨  #0000CD deep blue
╚═══════════════════════════════
```

Mercer speaking. A map earns its claim one check at a time.

## Observations on 2026-09-30

- Initial `git fetch origin` found SymbolOS main equal to origin/main, with only the interrupted Character Tree untracked.
- Commit `4bb3b53` preserves the full 804-line draft before corrections. No source history was reset or replaced.
- Saved node evidence has 45 rooms: 22 ok, 18 present, three unavailable and two missing.
- `station-tree check docs/station_map.md --live --json` passed at **02:22:50-07:00**: 45 nodes seen, no unmapped nodes, no findings. [Raw comparison](evidence/live-comparison-2026-09-30.json).
- The live check compares node state words, not every detail field or successful model turns. The map retains its original observation time.
- Both public README fences measured at most **36 terminal display columns**, counting wide glyphs. The station checker also enforces its code-point width rule.
- Public URL checks covered 28 unique doors across both READMEs, the map, Character Tree and profile guide. One moved contribution-doc URL was repaired and retested with HTTP 200.
- Four map repositories returned public 404. Authenticated GitHub API checks confirmed `private: true` for NexusFormStationWork, SetupScripts, groundskeeper and sudoplz. Their map doors now say authorized login required. Local path doors remain available on Ben's machine.
- [Initial URL results](evidence/external-links-2026-09-30.txt) retain the pre-fix observation. Private 404s are access boundaries, not missing station nodes.
- `git diff --check` passed. Prose lint found zero em dashes in the two new front pages and revised Character Tree. Character lore remains creative language, not telemetry.

## Repeat the local checks

From the repo root:

```sh
station-tree all README.md \
  docs/reading_order.md \
  docs/character_tree.md \
  docs/station_map.md \
  docs/style_station.md \
  docs/CHANGELOG.md docs/ASKS.md \
  docs/VERIFICATION-2026-09-30.md \
  --json

git diff --check

station-tree check \
  docs/station_map.md --live --json
```

## Publication passed

Content commit `33f9e3d` was pushed normally to `origin main` after the eight-document check passed with no findings. At **09:31 UTC**, fetch confirmed local and remote main equal, with a clean worktree. The public raw README returned HTTP 200 and matched local bytes exactly.

README SHA-256: `e3eb4b78ce8550d8a8422e087848c164bd03074b832b5b5f961e7400aed62c7f`.

The map was regenerated twice from the amended seed, with identical output. All 45 rooms now have a path or real repository door, including concrete paths for Nexus Model Palette, Hermes Bridge and Ollama. Private-door notes survive regeneration. [Final local check](evidence/local-check-2026-09-30.json).

This closing receipt commit changes no README content. No force push was used.

## Limits and protected surfaces

The old library index retains historical formatting and framework language. Only its new living-door banner was added. This receipt does not claim every historical doc fits a phone or has current external doors.

Agent process statements refer to the saved **01:29:59-07:00** receipt. No new inference test proves that a listed model is loaded or thinking. Mercer and Rhy are symbolic roles, not processes. Pi's process absence was not reverified.

The archive, other repositories, account settings, pins and desktop windows were not changed. No new workers, app releases or renames were created. No browser visual test was run. Width and link checks are structural checks, not a screenshot of every phone renderer.

```text
        /\_/\
       ( o.o )  "A check is a door.
        > ^ <    Walk through it."
       /|   |\
      (_|   |_)  Rhy 🦊
```

🚪 EXITS

- → [Entrance](../README.md)
- → [Asks](ASKS.md)
- → [Growth Rings](CHANGELOG.md)

💎 LOOT

- → Claims with their checks, and limits in the open.

☂🦊🐢
