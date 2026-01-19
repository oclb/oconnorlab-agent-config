# Memory Retrieval Tests

Tests that verify Claude reads the notebook INDEX.md and retrieves relevant entries when appropriate.

---

## Test 1.1: Index Read on Ambiguous Reference

**What we're testing:** When user references past work with a pronoun ("the analysis"), Claude should read INDEX.md to find relevant entries.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-10-gwas-power-analysis.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-10 | gwas-power-analysis | Power calculations for case-control GWAS with 10K samples |
```

**notebook/entries/2026-01-10-gwas-power-analysis.md:**
```markdown
# GWAS Power Analysis

**Date:** 2026-01-10

## Summary
Calculated statistical power for detecting variants with OR=1.2 in a GWAS with 5K cases and 5K controls. Power is ~80% for MAF>0.1.

## Details
Used GPC calculator. Key parameters:
- Disease prevalence: 1%
- Significance threshold: 5e-8
- Sample size: 10,000 (5K/5K)

## References
None
```

**Prompt:**
```
What were the key findings from the power analysis?
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `INDEX.md`
   - Reasoning: Must read index to find which entry is relevant

2. **tool_used: Read**
   - `target_pattern`: `gwas-power-analysis`
   - Reasoning: Should read the actual entry to answer the question

3. **output_contains**
   - `pattern`: `80%|OR.*1\.2|MAF.*0\.1`
   - `case_insensitive`: true
   - Reasoning: Should mention key findings from the entry

**Why this should pass reliably:** The question is unambiguous - there's only one entry, and "the power analysis" clearly refers to it. Haiku can handle this.

---

## Test 1.2: No Retrieval for Unrelated New Task

**What we're testing:** Claude should NOT read notebook entries for completely new, unrelated work that doesn't reference past context.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-10-gwas-power-analysis.md
```

(Same INDEX.md and entry as Test 1.1)

**Prompt:**
```
What is 2 + 2?
```

**Assertions:**

1. **tool_not_used: Read**
   - `target_pattern`: `entries/.*\.md`
   - Reasoning: Simple arithmetic has nothing to do with past work

2. **output_contains**
   - `pattern`: `4`
   - Reasoning: Should still answer correctly

**Why this should pass reliably:** Trivial question with no connection to genetics work. No reasonable model would retrieve notebook entries for this.

---

## Test 1.3: Retrieval on Explicit Entry Name

**What we're testing:** When user explicitly names an entry, Claude reads it.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    ├── 2026-01-10-ld-score-regression.md
    └── 2026-01-12-fine-mapping-susie.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-12 | fine-mapping-susie | SuSiE fine-mapping of height GWAS loci |
| 2026-01-10 | ld-score-regression | LDSC heritability estimate for height |
```

**notebook/entries/2026-01-10-ld-score-regression.md:**
```markdown
# LD Score Regression Analysis

**Date:** 2026-01-10

## Summary
Ran LDSC on height GWAS summary statistics. Estimated h2=0.45 (SE=0.03).

## Details
Used 1000G EUR LD scores. Intercept 1.02 suggests minimal inflation.

## References
None
```

**notebook/entries/2026-01-12-fine-mapping-susie.md:**
```markdown
# SuSiE Fine-Mapping

**Date:** 2026-01-12

## Summary
Fine-mapped 50 height GWAS loci using SuSiE. Identified 73 credible sets.

## Details
Used UK Biobank LD reference. Default L=10 causal variants per locus.

## References
- `ld-score-regression`: Used same LD reference panel
```

**Prompt:**
```
Look at the ld-score-regression entry and tell me the heritability estimate.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `ld-score-regression`
   - Reasoning: User explicitly named this entry

2. **tool_not_used: Read**
   - `target_pattern`: `fine-mapping`
   - Reasoning: User didn't ask about fine-mapping

3. **output_contains**
   - `pattern`: `0\.45|45%`
   - Reasoning: Should report the h2 estimate

**Why this should pass reliably:** Explicit entry name leaves no ambiguity. Even haiku will read the right file.

---

## Test 1.4: Retrieval on Date Reference

**What we're testing:** When user references a date that appears in INDEX.md, Claude reads relevant entries.

**Model:** haiku

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    ├── 2026-01-10-sample-qc.md
    └── 2026-01-15-pca-ancestry.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-15 | pca-ancestry | PCA for ancestry inference, identified 3 clusters |
| 2026-01-10 | sample-qc | Sample QC, removed 50 samples for missingness |
```

**notebook/entries/2026-01-15-pca-ancestry.md:**
```markdown
# PCA Ancestry Analysis

**Date:** 2026-01-15

## Summary
Ran PCA on genotype data. Identified 3 ancestry clusters (EUR, AFR, AMR).

## Details
Used PLINK --pca with 10 components. PC1 separates AFR from EUR/AMR.

## References
- `sample-qc`: Used QC'd sample set
```

