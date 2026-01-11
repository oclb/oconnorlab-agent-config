# Example Workflows

## Complete Example: T-test Analysis

```
User: "Run a t-test comparing treatment vs control groups in the expression data"

Claude:

STEP 1: Understand Motivation
Let me make sure I understand: You want to compare gene expression levels between
treatment and control groups to identify if there are significant differences.
This will help determine if the treatment has a measurable effect on gene expression.

STEP 2: Set Expectations
Based on the experimental design, I expect to see:
- Some genes with significant differential expression (treatment effect)
- Majority of genes unchanged (null effect)
- Effect sizes varying across genes
Confidence: Medium (depends on sample size and treatment efficacy)

STEP 3: Verify Resources
I need:
Data:
- Expression data: data/expression_matrix.csv
- Sample metadata: data/sample_info.csv (treatment/control labels)

Tools:
- Python with scipy, pandas, matplotlib (checking...)
✓ All required packages available

Let me verify the data structure...
[reads files to confirm format]
✓ Expression data: 10,000 genes × 50 samples
✓ Metadata: 50 samples (25 treatment, 25 control)

STEP 4: Make a Plan
Plan:
1. Load expression data and metadata
2. For each gene:
   - Extract treatment and control values
   - Run Welch's t-test
   - Calculate fold change
3. Adjust p-values for multiple testing (FDR)
4. Create volcano plot
5. Save results table
6. Report significant genes

STEP 5: Perform Analysis
[runs analysis script]
Analysis complete in 12 seconds.

STEP 6: Display Results
[Shows summary table of top genes]
[Displays volcano plot]

KEY TAKEAWAY: Found 342 significantly differentially expressed genes (FDR < 0.05),
with 178 upregulated and 164 downregulated in treatment vs control, indicating
substantial transcriptional response to treatment.

STEP 7: Document Choices and Challenges
Choices Made:
- Used Welch's t-test (unequal variances assumed)
- Applied Benjamini-Hochberg FDR correction
- Set significance threshold at FDR < 0.05

Challenges:
- Found 3 genes with missing values in >20% of samples, excluded from analysis

STEP 8: Files Created

Scripts:
- scripts/differential_expression_ttest.py

Results:
- results/ttest_results_all_genes.csv (complete results, 10,000 genes)
- results/significant_genes_fdr05.csv (342 significant genes)

Figures:
- figures/volcano_plot.png
- figures/expression_heatmap_top50.png
```

## Special Cases

### Exploratory vs. Confirmatory Analysis

**Exploratory**: No strong prior expectations
- State that you're exploring
- Use appropriate multiple testing corrections
- Focus on generating hypotheses
- Be careful about over-interpreting

**Confirmatory**: Testing specific hypothesis
- State the hypothesis clearly
- Pre-specify analysis plan
- Report exactly as planned
- Distinguish from any post-hoc analyses

### Negative Results

**If results don't match expectations:**
1. Don't assume failure
2. Check for data/code errors
3. Consider alternative explanations
4. Report honestly
5. Discuss implications

### Reproductions

**If reproducing a previous analysis:**
- State what you're reproducing
- Note any differences in methods/data
- Compare to original results
- Explain any discrepancies

### Multi-Stage Analyses

**For complex pipelines:**
- Save intermediate results
- Validate each stage
- Document transitions between stages
- Make it possible to restart from any stage
