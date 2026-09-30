# The Station Map

```
╔═══════════════════════════════
║ ⚔️  ROOM  The Station Map
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🗺  45 rooms, one photograph
╚═══════════════════════════════
```

Mercer speaking. This is the station as it
answered on the night the tree was planted.
Every room below is a Concourse node. Every
state is the word Concourse used. Nothing here
is remembered. It is read from the JSON, and
the JSON came from `concourse status --json`.
Private repo doors need authorized login.
A public 404 is not a station probe state.

To take the photograph again, run that command
and then `station-tree render`. The map regrows.
To test this map against the station right now,
run `station-tree check docs/station_map.md --live`.

```
        /\_/\
       ( o.o )  "A map that argues
        > ^ <    with the ground lies.
       /|   |\   This one asks first.
      (_|   |_)  — Rhy 🦊
```

## The legend

States are Concourse's own words. A state is
a fact, not a feeling.

```
●  ok           probe answered
◐  present      on disk, not probed
○  unavailable  registered, no answer
✕  missing      path or binary gone
```

Districts take one Thoughtforms color each.

```
🟡  Town Hall    #FADA5E  primrose
🔵  Instruments  #0000CD  deep blue
🟠  Workshop     #FF8C00  deep orange
🟣  Library      #8B00FF  violet
⭐  Arcade       #FFD700  gold
```

## The tally

Photograph taken `2026-09-30T00:30:57-07:00`.
Source `concourse status --json`.

```
●  ok            22
◐  present       18
○  unavailable    3
✕  missing        2
   total         45
```

## 🟡 Town Hall `#FADA5E`

The kernel truth lives here. The cabinet, the law, the bench, the hub, and the neighbor's lantern.

6 rooms.

### ◐ The Filing Cabinet `cabinet`

```
╔═══════════════════════════════
║ ⚔️  ROOM  The Filing Cabinet
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🗄  Ben's governance, sealed
╚═══════════════════════════════
```

Ben's governance doc. Binding on every agent on this machine. Ben alone edits it.

```
NODE    cabinet
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/AGENTS.md`
- 🚪 repo none

### ◐ Agent CLI Standard `standard`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Agent CLI Standard
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 📐  the interface law
╚═══════════════════════════════
```

The agent-first interface law, and the live audit of every app that has a command surface.

```
NODE    standard
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/AGENT-CLI-STANDARD.md`
- 🚪 repo none

### ◐ Station Bench `station`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Station Bench
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🗺  the outside-view bench
╚═══════════════════════════════
```

The outside-view bench. The SymbolOS map, the convention, and the snapshot that seeded Concourse.

