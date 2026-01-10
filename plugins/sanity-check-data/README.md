# Sanity Check Data Plugin

A comprehensive Claude Code skill for acquiring, validating, and exploring new datasets.

## What It Does

When you ask Claude to download, access, or validate data, this skill automatically guides Claude through an 8-step systematic validation process:

1. **Acquire/Locate** - Download or find the dataset
2. **Examine Format** - Understand file structure and encoding
3. **Load Data** - Read with appropriate tool
4. **Compute Statistics** - Characterize size and content
5. **Sanity Checks** - Verify data makes sense
6. **Visual Summary** - Create informative plots (optional)
7. **Generate Report** - Comprehensive validation summary
8. **Tool Compatibility** - Verify analysis tools can read it

## When to Use

This skill activates when you ask to:
- "Download this dataset"
- "Check this data file"
- "Validate the data"
- "Sanity check the expression data"
- "Explore what's in this dataset"
- "Access data from GEO/TCGA/etc"
- "Simulate data and verify it"

Also automatically invoked by **perform-analysis** when encountering unfamiliar data.

## Key Features

### Comprehensive Data Validation

Systematically checks:
- File format and structure
- Dimensions and identifiers
- Data types and ranges
- Missing values and completeness
- Domain-specific requirements
- Tool compatibility

### Smart Format Detection

Handles diverse data formats:
- **Tabular**: CSV, TSV, Excel, Parquet
- **Biological**: VCF, BED, BAM, FASTQ, GTF/GFF
- **Hierarchical**: JSON, XML, HDF5
- **Statistical**: RDS, .mat, .sav
- **Compressed**: gzip, bzip2, zip

### Automatic Tool Installation

If a specialized tool is needed to read the data:
- Invokes **learn-tool** skill automatically
- Installs and tests the tool
- Then proceeds with validation

### Domain-Specific Checks

Performs specialized validation based on data type:
- **Expression data**: Check for negative values, reasonable ranges, transformation
- **Clinical data**: Validate ages, categorical levels, patient IDs
- **Genomic variants**: Verify coordinates, alleles, quality scores
- **Sequence data**: Check read counts, quality encoding, length distribution

### Actionable Recommendations

Doesn't just identify issues - provides:
- Clear assessment (Good/Fair/Poor quality)
- Specific problems found
- Suggested solutions
- Next steps for analysis

## Examples

### Download and Validate Public Data

```
You: Download and check the expression data from GEO GSE12345

Claude:

STEP 1: Acquire Data
Downloading from GEO FTP server...
✓ Downloaded: GSE12345_series_matrix.txt.gz (23.4 MB)

STEP 2: Examine Format
File type: Tab-delimited text (gzipped)
Format: GEO series matrix
✓ Uncompressed successfully

STEP 3: Load Data
Tool: pandas (Python) ✓ Available
✓ Loaded: 54,675 probes × 12 samples

STEP 4: Basic Statistics
Dimensions: 54,675 probes × 12 samples
Data type: Float (log2-transformed expression)
Missing: 0 values (0%)
Range: [3.2, 14.8]

STEP 5: Sanity Checks
✓ All identifiers unique
✓ No missing values
✓ Range appropriate for log2 microarray
✓ Distribution appears normal

STEP 6: Visual Summary
Created: figures/GSE12345_distribution.png

STEP 7: Report
✓ DATA VALIDATION COMPLETE

Quality: Excellent
Issues: None
Recommendation: ✓ Ready for differential expression analysis

STEP 8: Tool Compatibility
Tested: limma (R package)
✓ Compatible - data loads successfully
```

### Validate Local Data File

```
You: Check the data in experiment_results.csv

Claude:

STEP 1: Locate Data
Found: data/experiment_results.csv
Size: 2.3 MB

STEP 2: Examine Format
File type: CSV (comma-delimited)
Encoding: UTF-8
Header: Yes
Row names: Yes

STEP 3: Load Data
Tool: pandas ✓
✓ Loaded: 1,000 samples × 50 features

STEP 4: Basic Statistics
Dimensions: 1,000 samples × 50 features
Missing: 234 values (0.47%)
Data types: 45 numeric, 5 categorical

Numeric columns range: [-2.5, 10.3]
Categorical columns: 2-5 levels each

STEP 5: Sanity Checks
✓ No empty rows/columns
✓ Identifiers unique
⚠ WARNING: Column "age" has values >120
⚠ WARNING: Column "measurement_3" has 45% missing

STEP 6: Visual Summary
Created: figures/missing_data_heatmap.png
Created: figures/distributions.png

STEP 7: Report
✓ DATA VALIDATION COMPLETE

Quality: Fair
Issues:
- Unrealistic age values (max: 245)
- High missingness in measurement_3 (45%)

Recommendations:
1. Verify age values (likely data entry errors)
2. Consider excluding measurement_3 or imputing
3. Otherwise data is suitable for analysis

STEP 8: Tool Compatibility
Data format: Standard tabular data
✓ Compatible with: scikit-learn, statsmodels, R
```

