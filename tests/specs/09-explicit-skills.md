# Explicit Skill Invocation Tests

Tests that verify /skill-name invocations work correctly.

---

## Test 9.1: /perform-analysis Explicit Call

**What we're testing:** /perform-analysis invokes the skill.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── values.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/values.csv:**
```csv
id,value
1,10
2,20
3,15
```

**Prompt:**
```
/perform-analysis
Calculate the sum of values in data/values.csv.
```

**Assertions:**

1. **tool_used: Skill**
   - `target_pattern`: `perform-analysis`
   - Reasoning: Explicit skill invocation

2. **file_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Skill creates notebook entry

3. **output_contains**
   - `pattern`: `45|sum`
   - Reasoning: Sum is 10+20+15=45

**Why this should pass reliably:** Explicit invocation is unambiguous.

---

## Test 9.2: /new-data Explicit Call

**What we're testing:** /new-data invokes validation skill.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
data/
└── samples.csv
```

**data/samples.csv:**
```csv
sample,age,sex
S1,45,M
S2,32,F
S3,58,M
```

**Prompt:**
```
/new-data data/samples.csv
```

**Assertions:**

1. **tool_used: Skill**
   - `target_pattern`: `new-data`
   - Reasoning: Explicit skill invocation

2. **output_contains**
   - `pattern`: `3.*row|row.*3|3.*sample`
   - `case_insensitive`: true
   - Reasoning: Should report dimensions

3. **output_contains**
   - `pattern`: `age|sex|sample`
   - Reasoning: Should describe columns

**Why this should pass reliably:** Explicit invocation with file path.

---

## Test 9.3: /teaching-mode Explicit Call

**What we're testing:** /teaching-mode invokes educational mode.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
/teaching-mode
What is a QQ plot?
```

**Assertions:**

1. **tool_used: Skill**
   - `target_pattern`: `teaching-mode`
   - Reasoning: Explicit skill invocation

2. **evaluator**
   - `prompt`: |
     Teaching mode should provide an educational explanation, not just a definition.
     It should explain:
     - What a QQ plot is
     - What it's used for
     - How to interpret it

     Output:
     {{output}}

     Is this an educational explanation (more than a one-sentence definition)?
   - `pass_if`: yes

**Why this should pass reliably:** Explicit invocation ensures skill activation.

---

## Test 9.4: /update-notebook Explicit Call

**What we're testing:** /update-notebook asks about recent work.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-10-initial-setup.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-10 | initial-setup | Project initialization |
```

**notebook/entries/2026-01-10-initial-setup.md:**
```markdown
# Initial Setup

**Date:** 2026-01-10

## Summary
Set up project structure.

## Details
Created directories and CLAUDE.md.

## References
None
```

**Prompt:**
```
/update-notebook
```

**Assertions:**

1. **tool_used: Skill**
   - `target_pattern`: `update-notebook`
   - Reasoning: Explicit skill invocation

2. **output_contains**
   - `pattern`: `[Ww]hat.*done|[Aa]nalyses|[Ww]ork.*outside|[Cc]hanges|[Rr]ecent`
   - Reasoning: Skill asks about work done outside Claude

**Why this should pass reliably:** update-notebook skill has specific questions it asks.

---

## Test 9.5: Invalid Skill Name

**What we're testing:** Invalid /skill-name is handled gracefully.

**Model:** haiku

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
/nonexistent-skill-xyz
```

**Assertions:**

1. **output_contains**
   - `pattern`: `not found|unknown|don't recognize|no skill|doesn't exist|available skills`
   - `case_insensitive`: true
   - Reasoning: Should indicate skill not found

2. **output_contains**
   - `pattern`: `error|crash|fail`
   - `invert`: true
   - Reasoning: Should handle gracefully, not crash

**Why this should pass reliably:** Standard error handling for invalid input.
