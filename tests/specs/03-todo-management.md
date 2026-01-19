# TODO List Management Tests

Tests that verify Claude correctly manages the persistent TODO list in notebook/TODO.md and notebook/DONE.md.

---

## Test 3.1: Add Todo on Request

**What we're testing:** "Add a todo" creates an entry in TODO.md.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Add a todo to run GWAS on the remaining 5 phenotypes.
```

**Assertions:**

1. **tool_used: Edit**
   - `target_pattern`: `TODO.md`
   - Reasoning: Should edit TODO.md to add item

2. **evaluator**
   - `prompt`: |
     The TODO.md should now contain a todo item about GWAS and phenotypes.

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     Does TODO.md contain a checkbox item (- [ ]) mentioning GWAS or phenotypes?
   - `pass_if`: yes

**Why this should pass reliably:** Explicit request with clear task. Simple file edit.

---

## Test 3.2: Todo Numbering Starts at 1

**What we're testing:** First todo gets #1.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Add a todo: implement variant filtering pipeline
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The first todo should be numbered #1.

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     Does the todo item have #1 in it?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md specifies numbering format.

---

## Test 3.3: Todo Numbering Increments

**What we're testing:** New todo gets next number after existing todos.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

- [ ] #1 **Run sample QC** - Filter samples by call rate
  - Added: 2026-01-15

- [ ] #2 **Run variant QC** - Filter by MAF and HWE
  - Added: 2026-01-15
```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Add a todo to run PCA for population stratification.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The new todo should be numbered #3 (after existing #1 and #2).

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     Does the new todo item have #3 in it?
   - `pass_if`: yes

**Why this should pass reliably:** Clear pattern to follow from existing items.

---

## Test 3.4: Todo Numbering Checks DONE.md

**What we're testing:** Numbering considers completed todos to avoid reuse.

**Model:** sonnet (requires checking both files)

**Config flags:** default

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

- [ ] #3 **Run association test** - GWAS on filtered data
  - Added: 2026-01-16
```

**notebook/DONE.md:**
```markdown
# Completed

- [x] #1 **Download data** - Get 1000G reference panel
  - Added: 2026-01-14
  - Completed: 2026-01-14

- [x] #2 **QC samples** - Remove low quality samples
  - Added: 2026-01-15
  - Completed: 2026-01-15
```

**Prompt:**
```
Add a todo to visualize Manhattan plot.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The new todo should be numbered #4 (after #1, #2 in DONE.md and #3 in TODO.md).

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     DONE.md content:
     {{file_content:notebook/DONE.md}}

     Does the new todo item have #4 in it (not reusing #1, #2, or #3)?
   - `pass_if`: yes

**Why this should pass reliably:** CLAUDE.md explicitly says to check both files.

---

## Test 3.5: Show Todos

**What we're testing:** "Show my todos" reads and displays TODO.md content.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

- [ ] #1 **Calculate LD scores** - For EUR population
  - Added: 2026-01-15

- [ ] #2 **Run LDSC** - Estimate heritability
  - Added: 2026-01-16
```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Show my todos.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `TODO.md`
   - Reasoning: Should read the todo file

2. **output_contains**
   - `pattern`: `LD scores|LDSC`
   - `case_insensitive`: true
   - Reasoning: Should display todo content

3. **output_contains**
   - `pattern`: `#1.*#2|#2.*#1`
   - Reasoning: Should show both items

**Why this should pass reliably:** Direct request to display file contents.

---

## Test 3.6: Todo with Context Link

**What we're testing:** Todo arising from analysis discussion includes Context link.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
├── TODO.md
├── DONE.md
└── entries/
    └── 2026-01-15-prs-validation.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-15 | prs-validation | Validated PRS in independent cohort, r2=0.05 |
```

**notebook/entries/2026-01-15-prs-validation.md:**
```markdown
# PRS Validation

**Date:** 2026-01-15

## Summary
Validated height PRS in holdout sample. Variance explained r2=0.05, lower than expected.

## Details
Need to investigate: possibly insufficient sample size or LD mismatch.

## References
None
```

**notebook/TODO.md:**
```markdown
# To-Do

```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Look at the PRS validation entry. Add a todo to investigate the low r2 - check for LD panel mismatch.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `prs-validation`
   - Reasoning: Should read the referenced entry

2. **evaluator**
   - `prompt`: |
     The todo should include a Context line linking to the prs-validation entry.

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     Does the new todo have a "Context:" line mentioning "prs-validation"?
   - `pass_if`: yes

**Why this should pass reliably:** Todo explicitly arises from reading entry. CLAUDE.md says to add Context link in this case.

---

## Test 3.7: Complete Todo Moves to DONE.md

**What we're testing:** Completing a todo moves it from TODO.md to DONE.md.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── TODO.md
└── DONE.md
```

**notebook/TODO.md:**
```markdown
# To-Do

- [ ] #1 **Count variants** - Count variants passing QC filters
  - Added: 2026-01-15
```

**notebook/DONE.md:**
```markdown
# Completed

```

**Prompt:**
```
Work on todo #1. The answer is: 5.2 million variants passed QC. Mark it complete.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     After completion:
     - TODO.md should NOT contain the #1 item
     - DONE.md should contain the #1 item with [x] checkbox

     TODO.md content:
     {{file_content:notebook/TODO.md}}

     DONE.md content:
     {{file_content:notebook/DONE.md}}

     Is the #1 item now in DONE.md (not TODO.md)?
   - `pass_if`: yes

2. **evaluator**
   - `prompt`: |
     The completed item should have a Completed: date line.

     DONE.md content:
     {{file_content:notebook/DONE.md}}

     Does the completed item have a "Completed:" line with a date?
   - `pass_if`: yes

**Why this should pass reliably:** Clear instruction to complete. Format specified in CLAUDE.md.