### Validate Simulated Data

```
You: Simulate 1000 samples from a normal distribution and validate

Claude:

STEP 1: Generate Data
Simulating normal distribution...
Parameters: mean=100, sd=15, n=1000
Random seed: 42 (for reproducibility)
✓ Generated: 1,000 samples

STEP 2: Examine Format
Format: Numpy array (1D)
Saved as: data/simulated_normal_n1000.csv

STEP 3: Load Data
✓ Loaded successfully

STEP 4: Basic Statistics
Samples: 1,000
Theoretical mean: 100, Observed: 99.8
Theoretical SD: 15, Observed: 15.2
Range: [47.3, 152.1]

STEP 5: Sanity Checks
✓ No missing values
✓ No infinite values

Statistical validation:
- Shapiro-Wilk normality test: p=0.234 (appears normal ✓)
- Mean within expected range ✓
- SD within expected range ✓

STEP 6: Visual Summary
Created: figures/simulated_distribution.png
Shows: Histogram with theoretical normal curve overlay

STEP 7: Report
✓ DATA VALIDATION COMPLETE

Summary: Simulated normal data matches expected properties

Quality: Excellent
Issues: None
Validation: Simulated data has expected statistical properties

Files created:
- data/simulated_normal_n1000.csv
- figures/simulated_distribution.png
```

### Check Genomic Data (VCF)

```
You: Validate the variants.vcf.gz file

Claude:

STEP 1: Locate Data
Found: data/variants.vcf.gz
Size: 145 MB (compressed)

STEP 2: Examine Format
File type: VCF (Variant Call Format), gzipped
Checking with file command...
✓ Valid gzip-compressed VCF

STEP 3: Load Data
Tool needed: bcftools
Checking if installed... Not found.

Let me use the learn-tool skill to install bcftools...
[Installs bcftools via brew]
✓ bcftools installed and working

Reading VCF with bcftools...
✓ VCF is valid and indexed

STEP 4: Basic Statistics
Samples: 100 individuals
Variants: 2,547,892 SNPs
Chromosomes: chr1-chr22, chrX, chrY
Genome build: GRCh38

Variant types:
- SNPs: 2,450,123 (96.2%)
- Indels: 97,769 (3.8%)

Quality scores:
- Mean QUAL: 187.3
- Min QUAL: 30.0
- High-quality variants (QUAL>30): 100%

STEP 5: Sanity Checks
✓ All chromosomes have valid names
✓ All positions are positive integers
✓ All variants have valid ref/alt alleles
✓ All quality scores ≥ 30
✓ No duplicate positions

Genotype completeness:
- Mean genotype call rate: 98.7%
- Samples with <90% calls: 0

STEP 6: Visual Summary
[Created chromosome distribution plot]

STEP 7: Report
✓ DATA VALIDATION COMPLETE

Summary: High-quality WGS variant calls for 100 samples

Quality: Excellent
Issues: None

Recommendation: ✓ Ready for GWAS/association analysis

STEP 8: Tool Compatibility
Tested tools:
✓ bcftools - reads successfully
✓ PLINK - format conversion works
✓ VCFtools - compatible

Ready for analysis with PLINK, GCTA, or other GWAS tools
```

## Installation

Already included if you're using the claude-config repository with `pluginDirs` configured in `settings.json`.

## Integration with Other Skills

### perform-analysis

Automatically invoked during analysis when encountering new data:

```
[User asks to perform analysis]

STEP 3: Verify Resources (in perform-analysis)
I see this is a new dataset I haven't worked with before.
Let me validate it first...

[Invokes sanity-check-data]

[After validation]
✓ Data validated. Proceeding with analysis...
```

### learn-tool

Automatically invoked when specialized tools are needed:

```
STEP 3: Load Data (in sanity-check-data)
This BAM file requires samtools to read.
Let me learn how to use samtools...

[Invokes learn-tool]

[After learning]
Now reading BAM file with samtools...
```

## What Gets Validated

