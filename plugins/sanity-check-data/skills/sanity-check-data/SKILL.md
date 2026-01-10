---
name: sanity-check-data
description: This skill should be used when the user asks to "download a dataset", "access new data", "check this data", "validate data", "sanity check data", "explore a dataset", or when working with unfamiliar data for the first time. Also triggers when simulating or generating new data that needs validation.
version: 1.0.0
---

# Sanity Check Data Skill

This skill provides a systematic framework for acquiring, validating, and exploring new datasets to ensure they are suitable for analysis.

## When This Skill Applies

Use this skill when:
- Downloading a new dataset from a URL or repository
- Accessing data for the first time
- Receiving or generating simulated data
- User explicitly asks to validate or sanity-check data
- The **perform-analysis** skill encounters unfamiliar data
- Need to verify data is in correct format
- Want to understand what a dataset contains

## Data Validation Process

Follow these steps systematically:

### Step 1: Acquire or Locate the Data

**Determine how to access the dataset.**

#### For Downloaded Data

If the user provides a URL or reference to public data:

1. **Identify the source**:
   - Public repository (GEO, TCGA, UK Biobank, etc.)
   - Supplementary data from paper
   - Direct URL to file
   - API endpoint

2. **Download the data**:
   ```bash
   # Direct download
   wget https://example.com/data.csv -O data/dataset_name.csv

   # Or using curl
   curl -L https://example.com/data.csv -o data/dataset_name.csv

   # For APIs
   curl -H "Authorization: Bearer TOKEN" https://api.example.com/data > data.json
   ```

3. **Verify download**:
   - Check file exists and is non-empty
   - Verify file size is reasonable
   - Check for error messages in the file

**Common data sources:**
- **GEO**: `wget ftp://ftp.ncbi.nlm.nih.gov/geo/...`
- **TCGA**: Use GDC data portal or API
- **UK Biobank**: Requires authentication
- **Kaggle**: `kaggle datasets download...`
- **Zenodo**: Direct download links

#### For Simulated Data

If generating/simulating data:

1. **Understand simulation requirements**:
   - What process is being simulated?
   - What parameters should be used?
   - What format should output be?

2. **Generate the data**:
   - Use appropriate tool (R, Python, custom script)
   - Set random seed for reproducibility
   - Document simulation parameters

3. **Save simulated data**:
   - Use descriptive filename
   - Include simulation parameters in filename or metadata
   - Save in appropriate format

**Example:**
```python
# Simulate expression data
np.random.seed(42)
expression = np.random.lognormal(mean=5, sigma=2, size=(1000, 50))
pd.DataFrame(expression).to_csv('data/simulated_expression_n50_g1000.csv')
```

#### For Existing Local Data

If data already exists locally:

1. **Locate the file**:
   - Ask user for path if unknown
   - Search common locations (data/, input/, results/)
   - Use Glob/Grep to find files matching patterns

2. **Verify accessibility**:
   ```bash
   # Check file exists and is readable
   ls -lh /path/to/data.csv

   # Check permissions
   test -r /path/to/data.csv && echo "Readable" || echo "Not readable"
   ```

**Document the source:**
```
Data Source:
- Location: data/experiment_2024.csv
- Source: [Downloaded from GEO GSE12345 / Simulated / Provided by user]
- Date acquired: 2026-01-10
- Original URL: [if applicable]
```

### Step 2: Examine the Format

**Understand the structure and format of the data.**

#### Determine File Type

**Common formats:**
- **Tabular**: CSV, TSV, Excel (.xlsx), Parquet
- **Biological**: VCF, BED, BAM, FASTQ, GFF/GTF
- **Hierarchical**: JSON, XML, HDF5, NetCDF
- **Statistical**: RDS, RData, .mat (MATLAB), .sav (SPSS)
- **Image**: TIFF, PNG, DICOM
- **Database**: SQLite, PostgreSQL dump

**Check format:**
```bash
# File extension
file data.csv

# Magic number (first few bytes)
head -c 100 data.bin | od -c

# Compression
file data.gz  # Should detect gzip, bzip2, etc.
```

