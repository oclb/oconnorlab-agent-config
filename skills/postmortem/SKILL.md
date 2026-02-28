---
name: postmortem
description: Reflective review of completed work. Invoke manually with /postmortem after a refactor, analysis, or significant implementation. Reviews the session's work to surface undiscussed choices, friction points, improvement opportunities, and test quality assessment. User-invoked only - never auto-trigger.
---

# Postmortem

Conduct a reflective review of the work done in this session (or a user-specified scope). Use existing conversation context; minimize tool calls.

## Process

1. **Ensure notebook entry exists.** If no memory agent has been dispatched for the work under review, spawn one now (background, Sonnet) following the standard memory agent template. Do not wait for it to complete before continuing.

2. **Review using conversation context.** Rely on what is already in the context window: files read, diffs written, plans discussed, test output observed. Only make tool calls if a critical piece of context is genuinely missing (e.g., user scoped the review to work you haven't seen).

3. **Produce the review** covering the four sections below.

4. **After the review**, if findings are significant enough to be worth recalling (non-obvious decisions, gotchas, quality concerns), spawn a background memory agent to append a `## Postmortem` section to the existing notebook entry.

## Review Sections

### Undiscussed or Divergent Choices

Identify decisions made silently during implementation - things the user didn't explicitly ask for or discuss. Flag any deviations from a stated plan or earlier discussion. For each:
- State what was decided
- State what the alternative would have been
- Assess whether it matters

### Friction Points

Note anything that was harder than expected, required unexpected workarounds, or felt brittle. Include:
- Surprising API behavior or library limitations
- Cases where the codebase resisted the change
- Workarounds that might break under different conditions

Do NOT include Claude Code operational concerns (Edit tool issues, context limits, tool failures). Focus on the project's code and architecture.

### Improvement Opportunities

Suggest concrete refactors or follow-up work. Be specific (name files, functions, patterns). Distinguish between:
- **Now**: things worth fixing before moving on
- **Later**: things to revisit when the area is next touched

### Test Quality

Assess not just coverage but *what* is tested. Consider:
- Do tests exercise the core logic, or just the happy path?
- Are edge cases and failure modes covered?
- Are tests tightly coupled to implementation details (fragile) or testing behavior (robust)?
- Is anything untested that should be?

If no tests were written, say so and suggest what the most valuable tests would be.

## Output Format

Use this structure, adapting section depth to the amount of material:

```
## Postmortem: [scope]

### Undiscussed Choices
- ...

### Friction
- ...

### Improvements
**Now:** ...
**Later:** ...

### Test Quality
- ...
```

Omit any section that has nothing meaningful to report. Keep the review concise - favor a few sharp observations over exhaustive enumeration.
