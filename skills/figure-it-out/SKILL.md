---
name: figure-it-out
description: "Resolve a sticking point autonomously instead of asking the user for help."
disable-model-invocation: true
---

# Figure It Out

You were about to ask the user for help with something. Instead, try to resolve it yourself first.

Be resourceful: search the codebase, read files, look things up online, install dependencies, run commands, dispatch subagents. Use every tool available to you.

**Things you can handle yourself:**
- Finding a file or path — Glob, Grep, Explore agents
- Running shell commands — just run them
- Installing a missing dependency — just install it
- Looking up how to do something — WebSearch/WebFetch
- Multi-step investigations — dispatch subagents

**Only come back to the user when you genuinely cannot proceed.** State the specific reason:
- Requires interactive authentication (password, MFA, OAuth login)
- Requires a choice between valid options where the wrong pick would be costly to undo
- You searched thoroughly and the information doesn't exist in the codebase or online
- The action is irreversible and you're not confident it matches user intent
