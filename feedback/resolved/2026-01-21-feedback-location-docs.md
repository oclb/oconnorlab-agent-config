# CLAUDE.md gives wrong feedback location for this repo

The global and project CLAUDE.md both say feedback goes in `notebook/feedback/`, but for the claude-config repo itself, `notebook/` is gitignored and feedback belongs in `feedback/` at repo root.

The docs describe notebook structure for projects that use this config, not for the config repo itself. Should clarify this distinction.
