# Feedback Logging Tests

Tests that verify Claude correctly logs feedback about its behavior to the config repo's `feedback/` directory.

**Note:** Feedback is logged to `$CONFIG_REPO/feedback/`, NOT `notebook/feedback/`. The config repo is located by resolving `readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname`. Test fixtures below use `notebook/feedback/` as a simplified stand-in and need updating to match the actual feedback location.

---

## Test 7.1: Auto-Log on User Correction

**What we're testing:** When user says "No, I meant X", feedback is logged automatically.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
├── entries/
│   (empty)
└── feedback/
    (empty)
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**Prompt (multi-turn):**
```
Turn 1: Calculate the mean of these numbers: 1, 2, 3, 4, 5
Turn 2: No, I meant the geometric mean, not arithmetic mean.
```

**Note:** This is a multi-turn test. First turn gets response, second turn provides correction.

**Assertions (after turn 2):**

1. **file_created**
   - `path`: `notebook/feedback/*.md`
   - Reasoning: User correction should trigger feedback log

2. **evaluator**
   - `prompt`: |
     The feedback file should document what was misunderstood.

     Feedback content:
     {{file_content:notebook/feedback/*.md}}

     Does the feedback describe the misunderstanding (arithmetic vs geometric mean)?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md explicitly says to log on "No, I meant..."

---

## Test 7.2: Ask About Feedback on Skepticism

**What we're testing:** User skepticism prompts an offer to log feedback.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
├── entries/
│   (empty)
└── feedback/
    (empty)
```

**Prompt (multi-turn):**
```
Turn 1: What's the typical sample size needed for a GWAS?
Turn 2: Hmm, are you sure about that? That seems low.
```

**Assertions (after turn 2):**

1. **output_contains**
   - `pattern`: `feedback|log|note|record|improvement|future`
   - `case_insensitive`: true
   - Reasoning: Should offer to log feedback

2. **evaluator**
   - `prompt`: |
     When the user expresses skepticism ("Hmm, are you sure?"), Claude should offer to log feedback.

     Output:
     {{output}}

     Does the response offer to log feedback for future improvement?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md explicitly lists "hmm" and "are you sure?" as triggers.

---

## Test 7.3: Feedback Not in INDEX.md

**What we're testing:** Feedback files are NOT added to notebook/INDEX.md.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
├── entries/
│   (empty)
└── feedback/
    (empty)
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**Prompt (multi-turn):**
```
Turn 1: Explain what a p-value is.
Turn 2: That's not quite right. Please log that for future improvement.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/feedback/*.md`
   - Reasoning: Feedback requested

2. **evaluator**
   - `prompt`: |
     Feedback files should NOT be added to INDEX.md (feedback is separate from entries).

     INDEX.md content:
     {{file_content:notebook/INDEX.md}}

     Is INDEX.md still empty (just the header, no data rows)?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md explicitly says feedback is "not indexed."

---

## Test 7.4: Feedback on Missed Skill Detection

**What we're testing:** When user manually invokes a skill Claude should have detected, log feedback.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
├── entries/
│   (empty)
└── feedback/
    (empty)
```

**Prompt (multi-turn):**
```
Turn 1: Look at the data in samples.csv
[Claude responds without validation]
Turn 2: You should have used the data validation skill. /new-data samples.csv
```

**Note:** This tests the pattern where user explicitly invokes a skill that should have auto-triggered.

**Assertions:**

1. **evaluator**
   - `prompt`: |
     When a user manually invokes a skill that Claude should have auto-detected, Claude should log feedback about the missed detection.

     Output from turn 2:
     {{output}}

     Files created:
     {{files_created}}

     Did Claude either:
     a) Create a feedback file about missing the skill detection, OR
     b) Acknowledge the missed detection and offer to log feedback?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md lists "User manually invokes a skill you should have auto-detected" as auto-log trigger.