#### Inspect File Structure

**For text files:**
```bash
# First 10 lines
head -10 data.csv

# Last 10 lines (check for footers)
tail -10 data.csv

# Line count
wc -l data.csv

# Check for special characters or encoding issues
file -i data.csv  # Shows encoding
```

**For binary files:**
```bash
# File size
ls -lh data.bam

# File type details
file data.bam
```

**Key questions to answer:**
- Does it have a header row?
- What delimiter is used? (comma, tab, space, pipe)
- Are there row names/IDs?
- Are there comment lines (# or //)?
- What is the encoding (UTF-8, ASCII, etc.)?
- Is it compressed?

#### Identify Data Structure

**Determine the organization:**

**Rectangular data (samples × features):**
- Rows = samples/observations, Columns = features/variables
- Rows = features/genes, Columns = samples
- Need to determine which orientation

**Sequence data:**
- FASTQ: Reads with quality scores
- VCF: Variants with genotypes
- BED: Genomic intervals

**Hierarchical data:**
- JSON: Nested objects/arrays
- HDF5: Groups and datasets
- XML: Nested tags

**Time series:**
- Timestamp column
- Value columns
- Potentially irregular intervals

**Document format:**
```
Data Format:
- File type: CSV
- Dimensions: [To be determined]
- Delimiter: Comma
- Header: Yes (first row)
- Row names: Yes (first column = sample IDs)
- Encoding: UTF-8
- Compression: None
```

### Step 3: Load Data with Appropriate Tool

**Use the right tool to read the data.**

#### Choose the Right Tool

**Based on format and domain:**

**Tabular data:**
- Python: `pandas.read_csv()`, `pandas.read_excel()`
- R: `read.csv()`, `read_tsv()`, `readr::read_csv()`

**Biological data:**
- VCF: `bcftools`, `pysam`, `vcftools`, Bioconductor `VariantAnnotation`
- BAM: `samtools`, `pysam`, Bioconductor `Rsamtools`
- BED: `bedtools`, `pybedtools`, Bioconductor `rtracklayer`
- FASTQ: `seqtk`, `Biopython`, Bioconductor `ShortRead`

**Statistical data:**
- R objects: `readRDS()`, `load()`
- MATLAB: `scipy.io.loadmat()`
- SPSS: `pyreadstat`, `haven::read_sav()`

**Hierarchical:**
- JSON: `json` module (Python), `jsonlite` (R)
- HDF5: `h5py` (Python), `rhdf5` (R)

#### Verify Tool Availability

**If tool doesn't exist:**
- Invoke the **learn-tool** skill to install it
- Example: "I need pysam to read this BAM file. Let me learn how to use it..."

#### Load the Data

**Attempt to read the data:**

```python
# Example for CSV
import pandas as pd
df = pd.read_csv('data/experiment.csv', index_col=0)

# Check if it loaded successfully
print(f"Loaded successfully: {df.shape}")
```

```R
# Example for TSV
data <- read.table('data/experiment.tsv', header=TRUE, sep='\t')
print(paste("Loaded:", nrow(data), "rows,", ncol(data), "columns"))
```

**Handle errors:**
- Encoding issues: Try different encodings (UTF-8, latin1, etc.)
- Delimiter confusion: Test comma, tab, space, semicolon
- Quote issues: Adjust quote character
- Skip rows: Handle headers or comments

**Verify data loaded correctly:**
```python
# Check for parsing issues
print(df.dtypes)  # Are columns the right type?
print(df.isnull().sum())  # How many missing values?
print(df.head())  # Does it look right?
```

**Document tool used:**
```
Loading Tool:
- Tool: pandas (Python)
- Command: pd.read_csv('data.csv', index_col=0)
- Status: ✓ Loaded successfully
- Issues: None / [describe any issues and solutions]
```

### Step 4: Compute Basic Statistics

**Characterize the size and scope of the data.**

#### Dimensions

**Determine the size:**

```python
# Python pandas
n_rows, n_cols = df.shape
print(f"Dimensions: {n_rows} rows × {n_cols} columns")
```

```R
# R
cat(sprintf("Dimensions: %d rows × %d columns\n", nrow(data), ncol(data)))
```

**Interpret dimensions based on data type:**

- **Expression data**:
  - Samples × Genes or Genes × Samples?
  - "1,000 genes × 50 samples"

- **Clinical data**:
  - "200 patients × 30 clinical variables"

- **Sequence data**:
  - "2.5M reads, paired-end"

- **Variants**:
  - "45,000 SNPs × 100 individuals"

#### Row and Column Names

**Examine identifiers:**

```python
# Check row names
print(f"Row names: {df.index[:5].tolist()}")  # First 5
print(f"Row name type: {type(df.index[0])}")

# Check column names
print(f"Column names: {df.columns[:10].tolist()}")  # First 10
print(f"Column name pattern: {df.columns[0]}")  # Example
```

**Verify names make sense:**
- Do sample IDs follow expected format?
- Are gene names recognizable (HUGO symbols, Ensembl IDs)?
- Are column names descriptive?

#### Data Types

**Check what kind of data each column contains:**

```python
# Python
print(df.dtypes)
print(f"Numeric columns: {df.select_dtypes(include='number').shape[1]}")
print(f"Categorical columns: {df.select_dtypes(include='object').shape[1]}")
```

```R
# R
print(sapply(data, class))
```

**Common types:**
- Numeric: Continuous measurements, counts
- Categorical: Factors, groups
- Text: Free text, sequences
- Dates: Timestamps, dates
- Boolean: True/False, Yes/No

#### Missing Data

**Quantify missingness:**

```python
# Python
missing = df.isnull().sum()
pct_missing = 100 * missing / len(df)
print(f"Columns with >5% missing:\n{pct_missing[pct_missing > 5]}")
```

```R
# R
missing <- sapply(data, function(x) sum(is.na(x)))
print(missing[missing > 0])
```

**Important questions:**
- Are missing values expected (e.g., dropout in single-cell)?
- Is there a pattern to missingness?
- Are any samples/features mostly missing?

#### Value Ranges

**For numeric data, check ranges:**

```python
# Summary statistics
print(df.describe())

# Min/max for key columns
for col in df.select_dtypes(include='number').columns[:5]:
    print(f"{col}: [{df[col].min():.2f}, {df[col].max():.2f}]")
```

**Check for:**
- Negative values where unexpected
- Extreme outliers
- Values outside biological/physical limits
- Scale (raw counts vs. normalized vs. log-transformed)

#### Summary Report

**Provide a clear statistical summary:**

```
Data Statistics:

Dimensions:
- Rows: 15,000 (genes)
- Columns: 48 (samples)
- Total cells: 720,000

Identifiers:
- Row names: Gene symbols (BRCA1, TP53, ...)
- Column names: Sample IDs (Sample_001, Sample_002, ...)

Data types:
- All columns: Numeric (float64)
- Values appear to be log2-transformed expression

Missing data:
- 0 missing values (complete dataset)

Value ranges:
- Min: -2.3 (likely low/no expression)
- Max: 15.7 (highly expressed genes)
- Mean: 5.2
- Median: 5.1
- Distribution appears roughly normal
```

### Step 5: Perform Sanity Checks

**Verify the data makes sense.**

#### Automatic Checks

**Perform these for all datasets:**

**1. No completely empty rows/columns:**
```python
# Check for all-NA rows/columns
empty_rows = df.isnull().all(axis=1).sum()
empty_cols = df.isnull().all(axis=0).sum()
print(f"Empty rows: {empty_rows}, Empty columns: {empty_cols}")
```

**2. Reasonable dimensions:**
```python
# Check if dimensions make sense
if n_rows < 1 or n_cols < 1:
    print("WARNING: Data has zero rows or columns!")
elif n_rows == 1 or n_cols == 1:
    print("NOTICE: Data is one-dimensional")
```

**3. Unique identifiers:**
```python
# Check for duplicate IDs
duplicate_rows = df.index.duplicated().sum()
duplicate_cols = df.columns.duplicated().sum()
print(f"Duplicate row names: {duplicate_rows}")
print(f"Duplicate column names: {duplicate_cols}")
```

**4. Consistent data types:**
```python
# Check if supposedly numeric columns have non-numeric values
for col in df.columns:
    if df[col].dtype == 'object':
        try:
            pd.to_numeric(df[col])
            print(f"WARNING: Column {col} is object but could be numeric")
        except:
            pass
```

**5. Special values:**
```python
# Check for infinite values
inf_count = np.isinf(df.select_dtypes(include='number')).sum().sum()
print(f"Infinite values: {inf_count}")

# Check for NaN (different from missing)
nan_count = np.isnan(df.select_dtypes(include='number')).sum().sum()
print(f"NaN values: {nan_count}")
```

#### Domain-Specific Checks

**Based on data type, perform specialized validation:**

**Expression data (RNA-seq, microarray):**
```python
# Should not have negative values (unless normalized)
if (df < 0).any().any():
    print("NOTICE: Negative values present (data may be normalized/scaled)")

# Raw counts should be integers
if df.dtype == float:
    if (df % 1 == 0).all().all():
        print("NOTICE: Values are floats but all integers (may be counts)")

# Check for reasonable expression range
if df.max().max() < 50:
    print("NOTICE: Low max value, data may be log-transformed")
```

**Clinical data:**
```python
# Age should be reasonable
if 'age' in df.columns:
    if df['age'].max() > 120 or df['age'].min() < 0:
        print("WARNING: Unrealistic age values detected")

# Categorical variables should have reasonable number of levels
for col in df.select_dtypes(include='object').columns:
    n_unique = df[col].nunique()
    if n_unique > 0.5 * len(df):
        print(f"NOTICE: {col} has {n_unique} unique values (might be ID, not categorical)")
```

**Genomic variants (VCF):**
```bash
# Check VCF integrity with bcftools
bcftools stats data.vcf.gz | grep "number of SNPs"
bcftools stats data.vcf.gz | grep "number of samples"

# Check for invalid coordinates
bcftools query -f '%CHROM\t%POS\n' data.vcf.gz | awk '$2 < 0 {print "Invalid position:", $0}'
```

**Sequence data (FASTQ):**
```bash
# Count reads
echo "Total reads:" $(( $(wc -l < data.fastq) / 4 ))

# Check quality encoding
head -n 100 data.fastq | awk 'NR % 4 == 0' | od -c | head
# Should see ASCII 33-126 for Phred+33

# Check read length distribution
head -n 40000 data.fastq | awk 'NR % 4 == 2 {print length}' | sort | uniq -c
```

#### User-Suggested Checks

**If the user suggests specific validations:**

```
User: "Check that all expression values are positive"

Claude:
Performing user-requested check: all values should be positive
Checking...
✓ PASS: All values are >= 0
Range: [0.0, 15.7]
```

```
User: "Verify the samples are balanced between cases and controls"

Claude:
Checking sample balance...
Cases: 24 samples (50%)
Controls: 24 samples (50%)
✓ PASS: Samples are balanced
```

**Always acknowledge and perform user-requested checks explicitly.**

#### Sanity Check Summary

**Report findings clearly:**

```
Sanity Checks:

Automatic Checks:
✓ No empty rows or columns
✓ Dimensions are reasonable (15,000 × 48)
✓ All row names are unique
✓ All column names are unique
✓ No infinite or NaN values
⚠ WARNING: 5 columns have >10% missing values

Domain-Specific Checks (Expression Data):
✓ All values are positive (or zero)
✓ Value range consistent with log2-transformed data [0, 15.7]
✓ No obvious outliers in global distribution
✓ Sample-wise mean expression is consistent across samples

User-Requested Checks:
✓ Verified gene symbols are valid HUGO symbols
✓ Confirmed data is from human (hg38 reference)

Overall: Data appears valid with minor concerns about missing values in 5 features
```

### Step 6: Provide Visual Summary (Optional)

**Create informative visualizations if helpful.**

#### Distribution Plots

**For numeric data:**
```python
import matplotlib.pyplot as plt

# Overall distribution
plt.figure(figsize=(10, 4))
plt.subplot(1, 2, 1)
df.values.flatten().hist(bins=50)
plt.title('Distribution of All Values')
plt.xlabel('Value')

# Per-sample distributions
plt.subplot(1, 2, 2)
df.boxplot()
plt.title('Distribution by Sample')
plt.xticks([])
plt.xlabel('Samples')

plt.tight_layout()
plt.savefig('figures/data_distribution.png', dpi=150)
```

#### Missing Data Heatmap

**Visualize missingness:**
```python
import seaborn as sns

# Missingness pattern
plt.figure(figsize=(10, 6))
sns.heatmap(df.isnull(), cbar=False, yticklabels=False)
plt.title('Missing Data Pattern')
plt.savefig('figures/missing_data.png', dpi=150)
```

#### Correlation Structure

**For smaller datasets:**
```python
# Pairwise correlations
corr = df.corr()
plt.figure(figsize=(8, 8))
sns.heatmap(corr, cmap='coolwarm', center=0, square=True)
plt.title('Feature Correlations')
plt.savefig('figures/correlation_matrix.png', dpi=150)
```

**Only create visualizations if they add value** - don't create plots just for the sake of it.

### Step 7: Generate Data Report

**Provide a comprehensive summary.**

#### Complete Report Template

```
================================================================================
DATA VALIDATION REPORT
================================================================================

Dataset: experiment_data.csv
Date checked: 2026-01-10
Source: Downloaded from GEO GSE12345

ACQUISITION
-----------
Location: data/experiment_data.csv
Size: 15.2 MB
Download: ✓ Successful
Format: CSV (comma-delimited)

FORMAT
------
File type: Text/CSV
Encoding: UTF-8
Delimiter: Comma
Header row: Yes
Row names: Yes (first column)
Compression: None

STRUCTURE
---------
Dimensions: 15,000 rows × 48 columns
Rows represent: Genes
Columns represent: Samples
Organization: Genes (rows) × Samples (columns)

Row names: Gene symbols (BRCA1, TP53, EGFR, ...)
  - Format: HUGO symbols
  - Unique: ✓ Yes
  - Pattern: All uppercase, alphanumeric

Column names: Sample IDs (Sample_001, Sample_002, ...)
  - Format: Sample_XXX
  - Unique: ✓ Yes
  - Pattern: Consistent naming

LOADING
-------
Tool: pandas (Python)
Command: pd.read_csv('data/experiment_data.csv', index_col=0)
Status: ✓ Successfully loaded
Issues: None

DATA STATISTICS
--------------
Total cells: 720,000
Data type: Numeric (float64)
Complete cells: 718,500 (99.8%)
Missing cells: 1,500 (0.2%)

Value range: [0.0, 15.7]
Mean: 5.2
Median: 5.1
SD: 2.3

MISSING DATA
-----------
Total missing: 1,500 (0.2%)
Rows with missing: 1,234 (8.2% of genes)
Columns with missing: 5 (10.4% of samples)

Columns with >10% missing:
  - Sample_042: 15.2% missing
  - Sample_043: 12.3% missing

SANITY CHECKS
------------
✓ No empty rows or columns
✓ Dimensions are reasonable
✓ All identifiers are unique
✓ No infinite values
✓ No NaN values
✓ All values are non-negative
✓ Range consistent with log2-transformed expression
✓ No extreme outliers detected
⚠ 5 samples have >10% missing values

INTERPRETATION
-------------
Data type: RNA-seq or microarray expression data (log2-transformed)
Quality: Good - data is well-formed with minimal missing values
Concerns: 5 samples have elevated missingness (consider excluding)
Recommendation: Data is suitable for analysis

Ready for downstream analysis: ✓ YES

FILES CREATED
------------
- None (read-only validation)

OR if visualizations were created:
- figures/data_distribution.png - Distribution of all values
- figures/missing_data.png - Pattern of missing values
- reports/data_validation_2026-01-10.txt - This report

================================================================================
```

#### Provide Actionable Recommendations

**Based on findings, suggest next steps:**

```
RECOMMENDATIONS:

1. Missing data handling:
   - Consider excluding Sample_042 and Sample_043 (>10% missing)
   - Impute missing values or use methods that handle missingness

2. Data preprocessing:
   - Data is already log-transformed, no further transformation needed
   - Check for batch effects between sample groups

3. Quality control:
   - Verify Sample_042 and Sample_043 didn't have technical issues
   - Consider PCA to identify outlier samples

4. Analysis readiness:
   ✓ Data is suitable for differential expression analysis
   ✓ Data is suitable for clustering/dimensionality reduction
   ✓ Format is compatible with DESeq2, edgeR, limma
```

### Step 8: Confirm Tool Compatibility

**If there's a specific tool designed for this data type, verify compatibility.**

#### Identify Expected Tools

**Based on data type:**

- **RNA-seq counts**: DESeq2, edgeR, limma
- **VCF variants**: bcftools, PLINK, GATK
- **BAM alignments**: samtools, GATK, Picard
- **BED intervals**: bedtools, UCSC tools
- **Single-cell**: Seurat, Scanpy, Bioconductor
- **GWAS**: PLINK, GCTA, BOLT-LMM

**Ask user if uncertain:**
```
This appears to be RNA-seq expression data. Common tools for analysis include:
- DESeq2 (R) - for differential expression from counts
- edgeR (R) - for differential expression
- limma (R) - for differential expression (microarray or RNA-seq)

Which tool do you plan to use for analysis, or should I test compatibility with all?
```

#### Test Tool Compatibility

**Verify the tool can read the data:**

```R
# Example: Test DESeq2 compatibility
library(DESeq2)

# Try to create DESeqDataSet
counts <- read.csv('data/counts.csv', row.names=1)
metadata <- read.csv('data/samples.csv', row.names=1)

# Check format requirements
if (!all(counts >= 0)) {
  print("ERROR: DESeq2 requires non-negative counts")
} else if (!all(counts %% 1 == 0)) {
  print("WARNING: DESeq2 expects integer counts, data has decimals")
} else {
  # Try to create object
  dds <- DESeqDataSetFromMatrix(
    countData = round(counts),
    colData = metadata,
    design = ~ condition
  )
  print("✓ Data is compatible with DESeq2")
}
```

**For command-line tools:**
```bash
# Test bcftools can read VCF
bcftools view data.vcf.gz | head

# Test samtools can read BAM
samtools view data.bam | head

# Test bedtools can read BED
bedtools sort -i data.bed | head
```

**Report compatibility:**
```
Tool Compatibility:

Tested: DESeq2 (R package for differential expression)
Status: ✓ Compatible
Notes: Data required rounding to integers (counts should be whole numbers)
Command to load:
  dds <- DESeqDataSetFromMatrix(
    countData = round(counts),
    colData = metadata,
    design = ~ condition
  )
```

## Integration with Other Skills

This skill works together with:

### perform-analysis
When perform-analysis encounters new/unfamiliar data in Step 3 (Verify Resources), it should invoke this skill:

```
[In perform-analysis Step 3]

I see this is the first time analyzing this dataset. Let me validate it first...

[Invokes sanity-check-data skill]

[After validation]
✓ Data validated successfully. Proceeding with analysis plan...
```

### learn-tool
When this skill needs a tool to read the data:

```
[In sanity-check-data Step 3]

This VCF file requires bcftools to read properly.
Let me learn how to use bcftools...

[Invokes learn-tool skill]

[After learning]
Now testing VCF file with bcftools...
```

## Special Cases

### Simulated Data

**When generating simulated data:**

1. Document simulation parameters
2. Set and report random seed
3. Verify simulated data has expected properties
4. Compare to theoretical expectations

**Example:**
```python
# Simulate data
np.random.seed(42)
data = np.random.normal(loc=100, scale=15, size=1000)

# Verify properties
print(f"Theoretical mean: 100, Observed: {data.mean():.2f}")
print(f"Theoretical SD: 15, Observed: {data.std():.2f}")

# Statistical test
from scipy import stats
_, p = stats.shapiro(data[:100])  # Test normality
print(f"Normality test p-value: {p:.3f}")
```

### Large Files

**For files too large to load entirely:**

1. **Sample the data**: Read first N rows
2. **Stream processing**: Process in chunks
3. **Index-based access**: Use tools that support indexing (tabix, BAM index)
4. **Summary statistics**: Use command-line tools (wc, awk)

**Example:**
```python
# For large CSV, read in chunks
chunk_size = 10000
chunks = []
for chunk in pd.read_csv('large_file.csv', chunksize=chunk_size):
    chunks.append(chunk.describe())

# Combine summaries
summary = pd.concat(chunks).groupby(level=0).mean()
```

### Remote Data

**For data that can't be downloaded:**

1. **API access**: Access via API endpoints
2. **Database query**: Query summary statistics
3. **Preview sample**: Download subset for validation

### Multiple Files

**For datasets split across files:**

1. Validate each file
2. Check consistency across files (same format, compatible dimensions)
3. Verify files can be merged/combined
4. Document the full dataset structure

## Best Practices

### 1. Always Document the Source
Track where data came from for reproducibility.

### 2. Be Thorough But Efficient
Don't load 100GB into memory if you can check it with command-line tools.

### 3. Validate Before Analyzing
Catch data issues early before spending time on analysis.

### 4. Communicate Clearly
Report findings in plain language, not just technical details.

### 5. Provide Actionable Recommendations
Don't just identify issues - suggest solutions.

### 6. Consider the User's Expertise
Explain what findings mean, don't assume knowledge.

### 7. Save Validation Reports
Create a record of what was checked and when.

## Output Template

Every data validation should end with:

```
✓ DATA VALIDATION COMPLETE

Summary: [One sentence describing the dataset]

Key findings:
- Dimension: [N × M]
- Type: [Data type]
- Quality: [Good/Fair/Poor]
- Issues: [None / List of issues]

Recommendation: [Ready for analysis / Needs preprocessing / Has critical issues]

[If issues exist]
Suggested actions:
1. [Action 1]
2. [Action 2]

Tool compatibility: ✓ Compatible with [tool name]

Files created:
- [List any reports or visualizations]
```

## Example Workflow

```
User: "Download and check the expression data from GEO GSE12345"

Claude:

STEP 1: Acquire Data
Downloading from GEO...
wget ftp://ftp.ncbi.nlm.nih.gov/geo/series/GSE12nnn/GSE12345/matrix/
✓ Downloaded: GSE12345_series_matrix.txt.gz (23.4 MB)

STEP 2: Examine Format
File type: Tab-delimited text (gzipped)
Format: GEO series matrix format
Uncompressing...
✓ Format identified: Expression matrix with GEO metadata header

STEP 3: Load Data
Tool needed: Python pandas (✓ available)
Loading data (skipping metadata header)...
✓ Loaded successfully: 54,675 probes × 12 samples

STEP 4: Basic Statistics
Dimensions: 54,675 probes × 12 samples
Total values: 656,100
Missing values: 0 (0%)
Value range: [3.2, 14.8]
Data type: Float (appears to be log2-transformed)

STEP 5: Sanity Checks
✓ No empty rows or columns
✓ All probe IDs unique
✓ All sample IDs unique
✓ Values in expected range for microarray
✓ No infinite or NaN values
✓ Distribution appears normal (appropriate for log2 data)

STEP 6: Visual Summary
Created: figures/GSE12345_distribution.png

STEP 7: Report
✓ DATA VALIDATION COMPLETE

Summary: Microarray expression data with 54,675 probes across 12 samples

Key findings:
- Dimension: 54,675 probes × 12 samples
- Type: Log2-transformed microarray expression
- Quality: Excellent - complete data, no missing values
- Issues: None detected

Recommendation: ✓ Ready for differential expression analysis

Tool compatibility: ✓ Compatible with limma (R package)

STEP 8: Tool Compatibility
Testing with limma...
✓ Data successfully loaded into ExpressionSet object
✓ Ready for limma analysis
```
