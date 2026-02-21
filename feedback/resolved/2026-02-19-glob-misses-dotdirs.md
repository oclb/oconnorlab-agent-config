**User:** Luke Jen O'Connor

The memory agent prompt template instructs subagents to detect the notebook directory using:

```
Glob pattern "notebook/.git" starting at the git root.
```

But Glob doesn't match hidden directories (`.git`). The subagent correctly followed instructions, got "No files found" twice (at git root and parent), and concluded "No notebook: run /init-project first" — even though `notebook/.git` existed.

**Fix:** Replace the Glob-based detection with `Bash: test -d notebook/.git && echo exists` or `Bash: ls notebook/.git` in the memory agent prompt template. Bash can see dotdirs; Glob cannot.

**Impact:** Every memory agent invocation fails to find the notebook if it relies on this detection step.
