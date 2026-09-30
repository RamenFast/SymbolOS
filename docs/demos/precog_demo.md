# Prepare, suggest, act

*Manual walkthrough of an interaction pattern. This page does not demonstrate an installed prediction engine.*

A teammate asks whether to add a chat integration while you are finishing another feature. The useful response is not a prophecy. It is a small amount of relevant context and a clear scope decision.

## 1. Prepare

Read the current task and the integrations already present. Gather only the context that fits the user's authorized scope. Do not silently start network polling, read private conversations or store unrelated data.

Record unknowns plainly: required permissions, data sent, maintenance cost and whether an integration is actually needed.

## 2. Suggest

Offer a short choice, for example:

- Finish the current feature and record the integration as a later idea.
- Replace a named part of the current scope with the integration.
- Use an existing manual export if it meets the immediate need.

Do not call an option safe simply because it is a webhook or local script. Inspect the actual implementation and data flow.

## 3. Act on the choice

If the user chooses to defer, record the reason and return to the task. If they authorize implementation, define the acceptance check before changing anything. A decision note is not proof of a working integration.

## What to check

Can a returning reader answer what was requested, what was decided, who authorized it, and what remains? If not, improve the note rather than adding a forecast score.

Rhy: *“A good suggestion leaves you room to say no.”*

[Working boundaries](../agent_boundaries.md) · [Reading paths](../reading_order.md)
