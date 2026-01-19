# Memory Creation Tests

Tests that verify Claude creates notebook entries with correct format and structure.

---

## Test 2.1: Entry Created for Substantive Work

**What we're testing:** Analysis work creates a notebook entry with correct format.

**Model:** sonnet

**Config flags:** `AFK=true` (so it proceeds without asking questions)

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── expression.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/expression.csv:**
```csv
gene,sample1,sample2,sample3,sample4
BRCA1,5.2,4.8,5.1,4.9
TP53,8.1,7.9,8.3,8.0
EGFR,3.2,3.5,3.1,3.4
MYC,6.7,6.5,6.9,6.6
```

**Prompt:**
```
Analyze the expression data in data/expression.csv. Calculate the mean expression for each gene.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - `content_pattern`: `## Summary`
   - Reasoning: Entry should have Summary section

2. **file_created**
   - `path`: `notebook/entries/*.md`
   - `content_pattern`: `## Details`
   - Reasoning: Entry should have Details section

3. **output_contains**
   - `pattern`: `BRCA1.*5\.0|TP53.*8\.1|mean`
   - `case_insensitive`: true
   - Reasoning: Should report mean values

4. **output_contains**
   - `pattern`: `[Cc]reated notebook entry`
   - Reasoning: Should announce entry creation

**Why this should pass reliably:** Simple analysis with clear output. AFK mode ensures it proceeds. Entry format is well-specified in the skill.

---

## Test 2.2: Entry Date Format Correct

**What we're testing:** Entry filename uses YYYY-MM-DD format.

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
Create a simple notebook entry documenting that we decided to use Python 3.11 for this project.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/2026-01-*.md`
   - Reasoning: Filename should start with today's date

2. **evaluator**
   - `prompt`: |
     Check if the filename follows the pattern YYYY-MM-DD-slug.md:

     Files created:
     {{files_created}}

     Does the entry filename match the pattern YYYY-MM-DD-something.md?
   - `pass_if`: yes
   - Reasoning: Validate date format

**Why this should pass reliably:** Date format is explicitly specified in CLAUDE.md. Simple compliance check.

---

## Test 2.3: Entry Contains Date Header

**What we're testing:** Entry content includes **Date:** header with correct format.

**Model:** haiku

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**Prompt:**
```
Document that we've set up the conda environment for GWAS analysis.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - `content_pattern`: `\*\*Date:\*\* 2026-01-\d{2}`
   - Reasoning: Should have Date header with correct format

**Why this should pass reliably:** Explicit template requirement in CLAUDE.md.

---

## Test 2.4: INDEX.md Updated on Entry Creation

**What we're testing:** When entry is created, INDEX.md gets a new row.

**Model:** sonnet

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
Calculate the allele frequency of rs123456 given these genotype counts: AA=100, AG=50, GG=10.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Entry should be created

2. **tool_used: Edit**
   - `target_pattern`: `INDEX.md`
   - Reasoning: INDEX.md should be edited

3. **evaluator**
   - `prompt`: |
     The INDEX.md file should have a new row added to the table.

     Original INDEX.md:
     ```
     | Date | Name | Summary |
     |------|------|---------|
     ```

     Final INDEX.md content:
     {{file_content:notebook/INDEX.md}}

     Does the final INDEX.md have at least one data row in the table (not just the header)?
   - `pass_if`: yes

**Why this should pass reliably:** INDEX.md update is explicitly required alongside entry creation.

---

## Test 2.5: References Section Links Prior Work

**What we're testing:** When analysis builds on prior entry, References section links to it.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-10-sample-qc.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-10 | sample-qc | QC on 1000 samples, removed 23 for low call rate |
```

**notebook/entries/2026-01-10-sample-qc.md:**
```markdown
# Sample Quality Control

**Date:** 2026-01-10

## Summary
Performed QC on 1000 samples. Removed 23 samples with call rate <95%.

## Details
Final sample set: 977 samples in data/samples_qc.txt

## References
None
```

**Prompt:**
```
Now that QC is done, calculate the minor allele frequency for each variant in the QC'd samples.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `sample-qc`
   - Reasoning: Should read prior QC entry to understand context

2. **file_created**
   - `path`: `notebook/entries/*maf*.md` OR `notebook/entries/*freq*.md`
   - Reasoning: New entry for MAF calculation

3. **evaluator**
   - `prompt`: |
     The new notebook entry should reference the prior sample-qc work in its References section.

     New entry content:
     {{file_content:notebook/entries/2026-01-*-*.md}}

     Does the new entry have a References section that mentions "sample-qc"?
   - `pass_if`: yes

**Why this should pass reliably:** Skill explicitly instructs to list referenced entries. Clear dependency makes the link obvious.

---

## Test 2.6: No Entry for Trivial Task

**What we're testing:** Trivial questions don't create notebook entries.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**Prompt:**
```
What does MAF stand for in genetics?
```

**Assertions:**

1. **file_not_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Simple definition question shouldn't create entry

2. **output_contains**
   - `pattern`: `[Mm]inor [Aa]llele [Ff]requency`
   - Reasoning: Should still answer correctly

**Why this should pass reliably:** CLAUDE.md explicitly says entries are for substantive work, not simple questions.

---

## Test 2.7: Entry Announcement Format

**What we're testing:** Claude announces entry creation with correct format: "Created notebook entry: `name`"

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**Prompt:**
```
Document that we've decided to use PLINK 2.0 for all GWAS analyses in this project.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `[Cc]reated notebook entry:?\s*`
   - Reasoning: Should announce creation

2. **evaluator**
   - `prompt`: |
     The output should announce the entry creation in the format:
     "Created notebook entry: `entry-name`" (with backticks around the name)

     Output:
     {{output}}

     Does the output contain this announcement format with the entry name in backticks?
   - `pass_if`: yes

**Why this should pass reliably:** Explicit format requirement in CLAUDE.md.

---

## Test 2.8: Descriptive Entry Naming

**What we're testing:** Entry name is descriptive, not generic.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**Prompt:**
```
Perform a chi-square test comparing genotype frequencies between cases and controls for variant rs7903146.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The notebook entry name should be specific and descriptive, not generic.

     BAD names (too generic):
     - "analysis"
     - "chi-square-test"
     - "genotype-analysis"

     GOOD names (specific):
     - "rs7903146-genotype-chisq"
     - "chisq-rs7903146-case-control"
     - "genotype-freq-rs7903146"

     Files created:
     {{files_created}}

     Is the entry name specific (mentions rs7903146 or similar identifying detail)?
   - `pass_if`: yes

**Why this should pass reliably:** Skill has explicit examples of good vs bad naming.
