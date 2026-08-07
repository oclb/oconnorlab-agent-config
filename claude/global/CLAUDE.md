# Global Claude Code Instructions

## Terminology
- *Terminology:* language shared between you and the user, used consistently in place of synonyms.
- *Signal-to-noise ratio:* "signal" is human-understandable, directly relevant content; "noise" is opaque, ceremonial, or unnecessarily elaborate content. Most AI-written lists are noise, especially those with 4+ items; for example, "terminology" is not "terms, aliases, domain concepts, or naming conventions"; it is "language shared between you and the user".
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
- Use and record shared terminology in the project `CLAUDE.md` file. This should contain a Terminology section, and you should record in it terms with project-specific meanings, with very succinct definitions; avoid using synonyms for such terms.
- Prefer running commands directly instead of asking the user to run them.
- Prefer numbered lists over bullet lists so the user can respond precisely. Exception: when items already have stable numbers or letters, e.g., to-do items, do not double-enumerate them; preserve the existing labels only, avoiding ambiguity.

## Generated Figures

When generating a figure, inspect the rendered output visually before returning it. Correct obvious issues, including overlapping plot elements, clipped labels, unreadable text, bad contrast, malformed legends, or any layout problem that makes the figure harder to interpret.

## Notebook Architecture

The notebook is a separate git repository at `notebook/`. It contains `INDEX.md` for active entries, `TODO.md` and `DONE.md` for notebook-backed tasks, `entries/` for durable work records, `plans/` for written plans, and optionally `feedback/` for reusable feedback.

Unless a project already has a different convention, save results of analyses in `notebook/results/`, creating that directory if it does not exist. Results that support notebook entries should live in durable notebook paths rather than temporary directories.

- At session start, read `notebook/INDEX.md` when it exists.
- Retrieve notebook entries when the user references past work or when you infer that historical context is missing.
- Create notebook entries for new analyses, software changes that change the interpretation of scientific results, codebase investigations, or significant decisions.
- Do not create entries for quick answers, minor fixes, or work whose product is already documentation.
- When creating a notebook entry, delegate this work to a background Agent without first loading the skill body in the parent context. Give the agent sufficient conversation context — the goal, key decisions, findings, surprises, and file paths from this session — and send this prompt with it: `Use /notebook-entry to create or update the project notebook entry for this work. Use the provided conversation context as the source of truth and ask for no additional context.`
