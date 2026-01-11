# Example Workflows

## Complete Example: GEO Dataset Validation

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

## Report Template

```
================================================================================
DATA VALIDATION REPORT
================================================================================

Dataset: [filename]
Date checked: [date]
Source: [origin]

ACQUISITION
-----------
Location: [path]
Size: [size]
Download: ✓/✗ [status]
Format: [format]

FORMAT
------
File type: [type]
Encoding: [encoding]
Delimiter: [delimiter]
Header row: Yes/No
Row names: Yes/No
Compression: [type or None]

STRUCTURE
---------
Dimensions: [rows] × [columns]
Rows represent: [what]
Columns represent: [what]

DATA STATISTICS
--------------
Total cells: [count]
Data type: [type]
Complete cells: [count] ([%])
Missing cells: [count] ([%])
Value range: [min, max]

SANITY CHECKS
------------
[✓/✗/⚠] [check description]
...

RECOMMENDATIONS
---------------
1. [recommendation]
2. [recommendation]

Ready for downstream analysis: ✓/✗ [YES/NO]

FILES CREATED
------------
- [path] - [description]
================================================================================
```

## Large File Handling

**For files too large to load entirely:**

```python
# Sample the data: Read first N rows
df_sample = pd.read_csv('large_file.csv', nrows=10000)

# Stream processing: Process in chunks
chunk_size = 10000
chunks = []
for chunk in pd.read_csv('large_file.csv', chunksize=chunk_size):
    chunks.append(chunk.describe())
summary = pd.concat(chunks).groupby(level=0).mean()
```

**For indexed files:**
- Use tabix for indexed VCF/BED
- Use BAM index for random access
- Query summary statistics with command-line tools
