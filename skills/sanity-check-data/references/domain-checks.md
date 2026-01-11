# Domain-Specific Validation

## Expression Data (RNA-seq, microarray)

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

## Clinical Data

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

## Genomic Variants (VCF)

```bash
# Check VCF integrity with bcftools
bcftools stats data.vcf.gz | grep "number of SNPs"
bcftools stats data.vcf.gz | grep "number of samples"

# Check for invalid coordinates
bcftools query -f '%CHROM\t%POS\n' data.vcf.gz | awk '$2 < 0 {print "Invalid position:", $0}'
```

## Sequence Data (FASTQ)

```bash
# Count reads
echo "Total reads:" $(( $(wc -l < data.fastq) / 4 ))

# Check quality encoding
head -n 100 data.fastq | awk 'NR % 4 == 0' | od -c | head
# Should see ASCII 33-126 for Phred+33

# Check read length distribution
head -n 40000 data.fastq | awk 'NR % 4 == 2 {print length}' | sort | uniq -c
```

## Tool Compatibility Testing

### RNA-seq with DESeq2

```R
library(DESeq2)

counts <- read.csv('data/counts.csv', row.names=1)
metadata <- read.csv('data/samples.csv', row.names=1)

if (!all(counts >= 0)) {
  print("ERROR: DESeq2 requires non-negative counts")
} else if (!all(counts %% 1 == 0)) {
  print("WARNING: DESeq2 expects integer counts, data has decimals")
} else {
  dds <- DESeqDataSetFromMatrix(
    countData = round(counts),
    colData = metadata,
    design = ~ condition
  )
  print("✓ Data is compatible with DESeq2")
}
```

### Command-line Tools

```bash
# Test bcftools can read VCF
bcftools view data.vcf.gz | head

# Test samtools can read BAM
samtools view data.bam | head

# Test bedtools can read BED
bedtools sort -i data.bed | head
```

## Expected Tools by Data Type

| Data Type | Common Tools |
|-----------|--------------|
| RNA-seq counts | DESeq2, edgeR, limma |
| VCF variants | bcftools, PLINK, GATK |
| BAM alignments | samtools, GATK, Picard |
| BED intervals | bedtools, UCSC tools |
| Single-cell | Seurat, Scanpy |
| GWAS | PLINK, GCTA, BOLT-LMM |