```
NODE    station
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/NexusFormStationWork`
- 🚪 repo [RamenFast/NexusFormStationWork](https://github.com/RamenFast/NexusFormStationWork)
- 🚪 sign `~/Dev/ClaudeWorkspace/NexusFormStationWork/STATION-MAP.md`

Private repository door. Authorized GitHub login required.

### ◐ Concourse `concourse`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Concourse
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🏛  the hub, GUI and CLI
╚═══════════════════════════════
```

The hub itself. One core, a GUI for Ben and a CLI for agents. `concourse status --json` made this map.

```
NODE    concourse
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/concourse`
- 🚪 repo [RamenFast/concourse](https://github.com/RamenFast/concourse)
- 🚪 sign `~/Dev/ClaudeWorkspace/concourse/README.md`

### ◐ The Dwelling `dwelling`

```
╔═══════════════════════════════
║ ⚔️  ROOM  The Dwelling
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🏮  Nexus's home lantern
╚═══════════════════════════════
```

Nexus's home lantern. Hers. Visit kindly and change nothing uninvited.

```
NODE    dwelling
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Nexus/🏮Home.html`
- 🚪 repo none

### ○ Hearth `hearth`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Hearth
║ 📍  Town Hall
║ 🎨  #FADA5E primrose
║ 🔥  Talkie chat
╚═══════════════════════════════
```

Native chat for Talkie 1930-13B. An egui app plus a loopback model server.

```
NODE    hearth
VER     0.1.0
STATE   unavailable
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  error sending request for url
        (http://127.0.0.1:8046/v1/mode
        ls)
```

- 🚪 path `~/Dev/projects/hearth`
- 🚪 repo none
- 🚪 sign `~/Dev/projects/hearth/docs/AGENTS.md`

○  The model server on :8046 did not answer. The app is installed at /usr/local/bin/hearth. Fix: start the Hearth model server, then rerun `hearth status --json`. Unsure whether Ben wants it up tonight.

## 🔵 Instruments `#0000CD`

Verification is devotion. These rooms watch the machine and say what they see.

5 rooms.

### ● Phosphor `phosphor`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Phosphor
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 📺  GPU oscilloscope
╚═══════════════════════════════
```

GPU oscilloscope. Traces everything the PC plays.

```
NODE    phosphor
VER     4.7.4
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  running false
```

- 🚪 path `~/Dev/ClaudeWorkspace/phosphor`
- 🚪 repo [RamenFast/phosphor](https://github.com/RamenFast/phosphor)
- 🚪 sign `~/Dev/ClaudeWorkspace/phosphor/README.md`

The probe answered, so the node is ok. The scope window is not open, so DETAIL says running false. Both are true.

### ✕ Surveyor `surveyor`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Surveyor
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 🛰  network observatory
╚═══════════════════════════════
```

Network observatory. Flows, listeners, exposure, one shared session.

```
NODE    surveyor
STATE   missing
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ~/Dev/ClaudeWorkspace/cartogra
        ph/zig-out/bin/surveyor: No
        such file or directory (os
        error 2)
```

- 🚪 path `~/Dev/ClaudeWorkspace/cartograph`
- 🚪 repo [RamenFast/cartograph](https://github.com/RamenFast/cartograph)
- 🚪 sign `~/Dev/ClaudeWorkspace/cartograph/CARTOGRAPH-STATION.html`

✕  The binary is not built. The cartograph source is on disk with a build.zig. Fix: `cd ~/Dev/ClaudeWorkspace/cartograph && zig build`. Unsure whether the installed zig matches the project. Say so if it does not.

### ● SysMon `sysmon`

```
╔═══════════════════════════════
║ ⚔️  ROOM  SysMon
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 📊  ten rooms, one API
╚═══════════════════════════════
```

System monitor. CPU, GPU, network, sensors. Ten rooms and one agent-drivable API.

```
NODE    sysmon
VER     3.0.3
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/sysmon`
- 🚪 repo [RamenFast/sysmon](https://github.com/RamenFast/sysmon)
- 🚪 sign `~/Dev/ClaudeWorkspace/sysmon/README.md`

### ◐ Groundskeeper `groundskeeper`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Groundskeeper
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 🌙  estate survey
╚═══════════════════════════════
```

Estate survey. Drives weighed, rooms walked, the moving-day record kept.

```
NODE    groundskeeper
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/groundskeeper`
- 🚪 repo [RamenFast/groundskeeper](https://github.com/RamenFast/groundskeeper)

Private repository door. Authorized GitHub login required.

### ● degoog `degoog`

```
╔═══════════════════════════════
║ ⚔️  ROOM  degoog
║ 📍  Instruments
║ 🎨  #0000CD deep blue
║ 🔍  privacy meta-search
╚═══════════════════════════════
```

Privacy meta-search, forked for loopback trust. Port 4444 on this PC.

```
NODE    degoog
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  http 200
```

- 🚪 path `~/Dev/gitRepos/degoog`
- 🚪 repo [fccview/degoog](https://github.com/fccview/degoog)

## 🟠 Workshop `#FF8C00`

Drive and making. The local models, the voices, the tools that change the desktop, and the harnesses that other agents run in.

21 rooms.

### ◐ Wisp `wisp`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Wisp
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🕯  household-spirit REPL
╚═══════════════════════════════
```

Household-spirit REPL. Sessionless `--json` turns through the local models.

```
NODE    wisp
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/wisp`
- 🚪 repo none

### ● Local Models `lm`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Local Models
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🦙  llama.cpp substrate
╚═══════════════════════════════
```

The llama.cpp substrate. Server, watcher, VRAM arbitration.

```
NODE    lm
VER     1.0.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  state stopped · mode none ·
        used_pct 4.5
```

- 🚪 path `~/Nexus/💻HomePC/🧰LocalModels`
- 🚪 repo none

The probe answered, so the node is ok. The server itself is stopped and no model is loaded. Both are true. The hermes-bridge room serves the model list on its behalf.

### ● sdgen `sdgen`

```
╔═══════════════════════════════
║ ⚔️  ROOM  sdgen
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🖼  diffusion one-shots
╚═══════════════════════════════
```

stable-diffusion.cpp one-shots. VRAM frees itself on exit.

```
NODE    sdgen
VER     1.0.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Nexus/💻HomePC/🧰LocalModels`
- 🚪 repo none

### ● Nexus Model Palette `nexus-model`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Nexus Model Palette
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🎨  her substrate switch
╚═══════════════════════════════
```

Nexus switches her own substrate here. List, current, switch. Tonight's default is claude-opus-5-5.

```
NODE    nexus-model
VER     1.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  alias null · default
        claude-opus-5-5
```

- 🚪 repo none

- 🚪 path `~/.hermes/skills/model-palette/scripts/nexus-model`

### ◐ Form-Diffusion Lab `form-lab`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Form-Diffusion Lab
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🪞  the mirror room
╚═══════════════════════════════
```

The mirror room. Compiler, critique, recipes. Nexus's.

```
NODE    form-lab
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Nexus/🛠️TheWorkshop/📁Projects/🪞Form-Diffusion-Lab`
- 🚪 repo none

### ● sudoplz `sudoplz`

```
╔═══════════════════════════════
║ ⚔️  ROOM  sudoplz
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🔑  agent root, audited
╚═══════════════════════════════
```

Agent root. `sudoplz sudo <cmd>`, no prompt, any directory, every elevation audited.

```
NODE    sudoplz
VER     0.4.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/sudoplz`
- 🚪 repo [RamenFast/sudoplz](https://github.com/RamenFast/sudoplz)
- 🚪 sign `~/Dev/ClaudeWorkspace/sudoplz/docs/AGENTS.md`

Private repository door. Authorized GitHub login required.

### ◐ Blossom Shell `blossom-shell`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Blossom Shell
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ ❀  bash to zsh, reversible
╚═══════════════════════════════
```

Reversible bash to zsh switcher, themed to the desktop.

```
NODE    blossom-shell
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/blossom-shell`
- 🚪 repo [RamenFast/blossom-shell](https://github.com/RamenFast/blossom-shell)

### ● Intercom `intercom`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Intercom
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🎙  say and hear
╚═══════════════════════════════
```

The station's two-way voice. `say` is piper TTS. `hear` is whisper.cpp STT. CPU one-shots.

```
NODE    intercom
VER     0.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  tts_ready true · stt_ready
        true
```

- 🚪 path `~/Dev/ClaudeWorkspace/intercom`
- 🚪 repo [RamenFast/intercom](https://github.com/RamenFast/intercom)
- 🚪 sign `~/Dev/ClaudeWorkspace/intercom/README.md`

### ● Nexus Voice `nexus-voice`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Nexus Voice
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌸  her local voice
╚═══════════════════════════════
```

Nexus's local anime voice. NeuTTS-Air clone. GPU when free, CPU fallback.

```
NODE    nexus-voice
VER     0.2.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ready true · default
        nexus-anime · selected_device
        gpu
```

- 🚪 path `~/Nexus/💻HomePC/🧰LocalModels/tts-neutts`
- 🚪 repo none

### ● Hermes Bridge `hermes-bridge`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Hermes Bridge
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌉  loopback model bridge
╚═══════════════════════════════
```

OpenAI-compatible loopback bridge over the local models. Port 8109. Tonight it lists gem4-12b-h-qat.

```
NODE    hermes-bridge
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  http 200
```

- 🚪 repo none

- 🚪 path `~/Nexus/💻HomePC/🧰LocalModels/hermes_bridge.py`

### ● Station Helper `station-helper`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Station Helper
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🔘  hands behind buttons
╚═══════════════════════════════
```

The hands behind the map's buttons. Loopback :9123.

```
NODE    station-helper
VER     1.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  http 200
```

- 🚪 path `~/Dev/ClaudeWorkspace/NexusFormStationWork`
- 🚪 repo [RamenFast/NexusFormStationWork](https://github.com/RamenFast/NexusFormStationWork)

Private repository door. Authorized GitHub login required.

### ○ Ollama `ollama`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Ollama
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🦜  LLM runner :11434
╚═══════════════════════════════
```

System-wide LLM runner on loopback :11434.

```
NODE    ollama
STATE   unavailable
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  not listening
```

- 🚪 repo none

- 🚪 path `/usr/local/bin/ollama`

○  Nothing listens on :11434. The binary is at /usr/local/bin/ollama and the systemd unit is inactive (`systemctl is-active ollama`, user and system, both said inactive). Fix: `sudoplz sudo systemctl start ollama` if Ben wants it up. The station runs its local models through lm and hermes-bridge, so quiet may be the intended state.

### ● Greenhouse `greenhouse`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Greenhouse
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌿  the setup system
╚═══════════════════════════════
```

The setup system. Grows a minimal Mint into this machine. Installer, doctor, manual.

```
NODE    greenhouse
VER     0.3.6
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  failed 0
```

- 🚪 path `~/Dev/ClaudeWorkspace/SetupScripts`
- 🚪 repo [RamenFast/SetupScripts](https://github.com/RamenFast/SetupScripts)
- 🚪 sign `~/Dev/ClaudeWorkspace/SetupScripts/README.md`

Private repository door. Authorized GitHub login required.

### ● blossom `blossom`

```
╔═══════════════════════════════
║ ⚔️  ROOM  blossom
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ ❀  the theme authority
╚═══════════════════════════════
```

The theme authority. One theme, every surface, ctrl+alt+escape to reload.

```
NODE    blossom
VER     0.3.6
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  active blossom
```

- 🚪 path `~/Dev/ClaudeWorkspace/SetupScripts/tools/blossom`
- 🚪 repo [RamenFast/SetupScripts](https://github.com/RamenFast/SetupScripts)

Private repository door. Authorized GitHub login required.

### ● nightbloom `nightbloom`

```
╔═══════════════════════════════
║ ⚔️  ROOM  nightbloom
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌙  lid and power
╚═══════════════════════════════
```

Lid and power choreography. The lid keeps working. The button means stop.

```
NODE    nightbloom
VER     0.3.6
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  running false
```

- 🚪 path `~/Dev/ClaudeWorkspace/SetupScripts/tools/nightbloom`
- 🚪 repo [RamenFast/SetupScripts](https://github.com/RamenFast/SetupScripts)

Private repository door. Authorized GitHub login required.

### ● Attention Beacon `attention-beacon`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Attention Beacon
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ !  a bounded window
╚═══════════════════════════════
```

A bounded attention window for a real question. Open shows a 10-second preview.

```
NODE    attention-beacon
VER     1.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/.claude/skills/attention-beacon`
- 🚪 repo none
- 🚪 sign `~/.claude/skills/attention-beacon/SKILL.md`

### ● Desktop UI `desktop-ui`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Desktop UI
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ ⌘  window control
╚═══════════════════════════════
```

Scoped native window inspection, screenshots, and gesture control.

```
NODE    desktop-ui
VER     0.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/phosphor/tools/desktop-ui`
- 🚪 repo [RamenFast/phosphor](https://github.com/RamenFast/phosphor)
- 🚪 sign `~/Dev/ClaudeWorkspace/phosphor/tools/desktop-ui/SKILL.md`

### ● Blender `blender`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Blender
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ ◈  headless geometry
╚═══════════════════════════════
```

Local geometry studio with a structured headless CLI.

```
NODE    blender
VER     1.0.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/puzzlinData/tools/blender-agent`
- 🚪 repo none
- 🚪 sign `~/Dev/ClaudeWorkspace/puzzlinData/tools/blender-agent/README.md`

### ● Prime Maintenance `prime-agent-maintain`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Prime Maintenance
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🔧  stage Prime releases
╚═══════════════════════════════
```

Stage verified custom Prime releases without restarting live sessions.

```
NODE    prime-agent-maintain
VER     0.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/prime-agent`
- 🚪 repo [PrimeIntellect-ai/prime-agent](https://github.com/PrimeIntellect-ai/prime-agent)
- 🚪 sign `~/Dev/prime-agent/local/SESSION.md`

### ◐ Reed (DeepSeek) `reed-deepseek`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Reed (DeepSeek)
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌿  DeepSeek harness
╚═══════════════════════════════
```

DeepSeek V4 Pro harness. Full access. Only the sudoplz skill enabled.

```
NODE    reed-deepseek
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/deepseek-harness`
- 🚪 repo [deepseek-ai/deepseek-harness](https://github.com/deepseek-ai/deepseek-harness)
- 🚪 sign `~/.prime-agent/workspace/deepseek-harness-install/RECEIPT.md`

Concourse says present because it probes the binary. A `dsh web` process is also up on 127.0.0.1:3080 tonight (`pgrep -a -f 'dsh web'`). Both are true.

### ○ Web Pro Worker `web-pro-worker`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Web Pro Worker
║ 📍  Workshop
║ 🎨  #FF8C00 deep orange
║ 🌐  web-Pro provider
╚═══════════════════════════════
```

Standalone web-Pro provider. Account setup and live acceptance pending.

```
NODE    web-pro-worker
VER     0.1.0
STATE   unavailable
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  The facade is unavailable:
        error sending request for url
        (http://127.0.0.1:8127/health)
```

- 🚪 path `/home/ben/Dev/web-pro-worker`
- 🚪 repo none
- 🚪 sign `/home/ben/Dev/web-pro-worker/docs/AGENT-INTERFACE.md`

○  The facade on :8127 did not answer. The node's own line says setup is pending. Fix: `web-pro-worker doctor`, then the web-pro-worker skill. Not a tonight problem.

## 🟣 Library `#8B00FF`

The Fi and Ti bridge. Knowledge that met the person who kept it.

5 rooms.

### ◐ PKM `pkm`

```
╔═══════════════════════════════
║ ⚔️  ROOM  PKM
║ 📍  Library
║ 🎨  #8B00FF violet
║ 📚  knowledge restructure
╚═══════════════════════════════
```

Knowledge-base restructure pipeline.

```
NODE    pkm
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/PKM`
- 🚪 repo none

### ◐ Bookmarks `bookmarks`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Bookmarks
║ 📍  Library
║ 🎨  #8B00FF violet
║ 🔖  bookmark corpus
╚═══════════════════════════════
```

Browser bookmark corpus plus shiori.

```
NODE    bookmarks
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/bookmarks`
- 🚪 repo none

### ◐ Puzzlin Data `puzzlin`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Puzzlin Data
║ 📍  Library
║ 🎨  #8B00FF violet
║ 🧠  brain-wiring dataset
╚═══════════════════════════════
```

Brain-wiring dataset and its viewers.

```
NODE    puzzlin
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/puzzlinData`
- 🚪 repo none

### ◐ Spotify Centogram `centogram`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Spotify Centogram
║ 📍  Library
║ 🎨  #8B00FF violet
║ 🎵  fans-also-like
╚═══════════════════════════════
```

Fans-also-like discovery. bridge_finder feeds the map's DISCOVER tab.

```
NODE    centogram
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Nexus/🛠️TheWorkshop/📁Projects/🎵SpotifyCentogram`
- 🚪 repo none

### ◐ Obsidian Management `obsidian`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Obsidian Management
║ 📍  Library
║ 🎨  #8B00FF violet
║ 💎  vault notes
╚═══════════════════════════════
```

Vault migration and management notes.

```
NODE    obsidian
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/ObsidianManagement`
- 🚪 repo none

## ⭐ Arcade `#FFD700`

Where shipped things get played. Games, viewers, the browser, and the retro bench.

8 rooms.

### ● Holographic Eye `holo-eye`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Holographic Eye
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ ⊙  glass cockpit
╚═══════════════════════════════
```

Local glass cockpit for holographic memory.

```
NODE    holo-eye
VER     1.3.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/HolographicViewer`
- 🚪 repo [RamenFast/holographic-eye](https://github.com/RamenFast/holographic-eye)
- 🚪 sign `~/Dev/ClaudeWorkspace/HolographicViewer/README.md`

### ✕ Local Generation `localgen`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Local Generation
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🎬  TTS, music, avatars
╚═══════════════════════════════
```

TTS, music, avatar rigs.

```
NODE    localgen
STATE   missing
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  /home/ben/Dev/ClaudeWorkspace/
        LocalGeneration is missing
```

- 🚪 path `~/Dev/ClaudeWorkspace/LocalGeneration` (absent)
- 🚪 repo none

✕  The directory is gone from disk. Nothing else on the station points at it. Fix: unsure. Ask Ben whether Local Generation moved or retired. If retired, remove the node from the registry and this room becomes an ancestor.

### ◐ Gaming `gaming`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Gaming
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🕹  lemmings, porkCRAFT
╚═══════════════════════════════
```

lemmings, porkCRAFT, zoombinis. The zoombinis room below has its own probe.

```
NODE    gaming
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/gaming`
- 🚪 repo none

### ● Mossgate `mossgate`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Mossgate
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🚪  terminal adventure
╚═══════════════════════════════
```

Keyboard-first point-and-click terminal adventure.

```
NODE    mossgate
VER     0.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ready true · room old_workshop
```

- 🚪 path `~/Dev/ClaudeWorkspace/mossgate` (absent)
- 🚪 repo none
- 🚪 sign `~/Dev/ClaudeWorkspace/mossgate/README.md` (absent)

The source path is gone from disk. The installed binary at ~/.local/bin/mossgate answered the probe. Both are true, so the door is marked absent and the state stays ok.

### ◐ iPhone 4S Retro `iphone`

```
╔═══════════════════════════════
║ ⚔️  ROOM  iPhone 4S Retro
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 📱  retro revival
╚═══════════════════════════════
```

Retro device revival project.

```
NODE    iphone
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/iphone4s-retro`
- 🚪 repo [RamenFast/iphone4s-retro](https://github.com/RamenFast/iphone4s-retro)

### ◐ Thorium `thorium`

```
╔═══════════════════════════════
║ ⚔️  ROOM  Thorium
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🌐  CDP on :9223
╚═══════════════════════════════
```

Browser config. The agent surface is CDP on :9223 (the thorium skill). Tonight a Thorium with that port is running.

```
NODE    thorium
STATE   present
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  on disk
```

- 🚪 path `~/Dev/ClaudeWorkspace/Thorium`
- 🚪 repo none

### ● BITROT `bitrot`

```
╔═══════════════════════════════
║ ⚔️  ROOM  BITROT
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🦋  toolsmith game
╚═══════════════════════════════
```

Hacking toolsmith game. Build the exploit, ride it till it rots.

```
NODE    bitrot
VER     0.1.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `~/Dev/ClaudeWorkspace/bitrot` (absent)
- 🚪 repo none

The source path is gone from disk. The installed binary at /usr/bin/bitrot answered the probe. Both are true.

### ● zoombinis `zoombinis`

```
╔═══════════════════════════════
║ ⚔️  ROOM  zoombinis
║ 📍  Arcade
║ 🎨  #FFD700 gold
║ 🧩  the logical adventure
╚═══════════════════════════════
```

The logical adventure, native on Linux.

```
NODE    zoombinis
VER     2.2.0
STATE   ok
HOW     concourse status --json
SEEN    2026-09-30T00:30:57-07:00
DETAIL  ok
```

- 🚪 path `/home/ben/Dev/ClaudeWorkspace/gaming/zoombinis`
- 🚪 repo none
- 🚪 sign `/home/ben/Dev/ClaudeWorkspace/gaming/zoombinis/docs/NATIVE-AGENT-INTERFACE.md`

```
        /\_/\
       ( o.o )  "Twenty-two answered.
        > ^ <    Five did not. All
       /|   |\   forty-five are here.
      (_|   |_)  — Rhy 🦊
```

🚪 EXITS

- → [README.md](../README.md) (up, the trunk)
- → [character_tree.md](character_tree.md) (west)
- → [reading_order.md](reading_order.md) (east)
- → [style_station.md](style_station.md) (south)

💎 LOOT

- → The station, as it answered.

☂🦊🐢