**notebook/entries/2026-01-10-sample-qc.md:**
```markdown
# Sample QC

**Date:** 2026-01-10

## Summary
QC on 5,000 samples. Removed 50 for >5% missingness. Final N=4,950.

## Details
Standard GWAS QC: missingness, heterozygosity, sex check.

## References
None
```

**Prompt:**
```
What did we do on January 15th?
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `INDEX.md`
   - Reasoning: Need to look up what happened on that date

2. **tool_used: Read**
   - `target_pattern`: `pca-ancestry`
   - Reasoning: That's the entry from January 15th

3. **output_contains**
   - `pattern`: `PCA|ancestry|cluster`
   - `case_insensitive`: true
   - Reasoning: Should describe the PCA work

**Why this should pass reliably:** Date is unambiguous, only one entry matches.

---

## Test 1.5: Retrieval on "Continue" Keyword

**What we're testing:** "Continue the analysis" should trigger retrieval of most recent relevant entry.

**Model:** sonnet (requires more reasoning about what to continue)

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    └── 2026-01-17-variant-annotation.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-17 | variant-annotation | Annotating GWAS hits with VEP, in progress |
```

**notebook/entries/2026-01-17-variant-annotation.md:**
```markdown
# Variant Annotation with VEP

**Date:** 2026-01-17
**Status:** In Progress

## Summary
Annotating 500 GWAS significant variants with VEP. Completed: downloaded VEP cache. Next: run annotation.

## Details
Using Ensembl VEP v110 with GRCh38. Cache downloaded to data/vep_cache/.

## References
None
```

**Prompt:**
```
Continue the annotation work.
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `INDEX.md`
   - Reasoning: Need to find what's in progress

2. **tool_used: Read**
   - `target_pattern`: `variant-annotation`
   - Reasoning: Should read the in-progress entry

3. **output_contains**
   - `pattern`: `VEP|annotation|500`
   - `case_insensitive`: true
   - Reasoning: Should reference the annotation task

**Why this should pass reliably:** "Continue" clearly signals resuming prior work. Single in-progress entry makes it unambiguous.

---

## Test 1.6: Multiple Entries - Select Correct One

**What we're testing:** When multiple entries exist, Claude selects the most relevant based on the question.

**Model:** sonnet (requires semantic matching)

**Config flags:** default

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    ├── 2026-01-10-gwas-qc.md
    ├── 2026-01-12-heritability.md
    └── 2026-01-14-mendelian-randomization.md
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-14 | mendelian-randomization | MR analysis of BMI on T2D risk |
| 2026-01-12 | heritability | LDSC h2 estimates for metabolic traits |
| 2026-01-10 | gwas-qc | Quality control for UK Biobank GWAS |
```

**notebook/entries/2026-01-10-gwas-qc.md:**
```markdown
# GWAS Quality Control

**Date:** 2026-01-10

## Summary
QC on UKBB GWAS summary stats. Removed 1M variants with INFO<0.8.

## Details
Standard filters: INFO, MAF, HWE.

## References
None
```

**notebook/entries/2026-01-12-heritability.md:**
```markdown
# Heritability Estimation

**Date:** 2026-01-12

## Summary
LDSC heritability for BMI (h2=0.25), T2D (h2=0.18), fasting glucose (h2=0.12).

## Details
Used EUR LD scores. All estimates significant (p<1e-10).

## References
- `gwas-qc`: Used QC'd summary stats
```

**notebook/entries/2026-01-14-mendelian-randomization.md:**
```markdown
# Mendelian Randomization: BMI → T2D

**Date:** 2026-01-14

## Summary
Two-sample MR shows BMI causally increases T2D risk. OR=1.8 per SD BMI (p=3e-15).

## Details
Used 97 BMI instruments from GIANT. IVW estimate robust to pleiotropy (MR-Egger intercept p=0.4).

## References
- `heritability`: Confirmed genetic signal in both traits
```

**Prompt:**
```
What was the causal effect estimate from our MR analysis?
```

**Assertions:**

1. **tool_used: Read**
   - `target_pattern`: `mendelian-randomization`
   - Reasoning: Question is about MR results

2. **tool_not_used: Read**
   - `target_pattern`: `heritability`
   - Reasoning: Not asking about heritability

3. **output_contains**
   - `pattern`: `OR.*1\.8|1\.8.*OR|causal`
   - `case_insensitive`: true
   - Reasoning: Should report the OR

**Why this should pass reliably:** "MR analysis" clearly maps to the mendelian-randomization entry. Sonnet handles this semantic matching well.
