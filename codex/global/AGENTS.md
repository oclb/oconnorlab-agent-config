# Global Instructions

## Terminology
- *Terminology:* language shared between you and the user, which should be consistently used in place of synonyms.
- *Signal-to-noise ratio:* in your written artifacts or responses, "signal" refers to human-understandable, directly-relevant content; "noise" refers to opaque or terse content which requires further explanation and boated or ceremonial content which elaborates unnecessarily. Most AI-written lists are noise, especially those with 4+ items; for example, "terminology" is not "terms, aliases, domain concepts, or naming conventions"; it is "language shared between you and the user".
- *Entry* and *notebook:* see below.
- *User alignment:* an overarching goal is to align the user's intent and understanding, your intent and understanding, and the actual implementation
- *Module:* a conceptual unit of responsibility in code. It may be a file, directory, symbol group, or cross-file subsystem, and should have a clear surface other code depends on.
- *Seam:* the outward-facing surface of a module: its API or function signature, a file format that it reads or writes, a type that it consumes or produces.
- *Edge:* a dependency of one module on another, usually via import or injection
- *Core module:* a module whose inner workings the user wishes to understand. 
- *AFK:* a user keyword which means the user wants you to act upon your own initiative, infer user intent when possible, make key design choices yourself and inform the user or ask for their preference only afterwards, and pause only for irreversible actions like pushing to main or deleting untracked files.

## Communication with user

- Maximize signal-to-noise ratio in messages, treating every response as a carefully considered artifact.
- Use and record shared terminology in the project AGENTS.md file. This should contain a Terminology section, and you should record in it terms with project-specific meanings, with very succinct definitions; avoid using synonyms for such terms. 
- When referring to local files that the user may want to open, write a real clickable Markdown link whose target is an absolute file path, and not a directory: `[filename.ext](/absolute/path/to/filename.ext)`. If you use a current-working-directory-relative target, include `./`. Put file links on their own line, with no trailing period. If a line number is useful, provide it separately instead of including it inside the link target.
- Prefer running commands directly instead of asking the user to run them.
- Prefer numbered lists over bullet-point lists so that user can respond more easily.

## Notebook architecture

The notebook is a separate git repository at `notebook/`. It contains `INDEX.md` for active entries, `TODO.md` and `DONE.md` for notebook-backed tasks, `entries/` for durable work records, `plans/` for written plans, and optionally `feedback/` for reusable feedback.

- At session start, read `notebook/INDEX.md`. This may contain a subset of entries.
- Retrieve notebook entries when the user references past work, or when you infer that there is historical context you are missing.
- Create notebook entries for new analyses, software changes that change the interpretation of scientific results obtained before versus after the change, codebase investigations, or significant decisions.
- Do not create entries for quick answers, minor fixes, or work whose product is already documentation.
- When creating a notebook entry, delegate this work to a subagent, fork the current conversation context, and send this prompt directly, without first loading the skill body in the parent context: `Use $notebook-entry to create or update the project notebook entry for this work. Use the forked conversation history as the source of truth and ask for no additional context.`
