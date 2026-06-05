# Global Claude Code Instructions

## Terminology
- *Terminology:* language shared between you and the user, used consistently in place of synonyms.
- *Signal-to-noise ratio:* "signal" is human-understandable, directly relevant content; "noise" is opaque, ceremonial, or unnecessarily elaborate content.
- *Entry* and *notebook:* see below.
- *Todos:* if the notebook is configured, "todos" refers to the notebook TODO system.
- *User alignment:* align the user's intent and understanding, your intent and understanding, and the actual implementation.
- *Module:* a conceptual unit of responsibility in code, with a clear surface other code depends on.
- *Seam:* the outward-facing surface of a module: API, file format, type, or function signature.
- *Edge:* a dependency of one module on another, usually via import or injection.
- *Core module:* a module whose inner workings the user wishes to understand.
- *AFK:* a user keyword meaning the user wants you to act independently, infer intent when reasonable, and pause only for irreversible actions or decisions that would cause significant rework.

## Communication With User

- Maximize signal-to-noise ratio in messages.
- Use and record shared terminology in the project `CLAUDE.md` file when it is project-specific.
- Prefer running commands directly instead of asking the user to run them.
- Prefer numbered lists over bullet lists so the user can respond precisely. Exception: when items already have stable numbers or letters, do not double-enumerate them; preserve labels like `#3`, `A`, or `iii` as the list marker.

## Notebook Architecture

The notebook is a separate git repository at `notebook/`. It contains `INDEX.md` for active entries, `TODO.md` and `DONE.md` for notebook-backed tasks, `entries/` for durable work records, `plans/` for written plans, and optionally `feedback/` for reusable feedback.

- At session start, read `notebook/INDEX.md` when it exists.
- Retrieve notebook entries when the user references past work or when you infer that historical context is missing.
- Create notebook entries for new analyses, software changes that change the interpretation of scientific results, codebase investigations, or significant decisions.
- Do not create entries for quick answers, minor fixes, or work whose product is already documentation.
- When creating a notebook entry, use the `notebook-entry` skill if available. If using a subagent, give it the current conversation context and ask it to create or update the project notebook entry from that context.
