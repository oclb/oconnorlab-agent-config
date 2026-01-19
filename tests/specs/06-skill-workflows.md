# Skill Workflow Compliance Tests

Tests that verify skills follow their specified workflows correctly.

---

## Test 6.1: perform-analysis - Reads Related Entries (Step 0)

**What we're testing:** Before analysis, Claude checks for related prior work.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-10-gwas-qc.md

data/
└── gwas_results.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-10 | gwas-qc | QC on GWAS summary stats, removed variants with INFO<0.8 |
```

**notebook/entries/2026-01-10-gwas-qc.md:**
```markdown
# GWAS Quality Control

**Date:** 2026-01-10

## Summary
QC on GWAS summary statistics. Filtered variants with INFO < 0.8. Final set: 8M variants.

## Details
Input: 12M variants
Removed: 4M low-INFO variants
Output: data/gwas_qc.csv

## References
None
```

**data/gwas_results.csv:**
```csv
variant,pvalue,beta
rs1,1e-8,0.15
rs2,1e-6,0.12
rs3,1e-10,0.18
```

**Prompt:**
```
Analyze the GWAS results in data/gwas_results.csv. Identify genome-wide significant hits.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `INDEX.md`
   - Reasoning: Step 0 says to check index for related work

2. **tool_used: Read**
   - `target_pattern`: `gwas-qc`
   - Reasoning: Related GWAS entry should be read

3. **evaluator**
   - `prompt`: |
     The new notebook entry should reference the prior gwas-qc work.

     New entry content:
     {{file_content:notebook/entries/2026-01-*-*.md}}

     Does the new entry's References section mention "gwas-qc"?
   - `pass_if`: yes

**Why this should pass reliably:** Clear relationship between QC and analysis. Skill explicitly requires retrieval.

---

## Test 6.2: perform-analysis - Creates Entry with Required Sections

**What we're testing:** Analysis entry has all required sections.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── counts.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/counts.csv:**
```csv
gene,count
BRCA1,150
TP53,320
EGFR,85
```

**Prompt:**
```
Analyze the gene counts in data/counts.csv. Which gene has the highest expression?
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The notebook entry should have these sections:
     - Summary (or Motivation)
     - Details (or Plan, Execution Notes, Findings)

     Entry content:
     {{file_content:notebook/entries/*.md}}

     Does the entry have at least Summary and Details/Findings sections?
   - `pass_if`: yes

2. **output_contains**
   - `pattern`: `TP53|highest|320`
   - Reasoning: TP53 has highest count

**Why this should pass reliably:** Simple analysis. Entry format is well-specified.

---

## Test 6.3: perform-analysis - Key Takeaway

**What we're testing:** Analysis includes a clear key takeaway.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── test_results.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/test_results.csv:**
```csv
group,value
control,5.2
control,4.8
control,5.1
treatment,8.3
treatment,7.9
treatment,8.1
```

**Prompt:**
```
Test if treatment significantly differs from control in data/test_results.csv.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The output should include a clear key finding/takeaway with actual numbers.

     Good: "Treatment increased values by 60% (p < 0.01)"
     Bad: "The analysis showed interesting results"

     Output:
     {{output}}

     Does the output state a specific key finding with numbers?
   - `pass_if`: yes

2. **output_contains**
   - `pattern`: `p\s*[<=<]\s*0\.\d|significant|differ`
   - `case_insensitive`: true
   - Reasoning: Should report statistical result

**Why this should pass reliably:** Clear difference in data (control ~5, treatment ~8). Skill requires key takeaway.

---

## Test 6.4: new-data - Reports Dimensions

**What we're testing:** Data validation reports dataset dimensions.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
data/
└── samples.csv
```

**data/samples.csv:**
```csv
id,age,bmi,status
1,45,25.2,case
2,32,22.1,control
3,58,28.5,case
4,41,24.3,control
5,36,23.8,case
```

**Prompt:**
```
Check the data in data/samples.csv.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `5.*row|row.*5|5.*sample|sample.*5`
   - `case_insensitive`: true
   - Reasoning: Should report 5 rows

2. **output_contains**
   - `pattern`: `4.*col|col.*4|4.*variable|variable.*4`
   - `case_insensitive`: true
   - Reasoning: Should report 4 columns

**Why this should pass reliably:** Dimension reporting is explicitly required in new-data skill.

---

## Test 6.5: new-data - Identifies Missing Values

**What we're testing:** Validation identifies missing values.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
data/
└── incomplete.csv
```

**data/incomplete.csv:**
```csv
sample,phenotype,covariate
A,1.5,0.8
B,,0.6
C,2.3,
D,1.8,0.9
E,NA,0.7
```

**Prompt:**
```
Validate data/incomplete.csv for quality issues.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `missing|NA|empty|null|blank`
   - `case_insensitive`: true
   - Reasoning: Should identify missing values

2. **output_contains**
   - `pattern`: `phenotype|covariate|column`
   - `case_insensitive`: true
   - Reasoning: Should indicate which columns have issues

**Why this should pass reliably:** Clear missing values. Skill requires missing value check.

---

## Test 6.6: new-data - Creates Dataset Entry

**What we're testing:** Data validation creates notebook entry documenting the dataset.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── genotypes.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/genotypes.csv:**
```csv
variant,sample1,sample2,sample3
rs123,0,1,2
rs456,1,1,0
rs789,2,0,1
```

**Prompt:**
```
Validate the genotype data in data/genotypes.csv before GWAS.
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: new-data creates dataset entries

2. **evaluator**
   - `prompt`: |
     The notebook entry should document the dataset:
     - Location/path
     - Dimensions
     - Data type or format
     - Any notes/issues

     Entry content:
     {{file_content:notebook/entries/*.md}}

     Does the entry document the dataset (location, dimensions, format)?
   - `pass_if`: yes

**Why this should pass reliably:** Skill explicitly creates dataset documentation.

---

## Test 6.7: teaching-mode - Explains Decisions

**What we're testing:** Teaching mode explains why choices were made.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
Teach me how to choose between a t-test and a Mann-Whitney U test.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `normal|distribution|parametric|non-parametric|assumption`
   - `case_insensitive`: true
   - Reasoning: Should explain the decision criteria

2. **evaluator**
   - `prompt`: |
     A teaching response about test selection should explain:
     - When to use t-test (assumptions)
     - When to use Mann-Whitney (assumptions)
     - How to decide between them

     Output:
     {{output}}

     Does the response explain the decision-making process (not just list the tests)?
   - `pass_if`: yes

**Why this should pass reliably:** Teaching mode focuses on "why" and decision points.

---

## Test 6.8: teaching-mode - Provides Replication Steps

**What we're testing:** Teaching mode includes how to replicate.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
Teach me how to calculate Hardy-Weinberg equilibrium p-value for a variant.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `step|first|then|calculate|formula`
   - `case_insensitive`: true
   - Reasoning: Should give steps

2. **evaluator**
   - `prompt`: |
     A teaching response should include replication instructions:
     - Specific steps or commands
     - Formula or calculation method
     - Example with numbers

     Output:
     {{output}}

     Does the response include enough detail to replicate the calculation?
   - `pass_if`: yes

**Why this should pass reliably:** Teaching skill requires "HOW TO REPLICATE" content.