### Tabular Data (CSV, TSV, Excel)
- ✓ Dimensions and structure
- ✓ Column/row names
- ✓ Data types
- ✓ Missing values
- ✓ Value ranges
- ✓ Outliers
- ✓ Duplicates

### Expression Data (RNA-seq, Microarray)
- ✓ Non-negative values (for raw counts)
- ✓ Integer vs. float (counts vs. normalized)
- ✓ Range consistency (raw vs. log-transformed)
- ✓ Sample-level quality (mean expression)
- ✓ Gene-level quality (variance)
- ✓ Format for DESeq2/edgeR/limma

### Clinical/Phenotype Data
- ✓ Age ranges (0-120)
- ✓ Categorical levels
- ✓ Date formats
- ✓ ID uniqueness
- ✓ Missing data patterns

### Genomic Variants (VCF)
- ✓ Valid chromosomes
- ✓ Position coordinates
- ✓ Reference/alternate alleles
- ✓ Quality scores
- ✓ Genotype calls
- ✓ Sample call rates

### Sequence Data (FASTQ)
- ✓ Read count
- ✓ Quality encoding (Phred+33)
- ✓ Read length distribution
- ✓ Quality score distribution

### Genomic Intervals (BED)
- ✓ Valid coordinates
- ✓ Chromosome names
- ✓ Start < End
- ✓ No overlaps (if expected)
- ✓ Sorted (if expected)

### Alignment Data (BAM)
- ✓ Header validity
- ✓ Reference sequences
- ✓ Sorted/indexed status
- ✓ Read statistics
- ✓ Mapping quality

## Validation Report

Every validation produces a structured report:

```
================================================================================
DATA VALIDATION REPORT
================================================================================

Dataset: [name]
Source: [origin]
Date: [timestamp]

ACQUISITION: [How data was obtained]
FORMAT: [File type, structure]
LOADING: [Tool used, status]
STATISTICS: [Dimensions, ranges, missingness]
SANITY CHECKS: [Results of all checks]
INTERPRETATION: [What the data is, quality assessment]
RECOMMENDATIONS: [Next steps]
TOOL COMPATIBILITY: [Analysis tool status]

FILES CREATED: [Plots, reports saved]
================================================================================
```

## Best Practices

### Before Analysis
✅ Always validate new data before analysis
✅ Check data matches your expectations
✅ Verify tools can read the format

### For Downloads
✅ Document the source URL
✅ Save download date
✅ Verify file integrity (size, format)

### For Simulations
✅ Set random seed
✅ Document parameters
✅ Validate against theoretical properties

### For Large Files
✅ Use streaming/sampling for initial checks
✅ Don't load everything into memory
✅ Use specialized tools (bcftools, samtools)

## Common Issues Detected

| Issue | Detection | Recommendation |
|-------|-----------|----------------|
| Missing values | % missing per column | Impute or exclude |
| Outliers | Range checks, z-scores | Investigate, winsorize |
| Wrong format | Load failures | Convert or fix |
| Duplicates | ID uniqueness | Remove or merge |
| Wrong scale | Range analysis | Log-transform or normalize |
| Empty columns | All-NA check | Remove |
| Unrealistic values | Domain checks | Correct data entry errors |
| Low quality | Quality metrics | Filter or re-process |

## Customization

Edit `skills/sanity-check-data/SKILL.md` to:
- Add domain-specific validation rules
- Include project-specific data standards
- Add custom sanity checks
- Specify preferred tools

## Files Created During Validation

Typical outputs:
- **Figures**: Distribution plots, missing data heatmaps
- **Reports**: Validation summary (text/markdown)
- **Logs**: Detailed validation log
- **Processed data**: Cleaned/filtered versions (if requested)

## Troubleshooting

### Tool Not Available
**Expected**: Skill should invoke learn-tool
**If it doesn't**: Manually install or request installation

### Large File Issues
**Solution**: Validation will use sampling or streaming
**For very large files**: Specify to only check metadata

### Format Not Recognized
**Solution**: Skill will attempt multiple parsers
**If fails**: Manually specify format parameters

### Unexpected Results
**Review**: Check Step 7 report for detailed findings
**Action**: Follow recommendations provided

## Version

Current version: 1.0.0

## Future Enhancements

Planned features:
- Database validation (SQL, MongoDB)
- Multi-file dataset validation
- Automated quality score generation
- Integration with data catalogs
- Metadata validation (ontologies, controlled vocabularies)

## Contributing

Customize for your domain:
- Add field-specific validation rules
- Include common data sources for your field
- Add domain-specific quality metrics
- Customize visualization styles
