# Handoffs: records, not standing orders

A handoff should tell the next reader what actually happened, what remains and which evidence to inspect. It should not invent a successful state merely to complete a form.

## Files in this directory

- `HO-20260211-001.json`: a dated report of the twelve-ring expansion, with self-reported scores.
- `HO-20260211-002.json`: a dated local-model design exchange. Its names and fields differ from the schema here, so it is not a conforming example of that contract.
- `HO-20260211-003.json`: an old request for a local model introduction, not current proof of availability.
- `HO-20260211-004.json`: a dated implementation handoff that explicitly leaves integration work open.

Preserve those reports as reports. Their claims and emotional language are not fresh measurements or new permission to start tasks.

## Templates

`template.handoff.json` and `template.payload.json` contain placeholders. They are valid JSON syntax, but are not ready-to-submit instances. Fill the identifiers, participants, time, scope, evidence and criteria from the actual task, then validate against the appropriate schema.

The old schema restricts participant names to a historical roster. Do not silently claim a current harness conforms to it. `scope` records an intention, not file visibility. Everything tracked here is public.

The handoff template deliberately leaves `ring_state` empty and omits `alignment_score`. Add an observation only when it has a defined meaning and supporting evidence. Do not use perfect or passing values as defaults.

## Legacy automation

The PowerShell local-agent loop looks for unprocessed `HO-*.json` files addressed to `LLAMA_LOCAL`. It sends their summary and listed context paths to an inference endpoint and appends the answer to the tavern board. It does not thereby prove the task was executed or read every listed file. Do not start it just to inspect these records.

[Current working set](../working_set.md) · [Working boundaries](../../docs/agent_boundaries.md)
