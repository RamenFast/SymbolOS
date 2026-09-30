# Open integration questions found during curation

These are specific gaps found by reading the source, not completed fixes or permission to launch an implementation project.

## Shared-map contract mismatch

The root `symbol_map.shared.json` uses `schema_version`. The documented schema at `docs/symbol_map.shared.schema.json` and the desktop TUI expect `schemaVersion`.

The other schema, `symbol_map.shared.schema.json`, requires metadata absent from the current data: PreEmotion lacks `added`, and Rhy's evolution entry lacks `reason` and `mercer_id`.

**Next check:** establish one versioned contract, inspect each consumer, and demonstrate a real load/validation path before advertising interchange compatibility. Do not invent missing historical dates or identifiers merely to make validation pass. During curation, the existing shape remains unchanged and these mismatches remain open.

## Prototype functionality is narrower than its descriptions

The inherited Go gateway matches a registry entry but returns a placeholder rather than forwarding. Filesystem and memory write operations are unimplemented. The Python memory server likewise does not perform the advertised Git commits.

**Next decision:** either retain these only as historical prototypes or authorize a separate implementation scope. Current reader-facing guides must not prescribe their nonexistent write/commit path. No prototype service was started during this review.

## Terminal prototype distribution

`desktop/Cargo.toml` declares MIT while the repository has a GPL license. The TUI also expects a different map version key and only restores terminal state on its normal exit path.

**Next check:** inspect licensing provenance, data compatibility and real terminal error recovery before distributing or installing it. The curation does not silently relicense or claim a working build.

## Historical observations

The saved station/agent receipts mix probes, filesystem presence, configuration and narrative annotations. A healthy probe can report a stopped model. A process count does not establish model output or an identity's inner state.

**Next check:** when current liveness is needed, gather a fresh bounded receipt and name the exact claim it supports. Keep old evidence dated rather than editing it into a current success report.

## Legacy alignment report overclaims its coverage

The read-only `scripts/symbolos_alignment_report.py --json` was exercised during curation. It returned overall `PASS` and exit 0 while its own map result reported version `unknown`. Its glyph comparison did correctly match 24 JSON glyphs to 24 documentation glyphs.

Source review explains the limit: `check_ring_coverage` puts every ring in the covered list whether a matching tag exists or not. Required-schema checks test file presence, not validation of the map. The privacy dashboard counts directory names as public/party/private rather than checking access controls.

**Next decision:** repair or retire this legacy report before using it as an acceptance gate. For now, report the narrow glyph-set result separately. Its overall `PASS` does not resolve the schema mismatch, establish liveness, or make tracked files private.

[Current asks](ASKS.md) · [Contribution guide](../CONTRIBUTING.md)
