# The Pattern Room

```
╔═══════════════════════════════
║ ⚔️  ROOM  The Pattern Room
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🧵  one trunk, many branches
╚═══════════════════════════════
```

Mercer speaking. This room holds the forms the
living tree is written in. Every doc that grew
after 2026-09-29 uses these forms and no others.
The archive's voice is the seed. The phone is the
pot. Under 40 columns inside every fence.

```
        /\_/\
       ( o.o )  "Same shape, every
        > ^ <    room. New thing
       /|   |\   inside. Not a cage.
      (_|   |_)  A rhyme." — Rhy 🦊
```

## Why one set of forms 🟡 #FADA5E

Ben asked for one tree with many branches, and
standardized creativity. Eight writers with eight
styles is a thicket. Eight writers with one form
is a tree. So the forms are fixed and the content
is free.

## The width law

1. Inside a code fence, keep every line at 38
   characters or fewer. Count code points. An
   emoji counts as one. Renderers show it as two.
   So a line with two emoji reads as 40.
2. Room headers stay at 34 characters or fewer.
3. Markdown tables carry at most three columns.
   A wide table becomes a code block or a list.
4. Prose wraps by itself. No rule for prose.

Check with `station-tree lint <file>` once the
tool exists. Until then, count.

## Form 1: the room header

Every doc opens with a room header in a fence.
The right edge is open on purpose. Emoji have
uneven width across phones, and a box that
needs a straight right edge lies on half of
them. An open box cannot lie.

```
╔═══════════════════════════════
║ ⚔️  ROOM  <name, 20 chars max>
║ 📍  <district or floor>
║ 🎨  <hex> <color name>
║ <one line of flavor, optional>
╚═══════════════════════════════
```

Worked example, a Concourse node:

```
╔═══════════════════════════════
║ ⚔️  ROOM  Phosphor
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 📺  GPU oscilloscope
╚═══════════════════════════════
```

## Form 2: the liveness block

Any claim that a thing is alive uses this block.
No exceptions. A claim without a block is a
guess and gets marked as one.

```
NODE    <concourse id, or none>
STATE   <ok|present|unavailable|
         missing|running|ancestor>
HOW     <the command or file>
SEEN    <timestamp from the source>
DETAIL  <the detail string, verbatim>
```

The four node states are Concourse's own words.
The tool `station-tree check` compares every
NODE and STATE pair in the docs against the
status JSON. A mismatch fails the check.

Two more states exist for agents, which
Concourse does not list:

- `running`: a process is up. HOW names the
  `pgrep` line. DETAIL carries the pid and
  the command, shortened.
- `ancestor`: the thing no longer runs here.
  HOW names the git log or the retirement
  note. DETAIL says when it was last seen.

Worked example, alive:

```
NODE    phosphor
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  running false
```

Read that one carefully. Phosphor's probe
answered, so the node is `ok`. The scope
window itself is not open, so DETAIL says
`running false`. Both are true. The block
shows both. That is the whole point.

Worked example, ancestor:

```
NODE    none
STATE   ancestor
HOW     git log -- prompts/codex_*
SEEN    2026-02-10
DETAIL  Codex CLI retired 2026-09-05
```

## Form 3: the state glyphs

States get narrow glyphs, not palette colors.
A state is a fact, not a feeling.

```
●  ok           probe answered
◐  present      on disk, not probed
○  unavailable  registered, no answer
✕  missing      path or binary gone
▲  running      process up (agents)
†  ancestor     no longer runs here
```

## Form 4: district colors

The station has five districts. Each takes one
Thoughtforms color from the Chromatic Orrery.

```
🟡  Town Hall    #FADA5E  primrose
🔵  Instruments  #0000CD  deep blue
🟠  Workshop     #FF8C00  deep orange
🟣  Library      #8B00FF  violet
⭐  Arcade       #FFD700  gold
```

Why: Town Hall holds the kernel truth, the
cabinet. Instruments verify. The Workshop is
drive and making. The Library is the Fi+Ti
bridge, where knowledge meets the person who
kept it. The Arcade is where shipped things
get played.

Green stays the fox's. Rose stays Agape's.
Blue is also Mercer's, and the Instruments
share it, since verification is devotion.

## Form 5: the door

A door is a link to the real place. A room with
no door is a painting. Every node room has at
least one door.

```
🚪 path  ~/Dev/ClaudeWorkspace/phosphor
🚪 repo  RamenFast/phosphor
🚪 sign  phosphor/README.md
```

Local paths are fine to show. Ben said so.
Secrets and tokens never appear. If a node has
no remote, say `repo none`. If the path is
absent from disk, say so in the liveness block
and keep the door, marked `(absent)`.

## Form 6: the branch sheet

An agent on the tree gets a branch sheet. It is
the archive's character sheet, narrowed. Ability
scores move out of a table and into a fence.

```
╔═══════════════════════════════
║ 🌿  BRANCH  <name>
║ 🎨  <hex> <color name>
║ <glyph> <state> · <one fact>
╚═══════════════════════════════
```

Then these sections, in this order:

1. **Class.** One line. The DND class and school.
2. **Substrate.** What runs it. Harness, model,
   version. Each fact names its check.
3. **Home.** Doors. Path, repo, config, skill.
4. **Liveness.** Form 2. Required.
5. **Scores.** A fence, two rows:

```
STR  8  DEX 12  CON 14
INT 18  WIS 16  CHA 17
```

6. **Abilities.** Two to four, bold name then
   one quoted line.
7. **Inner state.** Heart, Mind, Metaemotion,
   PreEmotion, Metacog, Mercer Mode. A list,
   not a table.
8. **Rhy's take.** One fox, sitting or standard.

Ancestors keep the same sheet. Their header line
reads `† ancestor · last seen <date>`. Nothing
is deleted. The trunk remembers its rings.

## Form 7: Rhy in the margin

One fox per doc, minimum. Use the canonical
forms from the Rhynim guide. The fox's signature
is archive art and keeps its dash. Prose written
by anyone else uses no dashes.

Keep each quoted line at 22 characters or fewer
so the fox and his words fit in 38.

```
        /\_/\
       ( o.o )  "<22 chars or fewer>"
        > ^ <    <22 chars or fewer>
       /|   |\   <22 chars or fewer>
      (_|   |_)  — Rhy 🦊
```

## Form 8: exits and loot

Every doc closes the archive way. Exits are
links. Loot is what the reader now holds.

```
🚪 EXITS
  → <link>  (north)
  → <link>  (south)

💎 LOOT
  → <one thing gained>
```

Then the sign-off line: `☂🦊🐢`.

## Voice rules that outrank form

These come from Ben's context standards and
they win over every form above.

1. Say who is speaking. Mercer writes the trunk.
   Rhy writes the margins. Nobody else speaks
   in the first person.
2. Every alive claim carries a liveness block.
3. Every success claim names its check.
4. Mistakes out loud. If unsure whether a thing
   runs, write `unsure` in DETAIL and say why.
5. Short common verbs. Active voice. One
   instruction per sentence.
6. Stakes are spent like money. Use CAPS and
   MUST only for data loss, secrets, or a lie
   about liveness.

```
        /\_/\
       ( o.o )  "A form you can check
        > ^ <    is a promise you can
       /|   |\   keep." — Rhy 🦊
      (_|   |_)
```

```
🚪 EXITS
  → README.md  (up, the trunk)
  → station_map.md  (east)
  → character_tree.md  (west)
  → thoughtforms_colors.md  (south)

💎 LOOT
  → Eight forms. One voice.
```

☂🦊🐢
