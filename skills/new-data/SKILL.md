---
name: new-data
description: This skill should be used when the user asks to "download a dataset", "access new data", "check this data", "validate data", "sanity check data", "explore a dataset", or when working with unfamiliar data for the first time. Also triggers when simulating or generating new data that needs validation.
version: 1.0.0
---

# Sanity Check Data Skill

Systematic framework for acquiring, validating, and exploring datasets to ensure they are suitable for analysis.

## Notebook Integration

This skill writes to `notebook/methods/` to track datasets used in the project (as `type: data` entries).

**Before starting:** Check if dataset is already documented:
```bash
ls notebook/methods/ 2>/dev/null | grep -i "<dataset-name>"
```

If an entry exists, read it to see what's already known. You may just need to update it rather than start fresh.

## AFK Mode Behavior

At the start, check `~/.claude/behavior.conf` for the `AFK` flag.

**When AFK=true:**
- Proceed directly with domain-appropriate validation checks without asking
- Auto-select tools based on file type (pandas for CSV, bcftools for VCF, etc.)
- Document tool selection and reasoning in the report
- Respect current tool permissions (sandbox settings) - only use allowed tools
- Attempt autonomous troubleshooting on errors (max 2 attempts), then stop and report
- Only pause for: inaccessible files, ambiguous data source, critical format issues

**When AFK=false (default):**
- Ask which tool the user plans to use if multiple options exist
- Confirm before downloading large files
- Ask about specific validation checks if domain is unclear

## When This Skill Applies

Use when:
- Downloading a new dataset from a URL or repository
- Accessing data for the first time
- Receiving or generating simulated data
- User explicitly asks to validate or sanity-check data
- The **perform-analysis** skill encounters unfamiliar data
- Need to verify data format or understand contents

## Data Validation Process

Follow these 8 steps systematically:

### Step 1: Acquire or Locate the Data

**Determine how to access the dataset.**

**For downloaded data:**
1. Identify source (GEO, TCGA, Kaggle, direct URL, API)
2. Download: `wget URL -O data/filename` or `curl -L URL -o data/filename`
3. Verify: file exists, non-empty, reasonable size

**For simulated data:**
1. Understand simulation parameters
2. Generate with reproducible random seed
3. Document parameters in filename or metadata

**For local data:**
1. Locate file (ask user or search common locations)
2. Verify accessibility: `ls -lh path/to/file`

**Document source:**
```
Data Source:
- Location: data/experiment.csv
- Source: [Downloaded from GEO GSE12345 / Simulated / Provided by user]
- Date acquired: [date]
```

### Step 2: Examine the Format

**Understand structure and format.**

**Common formats:**
- Tabular: CSV, TSV, Excel, Parquet
- Biological: VCF, BED, BAM, FASTQ, GFF
- Hierarchical: JSON, XML, HDF5
- Statistical: RDS, RData, .mat, .sav

**Check:**
```bash
file data.csv          # File type
head -10 data.csv      # First lines
wc -l data.csv         # Line count
file -i data.csv       # Encoding
```

**Key questions:** Header row? Delimiter? Row names? Compression? Encoding?

### Step 3: Load Data with Appropriate Tool

**Choose tool based on format:**

| Format | Tool |
|--------|------|
| CSV/TSV | pandas, readr |
| VCF | bcftools, pysam |
| BAM | samtools, pysam |
| BED | bedtools, pybedtools |
| JSON | json module, jsonlite |
| HDF5 | h5py, rhdf5 |

**If tool unavailable:** Use the **learn-tool** skill to install it.

**Load and verify:**
```python
import pandas as pd
df = pd.read_csv('data.csv', index_col=0)
print(f"Loaded: {df.shape}")
print(df.dtypes)
print(df.head())
```

### Step 4: Compute Basic Statistics

**Characterize the data:**

