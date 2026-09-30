# Keep the request in view

*Fictional planning exercise. No product gate, team vote or successful deployment is asserted.*

You are finishing a character-sheet export. Someone asks for a live chat integration as well. The question is whether that belongs in the same task, not whether all integrations are bad.

## Write down the boundary

```text
Deliver: a local JSON export
Check: open it in the target reader
Not included: live chat delivery
Unknown: the chat permission model
```

The words do the work. A shield symbol can help somebody remember the boundary. It cannot enforce it.

## Respond with a real choice

Explain the additional behavior and ask whether to defer it, replace part of the current scope, or authorize a separate task. If the user has already made that decision, follow it rather than asking again.

Credentials require appropriate storage and authorization, but possessing an API token is not itself a privacy violation. Determine what data would be sent, to whom, and under whose authority.

## Leave a useful note

Record the decision, its owner, the reason and the next action. Include a commit or receipt only if it exists. Do not put an example hash or imagined team vote into a real evidence ledger.

The turtle asks: **“Did we finish the thing we came here to do?”**

[Working boundaries](../agent_boundaries.md) · [Contribution guide](../../CONTRIBUTING.md)
