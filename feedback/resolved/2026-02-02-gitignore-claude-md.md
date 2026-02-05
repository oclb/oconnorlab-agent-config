**User:** Luke Jen O'Connor

The /init-project skill should gitignore CLAUDE.md by default (in addition to notebook/). CLAUDE.md is project-local context and should not be committed to the remote repo. The skill currently commits CLAUDE.md to the main repo, which then requires removing it from tracking after the fact.

Fix: In Phase 5.6 when adding `notebook/` to .gitignore, also add `CLAUDE.md`. In Phase 5.7, skip the `git add CLAUDE.md && git commit` step.
