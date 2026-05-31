---
name: skill-creator
description: Guide for creating effective skills. Use when users want to create a new skill or update an existing skill that extends Codex with specialized knowledge, workflows, or tool integrations.
disable-model-invocation: true
license: Complete terms in LICENSE.txt
---

# Skill Creator

Create skills that give another Codex instance the context, workflow, rules, and assets it needs to do a specific job well. Skills should start out small because it is easier to add extra rules or context later when it is found to be missing than it is to delete unneeded content which silently adds context bloat.

## Quick Workflow

1. Align with user intent: what user requests should trigger the skill, what scope is intended, and what failure modes the skill should prevent. If scope is unclear, ask whether the skill is project-specific, user-specific, or shared.
2. Scrape the project notebook or AGENTS.md file for user's previous guidance, gotchas that have been encountered before, or context that is clearly relevant.
3. Initialize new skills with `scripts/init_skill.py <skill-name> --path <output-directory>` unless editing an existing skill. Add `--resources scripts,references,assets` only for resource types the skill really needs, and use `--examples` only when placeholders will speed iteration.
4. Draft or edit the skill.
5. Validate skills with `scripts/quick_validate.py <path/to/skill-folder>`. When distributing beyond local or repo use, package the skill as a Codex plugin; use `scripts/package_skill.py` only when the user explicitly needs a legacy `.skill` archive.
6. Provide the user with a clickable link to `SKILL.md`, and prompt them to read it, try it out, and iterate following their instructions.

## Scope

When creating a skill, choose the storage model before writing files:

- Project-specific: create it in the active project's local skill directory and keep its content tied to that project.
- User-specific: create it in the user's Codex skills directory so it follows that user across projects.
- Shared: create it in the user's local copy of the config repo, symlink it into the user or project skills directory for trial use, and do not publish it upstream until the user explicitly says it is ready. When they approve, add it to the shared config repo through a PR.

## Structure

Every skill has a required `SKILL.md` with YAML frontmatter and optional bundled resources:

```text
skill-name/
|-- SKILL.md
|-- agents/
|   `-- openai.yaml
|-- scripts/
|-- references/
`-- assets/
```

- `SKILL.md`: frontmatter plus the core instructions. Put `name` and a trigger-focused `description` in frontmatter; put "when to use" guidance in `description`, not the body.
- `agents/openai.yaml`: OpenAI/Codex UI metadata. Generate or refresh it with `scripts/generate_openai_yaml.py` or `scripts/init_skill.py --interface key=value`; read `references/openai_yaml.md` when setting non-obvious fields.
- `scripts/`: executable helpers for deterministic or repeatedly generated operations.
- `references/`: detailed documentation to load only when needed.
- `assets/`: templates, images, boilerplate, fonts, or other files used in outputs.

Keep distributable-skill frontmatter minimal unless the target platform explicitly supports more fields. In this repo, preserve local metadata such as `disable-model-invocation` or `license` when already present. Avoid extra files such as `README.md` unless the skill's actual job requires them.

## SKILL.md Rules

Keep `SKILL.md` under 1,000 words. Shorter is often better: a skill can and should be under 100 words when it prescribes a simple workflow or provides basic context.

Maximize signal-to-noise ratio:

- Omit rules that you would probably follow anyway.
- Omit unnecessary or long lists. You have a tendency to write overly-long lists; do not write lists like "entry points, commands, APIs, routes, scripts, or workflows" when "relevant entry points" says the same thing. When revising, ask yourself if the lists you write are really needed.
- Do not create subsections that do the work of one sentence.

Use substructure judiciously:
- Move detail out of `SKILL.md` when it is more than ~100 words and likely needed less than half the time.
- For multi-step or branching workflows, see `references/workflows.md`.
- For strict output formats, see `references/output-patterns.md`.
- Keep references one level deep from `SKILL.md`, and say when to read each referenced file.

## Validation Checklist

Before finishing:

- The trigger description is specific enough to distinguish this skill from neighboring skills.
- `SKILL.md` is under 1,000 words.
- Scripts, references, and assets are included only when they will be reused.
- `agents/openai.yaml` exists for standalone skills and still matches `SKILL.md`.
- Any added script has been run or otherwise validated.
- The skill avoids time-sensitive facts unless it instructs Codex how to refresh them.
- The skill has maximal signal-to-noise and no filler subsections.
