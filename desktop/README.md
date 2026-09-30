# Terminal explorer prototype

This inherited Rust/ratatui application is an optional view of SymbolOS's older vocabulary, not the current station hub. Concourse remains the installed tool-discovery interface on Ben's machine.

## What the source actually does

- Reads `schemaVersion` and `symbols` from a supplied JSON file.
- Displays an eight-ring color list with a cycling highlight.
- Lets you browse symbol names, meanings and tags.
- Displays seven hardcoded historical character entries. Their HP gauges are illustrative constants, not live agent health.
- Cycles through twenty fixed quotations when you press `r`. Despite the old d20 label, this is sequential, not random.

There is no document-opening action and no live service polling. The twelve-ring design in other inherited documents is not what this implementation renders.

## Source-level controls

- Left/right or Tab: switch tabs.
- Up/down or `j`/`k`: move through the symbol list.
- `r`: next quotation.
- `q` or Escape: quit.

The source searches for `symbol_map.shared.json` relative to its working directory or accepts a path as its first argument. A missing file produces an empty fallback, while malformed JSON returns an error.

## Review status

The complete 606-line source was read during the September 30 curation. This is not a fresh build or terminal acceptance receipt. The manifest's `MIT` license declaration needs reconciliation with the repository's license history before distributing this component. Error paths after terminal setup also need a cleanup check before calling the tool robust.

Build inputs are in [Cargo.toml](Cargo.toml) and [Cargo.lock](Cargo.lock). Implementation is in [src/main.rs](src/main.rs). Keep this prototype out of the default quickstart until those questions are resolved.

[Living entrance](../README.md)