```
Data Statistics:

Dimensions: 15,000 rows × 48 columns
Row names: Gene symbols (BRCA1, TP53, ...)
Column names: Sample IDs (Sample_001, ...)

Data types: Numeric (float64)
Missing: 1,500 cells (0.2%)

Value range: [0.0, 15.7]
Mean: 5.2, Median: 5.1
```

**Check:** Dimensions, identifiers, data types, missing values, value ranges.

### Step 5: Perform Sanity Checks

**Automatic checks (all datasets):**
- No empty rows/columns
- Reasonable dimensions
- Unique identifiers
- No infinite/NaN values
- Consistent data types

**Domain-specific checks:** See [references/domain-checks.md](references/domain-checks.md) for expression data, clinical data, VCF, FASTQ validation patterns.

**Report findings:**
```
Sanity Checks:
✓ No empty rows or columns
✓ All identifiers unique
✓ Values in expected range
⚠ 5 columns have >10% missing
```

### Step 6: Provide Visual Summary (Optional)

Create visualizations only if they add value:
- Distribution plots for numeric data
- Missing data heatmaps
- Correlation structure

### Step 7: Generate Data Report

**Provide comprehensive summary:**

```
✓ DATA VALIDATION COMPLETE

Summary: [One sentence describing dataset]

Key findings:
- Dimension: [N × M]
- Type: [Data type]
- Quality: [Good/Fair/Poor]
- Issues: [None / List]

Recommendation: [Ready for analysis / Needs preprocessing / Has critical issues]

Tool compatibility: ✓ Compatible with [tool name]

Files created:
- [List any reports or visualizations]
```

**In AFK mode:** Include a "Choices Made" section documenting autonomous decisions.

### Step 8: Confirm Tool Compatibility

**If a specific analysis tool is expected, verify compatibility:**

```R
# Example: Test DESeq2 compatibility
if (!all(counts >= 0)) print("ERROR: DESeq2 requires non-negative counts")
```

See [references/domain-checks.md](references/domain-checks.md) for tool-specific compatibility tests.

### Step 9: Write to Notebook

**Create or update `notebook/methods/YYYY-MM-DD-<dataset-name>.md`:**

```markdown
# <Dataset Name>

**Date:** YYYY-MM-DD
**Type:** data
**Commit:** N/A

## Summary
[One sentence: what this dataset is and where it came from]

## Details
- **Location:** `data/experiment.csv` (or full path)
- **Source:** [Downloaded from GEO GSE12345 / Simulated / Provided by user]
- **URL:** [if downloaded]
- **Dimensions:** N rows × M columns
- **Row identifiers:** [Gene symbols, sample IDs, etc.]
- **Column identifiers:** [Sample IDs, features, etc.]
- **Data type:** [Counts, continuous, categorical, etc.]
- **File format:** [CSV, VCF, BAM, etc.]
- **Quality:** [Good / Fair / Poor]

## Notes
[Any issues, gotchas, or things to remember]
- 5 columns have >10% missing values
- Column "age" contains some negative values (likely data entry errors)
- Gene IDs use older HGNC symbols
```

**Update notebook/INDEX.md:**
Add a row to the Methods table:
```
| YYYY-MM-DD | data | <dataset-name>: <brief description> |
```

**Commit the notebook entry:**
```bash
mkdir -p notebook/methods
git add notebook/methods/YYYY-MM-DD-<dataset-name>.md notebook/INDEX.md
git commit -m "methods: document <dataset-name> dataset"
```

## Best Practices

1. **Document the source** - Track origin for reproducibility
2. **Be thorough but efficient** - Use command-line for large files
3. **Validate before analyzing** - Catch issues early
4. **Provide actionable recommendations** - Don't just identify issues
5. **Save validation reports** - Create audit trail

## Integration with Other Skills

- **perform-analysis**: Invokes this skill when encountering unfamiliar data
- **learn-tool**: Use when a required tool is not available

## References

- **[references/domain-checks.md](references/domain-checks.md)**: Domain-specific validation (expression, clinical, VCF, FASTQ) and tool compatibility tests
- **[references/examples.md](references/examples.md)**: Complete example workflow, report template, large file handling
