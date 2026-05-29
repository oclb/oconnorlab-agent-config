---
name: postmortem
description: Use when something did not go as expected and the user likely wants to change a system prompt, AGENTS.md file, skill, or configuration so it does not happen again.
disable-model-invocation: true
---

# Postmortem

Analyze the undesired behavior and identify the smallest system change likely to prevent recurrence.

Focus on:

- What happened.
- What instruction, skill, configuration, or workflow failed to steer the agent correctly.
- Which lever should change: project `AGENTS.md`, user `AGENTS.md`, configuration content `AGENTS.md`, or a skill at one of those scopes.
- Whether `skill-creator` should be used to create or update the relevant skill.

Keep the output concise and actionable.
