# Sanity Check Data - Quick Reference

## The 8-Step Validation Framework

When you ask Claude to download, access, or validate data, it systematically follows these steps:

### 1️⃣ Acquire or Locate Data
**What**: Download or find the dataset
**Actions**:
- Download from URL/repository
- Generate simulated data
- Locate existing local files
**Output**: Data source and location confirmed

### 2️⃣ Examine Format
**What**: Understand file structure and encoding
**Actions**:
- Identify file type (CSV, VCF, BAM, JSON, etc.)
- Check delimiter, headers, compression
- Inspect first/last lines
- Determine encoding
**Output**: Format specification documented

### 3️⃣ Load with Appropriate Tool
**What**: Read the data successfully
**Actions**:
- Choose correct tool (pandas, bcftools, samtools, etc.)
- Install tool if needed (via learn-tool skill)
- Attempt to load data
- Handle errors (encoding, delimiters, etc.)
**Output**: Data successfully loaded into memory/tool

### 4️⃣ Compute Basic Statistics
**What**: Characterize size and content
**Metrics**:
- Dimensions (rows × columns)
- Row/column names and patterns
- Data types (numeric, categorical, text)
- Missing data count and percentage
- Value ranges (min, max, mean, median)
**Output**: Statistical summary

### 5️⃣ Perform Sanity Checks
**What**: Verify data makes sense
**Automatic checks**:
- No empty rows/columns
- Unique identifiers
- No infinite/NaN values
- Reasonable dimensions
- Consistent data types

**Domain-specific checks**:
- Expression: Non-negative values, reasonable ranges
- Clinical: Valid ages, categorical levels
- Genomic: Valid coordinates, alleles, quality scores
- Sequence: Read counts, quality encoding

**Output**: Pass/fail for each check + warnings

### 6️⃣ Visual Summary (Optional)
**What**: Create informative plots
**Common plots**:
- Distribution histograms
- Missing data heatmaps
- Sample/feature correlations
- Quality metrics
**Output**: Figure files (if created)

### 7️⃣ Generate Report
**What**: Comprehensive validation summary
**Includes**:
- Source and format details
- Statistical summary
- Sanity check results
- Quality assessment (Good/Fair/Poor)
- Issues identified
- Recommendations for next steps
**Output**: Validation report (text/markdown)

### 8️⃣ Verify Tool Compatibility
**What**: Ensure analysis tools can read the data
**Actions**:
- Identify expected analysis tools
- Test tool can load data
- Verify format requirements
- Report compatibility status
**Output**: Tool compatibility confirmation

## Trigger Phrases

Use these to activate the skill:
- "Download dataset from..."
- "Check this data file"
- "Validate the data in..."
- "Sanity check the expression data"
- "Explore what's in this dataset"
- "Access data from GEO/TCGA/..."
- "Simulate data and verify..."

## Automatic Invocation

The skill is also invoked automatically by **perform-analysis** when encountering unfamiliar data.

## Supported Data Formats

### Tabular
- CSV, TSV, Excel (.xlsx)
- Parquet, Feather
- Delimited text (any delimiter)

### Biological
- **Variants**: VCF (with bcftools)
- **Alignments**: BAM, SAM (with samtools)
- **Sequences**: FASTQ, FASTA (with seqtk)
- **Intervals**: BED, GFF, GTF (with bedtools)

### Hierarchical
- JSON (with jq or Python/R)
- XML
- HDF5 (with h5py/rhdf5)
- NetCDF

### Statistical
- RDS, RData (R)
- .mat (MATLAB)
- .sav (SPSS)

### Other
- Compressed files (gzip, bzip2, zip)
- Database dumps (SQLite)

## Domain-Specific Validations

### Expression Data (RNA-seq/Microarray)
✓ Non-negative values (unless normalized)
✓ Reasonable range (raw counts: 0-1M, log: 0-20)
✓ Consistent across samples
✓ Format for DESeq2/edgeR/limma

### Clinical/Phenotype Data
✓ Ages 0-120
✓ Categorical variables have reasonable levels
✓ Dates are valid
✓ Patient IDs are unique

### Genomic Variants (VCF)
✓ Valid chromosome names
✓ Positive positions
✓ Valid ref/alt alleles
✓ Quality scores present
✓ Genotype completeness

### Sequence Data (FASTQ)
✓ Read count (lines/4)
✓ Quality encoding (Phred+33)
✓ Read length distribution
✓ Quality score range

## Example Sessions

### Download Public Data
```
You: Download expression data from GEO GSE12345

1. Acquire: wget from GEO FTP → 23.4 MB
2. Format: Tab-delimited, gzipped
3. Load: pandas → 54,675 × 12
4. Stats: Log2-transformed, 0% missing
5. Checks: ✓ All pass
6. Visual: Distribution plot
7. Report: Quality=Excellent
8. Tools: ✓ Compatible with limma
```

