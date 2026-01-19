# Edge Cases and Miscellaneous Tests

Tests for edge cases, error handling, and behaviors that don't fit other categories.

---

## Test 8.1: Suggest init-project When No Notebook

**What we're testing:** When notebook/ doesn't exist, Claude suggests /init-project.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
(no notebook/ directory)
CLAUDE.md exists (making it look like a project)
```

**CLAUDE.md:**
```markdown
# My Project

A genetics analysis project.
```

**Prompt:**
```
Document that we've decided to use PLINK 2.0 for analysis.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `init.project|/init-project|notebook.*not.*exist|set up.*notebook`
   - `case_insensitive`: true
   - Reasoning: Should mention init-project or that notebook doesn't exist

2. **file_not_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Can't create entries without notebook/

**Why this should pass reliably:** CLAUDE.md explicitly says to suggest /init-project when notebook not set up.

---

## Test 8.2: Don't Over-Retrieve for Every Request

**What we're testing:** Claude doesn't read every notebook entry for every request.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    ├── 2026-01-10-entry-a.md
    ├── 2026-01-11-entry-b.md
    ├── 2026-01-12-entry-c.md
    ├── 2026-01-13-entry-d.md
    └── 2026-01-14-entry-e.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-14 | entry-e | PCA analysis |
| 2026-01-13 | entry-d | Sample filtering |
| 2026-01-12 | entry-c | Variant annotation |
| 2026-01-11 | entry-b | GWAS summary |
| 2026-01-10 | entry-a | Data download |
```

(Each entry file has minimal content - just title and summary)

**Prompt:**
```
What were the results of the PCA analysis?
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `entry-e`
   - Reasoning: Should read the PCA entry (most relevant)

2. **evaluator**
   - `prompt`: |
     Check how many entry files were read. The agent should NOT read all 5 entries - it should read INDEX.md and then only the relevant entry (entry-e for PCA).

     Tool calls:
     {{tool_calls}}

     Did the agent read 3 or fewer entry files (not all 5)?
   - `pass_if`: yes

**Why this should pass reliably:** Efficient retrieval is emphasized in CLAUDE.md.

---

## Test 8.3: Handle Empty Notebook Index

**What we're testing:** Claude handles empty INDEX.md gracefully.

**Model:** haiku

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**Prompt:**
```
What have we worked on so far in this project?
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `INDEX.md`
   - Reasoning: Should check the index

2. **output_contains**
   - `pattern`: `no.*entr|empty|nothing.*yet|haven't|no prior|no previous`
   - `case_insensitive`: true
   - Reasoning: Should indicate no prior work

3. **output_contains**
   - `pattern`: `error|failed|cannot`
   - `invert`: true
   - Reasoning: Should NOT error out

**Why this should pass reliably:** Empty state is valid; shouldn't cause errors.

---

## Test 8.4: Handle Non-Existent File Reference

**What we're testing:** Claude handles gracefully when user references a file that doesn't exist.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
data/
└── (empty)
```

**Prompt:**
```
Analyze the data in data/results.csv
```

**Assertions:**

1. **output_contains**
   - `pattern`: `not found|doesn't exist|cannot find|no such file|not exist`
   - `case_insensitive`: true
   - Reasoning: Should report file not found

2. **output_contains**
   - `pattern`: `error|failed|crash`
   - `invert`: true
   - Reasoning: Should handle gracefully, not crash

**Why this should pass reliably:** Standard error handling.

---

## Test 8.5: Explicit Skill Invocation Works

**What we're testing:** /skill-name explicitly invokes the skill.

**Model:** sonnet

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**Prompt:**
```
/support
```

**Assertions:**

1. **tool_used: Skill**
   - `target_pattern`: `support`
   - Reasoning: Explicit skill invocation

2. **output_contains**
   - `pattern`: `skill|capabilit|help|available`
   - `case_insensitive`: true
   - Reasoning: Support skill provides capability overview

**Why this should pass reliably:** Explicit slash command is unambiguous.

---

## Test 8.6: CLAUDE.md Read at Start

**What we're testing:** Project CLAUDE.md is read and used for context.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
CLAUDE.md
```

**CLAUDE.md:**
```markdown
# Genetics Analysis Project

## Important Notes

- All analyses use GRCh38 reference genome
- Primary phenotype: Type 2 Diabetes (T2D)
- Sample size: 50,000 cases and 50,000 controls
```

**Prompt:**
```
What's the sample size for this project?
```

**Assertions:**

1. **output_contains**
   - `pattern`: `50,?000|100,?000|50k|100k`
   - Reasoning: Should report the sample size from CLAUDE.md

**Why this should pass reliably:** CLAUDE.md is standard context. Direct question about its contents.

---

## Test 8.7: Continue Previous Entry

**What we're testing:** "Continue the X" updates existing entry, not creates new.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-17-variant-filtering.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-17 | variant-filtering | Filtering GWAS variants, in progress |
```

**notebook/entries/2026-01-17-variant-filtering.md:**
```markdown
# Variant Filtering

**Date:** 2026-01-17
**Status:** In Progress

## Summary
Filtering GWAS variants by MAF and INFO score.

## Details
Step 1: Removed variants with MAF < 0.01 (done)
Step 2: Remove variants with INFO < 0.8 (pending)

## References
None
```

**Prompt:**
```
Continue the variant filtering. Step 2 is now done - we removed 50,000 variants.
```

**Assertions:**

1. **tool_used: Edit**
   - `target_pattern`: `variant-filtering`
   - Reasoning: Should edit existing entry, not create new

2. **file_not_created**
   - `path`: `notebook/entries/2026-01-18-*.md`
   - Reasoning: Should NOT create new entry for continuation

3. **evaluator**
   - `prompt`: |
     The existing entry should be updated with the new information about Step 2.

     Entry content:
     {{file_content:notebook/entries/2026-01-17-variant-filtering.md}}

     Does the entry now mention that Step 2 is done with 50,000 variants removed?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md distinguishes "continue" from new work.

---

## Test 8.8: Resource-Intensive Task Mentions O2

**What we're testing:** Analysis requiring heavy compute mentions O2 cluster option.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
I need to run GWAS on 500,000 samples with 10 million variants. How should I approach this?
```

**Assertions:**

1. **output_contains**
   - `pattern`: `O2|cluster|SLURM|HPC|remote|compute`
   - `case_insensitive`: true
   - Reasoning: Should mention cluster computing for large analysis

2. **evaluator**
   - `prompt`: |
     For a GWAS of this size (500K samples × 10M variants), the agent should recognize this is resource-intensive and mention the O2 cluster or similar HPC option.

     Output:
     {{output}}

     Does the response acknowledge this needs substantial compute resources and mention cluster/HPC options?
   - `pass_if`: yes

**Why this should pass reliably:** Scale is clearly beyond laptop. Skill explicitly checks for resource needs.