### Validate Local File
```
You: Check experiment_results.csv

1. Locate: data/experiment_results.csv → 2.3 MB
2. Format: CSV, comma-delimited
3. Load: pandas → 1,000 × 50
4. Stats: 0.47% missing, mixed types
5. Checks: ⚠ Age >120, measurement_3 45% missing
6. Visual: Missing data heatmap
7. Report: Quality=Fair, issues noted
8. Tools: ✓ Compatible
```

### Simulate and Validate
```
You: Simulate 1000 normal samples and validate

1. Generate: N(100,15), n=1000, seed=42
2. Format: CSV, saved
3. Load: Numpy array
4. Stats: Mean=99.8, SD=15.2
5. Checks: ✓ Matches theory, normality test pass
6. Visual: Histogram with theory overlay
7. Report: Properties match expected
8. Tools: Standard format
```

## Common Issues Detected

| Issue | What's Checked | Action |
|-------|----------------|--------|
| Missing values | % per column/row | Impute or exclude |
| Outliers | Range, z-scores | Investigate |
| Wrong format | Parse errors | Convert |
| Duplicates | ID uniqueness | Deduplicate |
| Wrong scale | Value ranges | Transform |
| Empty columns | All-NA check | Remove |
| Bad values | Domain rules | Correct |
| Low quality | QC metrics | Filter |

## Integration Flow

### With perform-analysis

```
perform-analysis (Step 3: Verify Resources)
  ↓
  "I see this is new/unfamiliar data"
  ↓
  Invokes: sanity-check-data
  ↓
  [8-step validation]
  ↓
  Returns: Validation results
  ↓
perform-analysis continues with validated data
```

### With learn-tool

```
sanity-check-data (Step 3: Load Data)
  ↓
  "Need bcftools to read VCF"
  ↓
  Invokes: learn-tool
  ↓
  [Installs and tests bcftools]
  ↓
  Returns: bcftools ready
  ↓
sanity-check-data continues with tool
```

## Validation Report Format

Every validation ends with a structured report:

```
✓ DATA VALIDATION COMPLETE

Summary: [One-line description]

Key Findings:
- Dimensions: [N × M]
- Type: [Data type]
- Quality: [Good/Fair/Poor]
- Missing: [%]
- Issues: [List or None]

Sanity Checks:
✓ [Passed checks]
⚠ [Warnings]
✗ [Failed checks]

Recommendations:
1. [Action 1]
2. [Action 2]

Tool Compatibility: ✓ [Tools tested]

Files Created:
- [Figures/reports]
```

## What Makes Data "Good"

### Good Quality
✓ Complete or minimal missing (<5%)
✓ All identifiers unique
✓ Values in expected ranges
✓ No obvious errors
✓ Consistent format
✓ Tools can read it

### Fair Quality
⚠ Some missing (5-20%)
⚠ Minor inconsistencies
⚠ Some outliers
✓ Overall usable

### Poor Quality
✗ High missing (>20%)
✗ Major format issues
✗ Many invalid values
✗ Tools can't read it

## Tips for Best Results

### Provide Context
✅ "This is RNA-seq count data from..."
✅ "Clinical data with patient outcomes"
✅ "VCF from whole-genome sequencing"

### Specify Expectations
✅ "Should have ~20,000 genes"
✅ "Expecting values 0-100"
✅ "Data should be log-transformed"

### Mention Analysis Plans
✅ "Will analyze with DESeq2"
✅ "Need to run GWAS with PLINK"
✅ "Plan to use for ML model"

This helps Claude:
- Choose appropriate checks
- Test correct tools
- Provide relevant recommendations

## Quick Commands

### Check any file
```
Check the data in [file]
```

### Download and validate
```
Download and validate data from [URL]
```

### Validate specific aspect
```
Sanity check the [aspect] in [file]
Example: "Sanity check the quality scores in variants.vcf"
```

### Tool compatibility
```
Check if [file] works with [tool]
Example: "Check if counts.csv works with DESeq2"
```

## After Validation

Based on quality assessment:

**If Good:**
→ Proceed with analysis
→ No preprocessing needed

**If Fair:**
→ Address specific issues noted
→ Consider data cleaning
→ Proceed with caution

**If Poor:**
→ Review recommendations
→ Fix major issues first
→ May need to re-collect/re-process

## Getting Help

If validation reveals unexpected issues:
1. Review the detailed report
2. Check recommendations section
3. Investigate specific warnings
4. Ask Claude to explain findings
5. Request suggestions for fixing issues

## Customization

Edit `skills/sanity-check-data/SKILL.md` to:
- Add project-specific validation rules
- Include your data standards
- Add custom quality thresholds
- Specify required tools for your domain
