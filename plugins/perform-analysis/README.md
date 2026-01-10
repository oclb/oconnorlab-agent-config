# Perform Analysis Plugin

A comprehensive Claude Code skill that provides a systematic framework for performing data analyses and experiments.

## What It Does

When you ask Claude to perform a data analysis or run an experiment, this skill automatically guides Claude through an 8-step rigorous process:

1. **Understand Motivation** - Clarify why the question is being asked
2. **Set Expectations** - Determine expected results and confidence level
3. **Verify Resources** - Ensure all data and tools are available
4. **Make a Plan** - Create a step-by-step analysis plan
5. **Perform Analysis** - Execute the plan with appropriate monitoring
6. **Display Results** - Create tables/figures highlighting key findings
7. **Document Choices** - Explain decisions and challenges
8. **List Files** - Provide paths to all created files

## When to Use

This skill activates when you ask to:
- "Perform an analysis"
- "Run an experiment"
- "Analyze this data"
- "Test if X is correlated with Y"
- "Compute statistics on this dataset"
- "Run a model to predict..."

## Key Features

### Systematic Approach

The skill ensures Claude doesn't just run code blindly, but instead:
- Understands the scientific context
- Sets expectations before running analyses
- Validates data and tools exist
- Creates a clear plan
- Documents the entire process

### Resource Verification

Before starting, Claude will:
- Verify all required datasets are accessible
- Check that necessary tools are installed
- Ask you for file paths if needed
- Invoke the **learn-tool** skill for unfamiliar tools
- Invoke the **sanity-check-data** skill for new data (when available)

### Time Estimation

For long-running analyses, Claude will:
- Estimate how long the analysis will take
- Use sleep commands to wait appropriately
- Monitor progress periodically
- Update you on status

### Cluster Integration

For analyses requiring substantial compute:
- Detects when O2 cluster should be used
- Invokes **submit-O2-job** skill (when available)
- Handles SLURM job submission and monitoring

### Complete Documentation

After analysis, you'll receive:
- Clear visualization (table or figure)
- The single most important finding highlighted
- Explanation of methodological choices
- Challenges encountered and solutions
- Complete list of all files created (scripts, data, figures)

## Examples

### Simple Statistical Test

```
You: Run a t-test comparing group A and B in data.csv

Claude:
STEP 1: Understand Motivation
You want to test if there's a significant difference between groups A and B.

STEP 2: Set Expectations
Expected: Possible difference based on preliminary observations
Confidence: Medium

STEP 3: Verify Resources
✓ Data: data.csv (found)
✓ Tool: Python scipy (installed)

STEP 4: Plan
1. Load data
2. Run t-test
3. Report results

STEP 5: Perform Analysis
[runs analysis]

STEP 6: Display Results
| Group | Mean  | SD   |
|-------|-------|------|
| A     | 12.3  | 2.1  |
| B     | 15.7  | 2.4  |

t-statistic: -4.23, p-value: 0.0001

KEY TAKEAWAY: Group B shows significantly higher values than Group A
(t=-4.23, p<0.001, d=1.47), indicating a strong effect.

STEP 7: Choices Made
- Used Welch's t-test (unequal variances)

STEP 8: Files Created
- scripts/ttest_analysis.py
- results/ttest_results.csv
```

### Complex Analysis Pipeline

```
You: Analyze the RNA-seq data to find differentially expressed genes

Claude:
STEP 1: Understand Motivation
Let me confirm: You want to identify genes with significantly different
expression between conditions to understand the biological response?

[Interactive clarification if needed]

STEP 2: Set Expectations
Expected: Some differential expression based on experimental design
Confidence: Medium-High

STEP 3: Verify Resources
Data needed:
- Count matrix: data/counts.txt - Location?
- Sample info: data/samples.csv - Location?

[Asks for confirmation]

Tools needed:
- DESeq2 (R package)

I see DESeq2 isn't installed. Let me use the learn-tool skill to set it up...

[Installs and tests DESeq2]

STEP 4: Plan
1. Load count data and metadata
2. Filter low-count genes
3. Run DESeq2 differential expression
4. Apply FDR correction
5. Create MA plot and volcano plot
6. Generate results table

STEP 5: Perform Analysis
This analysis will take approximately 3-5 minutes (12,000 genes, 48 samples).

[Runs analysis with progress updates]

STEP 6: Display Results
[Shows summary table of top genes]
[Displays volcano plot]

KEY TAKEAWAY: Identified 1,247 differentially expressed genes (FDR < 0.05),
with enrichment in immune response pathways, consistent with the
treatment's expected mechanism.

STEP 7: Choices and Challenges
Choices:
- Used DESeq2 (better for count data than t-tests)
- FDR threshold of 0.05
- Filtered genes with mean count < 10

Challenges:
- 2 samples showed outlier behavior in PCA, excluded after inspection
- Initial run failed due to zeros in size factors, added pseudocount

STEP 8: Files Created
Scripts:
- scripts/deseq2_analysis.R
- scripts/preprocessing.R

Data:
- data/filtered_counts.csv
- results/deseq2_results_all.csv
- results/significant_genes_fdr05.csv

Figures:
- figures/ma_plot.png
- figures/volcano_plot.png
- figures/pca_plot.png
- figures/top50_heatmap.png
```

## Installation

### Option 1: Use from Config Repository

If you're using the claude-config repository, the plugin is automatically available via the `pluginDirs` setting in `settings.json`.

### Option 2: Manual Installation

```bash
mkdir -p ~/.claude/plugins/
cp -r plugins/perform-analysis ~/.claude/plugins/
```

Then restart Claude Code.

## Configuration

No configuration needed. The skill automatically activates when you request data analysis tasks.

## Integration with Other Skills

This skill is designed to work with:

### learn-tool (included)
When the analysis requires a tool Claude hasn't used before, it will automatically:
1. Search for documentation
2. Install the tool
3. Run sanity checks
4. Then use it for your analysis

### sanity-check-data (to be created)
When working with new or unfamiliar data, Claude will:
1. Validate data structure
2. Check for obvious issues
3. Understand data format
4. Then proceed with analysis

### submit-O2-job (to be created)
For computationally intensive analyses:
1. Detect when cluster is appropriate
2. Create SLURM submission script
3. Submit job
4. Monitor and retrieve results

## Analysis Types Supported

### Statistical Tests
- t-tests, ANOVA, chi-square
- Correlation and regression
- Non-parametric tests
- Multiple testing correction

### Machine Learning
- Classification and regression models
- Cross-validation
- Feature selection
- Model evaluation

### Bioinformatics
- Differential expression (RNA-seq, microarray)
- Enrichment analysis
- Sequence analysis
- Variant calling

### Time Series
- Trend analysis
- Forecasting
- Change point detection

### General Data Science
- Exploratory data analysis
- Summary statistics
- Data visualization
- Hypothesis testing

## Best Practices

### Be Specific About Questions

❌ "Analyze this data"
✅ "Test if treatment increases expression compared to control"

❌ "Look at the correlation"
✅ "Calculate the correlation between age and disease severity"

### Provide Context When Possible

Including context helps Claude make better choices:
- "We expect to see a positive correlation based on prior studies"
- "This is exploratory, we're not sure what to expect"
- "We've seen effect sizes around 0.5 in similar experiments"

### Organize Your Data

Claude works best when:
- Data files are in consistent locations (data/, results/, etc.)
- File names are descriptive
- Metadata is clearly structured
- README files explain data contents

### Review Plans Before Long Analyses

For complex analyses, Claude will present a plan. Review it before execution:
- Confirm the approach makes sense
- Suggest modifications if needed
- Verify assumptions are correct

## Troubleshooting

### Skill Not Activating

**Symptoms**: Claude doesn't follow the 8-step process

**Solutions**:
- Use explicit trigger phrases: "perform analysis", "run experiment"
- Restart Claude Code session to load plugins
- Verify pluginDirs in settings.json

### Missing Data or Tools

**Symptoms**: Analysis fails due to missing resources

**Expected Behavior**: Claude should detect this in Step 3 and ask you

**If it doesn't**:
- Provide file paths explicitly
- Mention if tools need installation
- Use learn-tool skill separately if needed

### Unexpected Results

**What Claude Should Do** (Step 2 & 7):
- Compare results to expectations
- Investigate discrepancies
- Document what was found

**What You Should Do**:
- Review the "Choices Made" section
- Check if assumptions were violated
- Verify data is correct

### Performance Issues

For slow analyses:
- Claude should estimate time in Step 5
- Consider using O2 cluster for large jobs
- Break analysis into smaller steps if possible

## Customization

You can modify `skills/perform-analysis/SKILL.md` to:
- Adjust the 8-step process for your workflow
- Add domain-specific guidelines
- Include preferred methods or tools
- Add custom quality checks

## Example Workflows

### Genomics Research
1. Differential expression → Find significant genes
2. Enrichment analysis → Identify affected pathways
3. Visualization → Create publication figures

### Clinical Data Analysis
1. Summary statistics → Describe cohort
2. Statistical tests → Compare outcomes
3. Regression → Identify risk factors

### Machine Learning
1. EDA → Understand features
2. Model training → Build predictor
3. Validation → Assess performance

Each workflow benefits from the systematic approach ensuring nothing is missed.

## Version

Current version: 1.0.0

## Future Enhancements

Planned additions:
- sanity-check-data skill integration
- submit-O2-job skill integration
- Templates for common analysis types
- Automated quality control checks
- Integration with lab notebooks

## Contributing

Feel free to customize this skill for your research domain. Common modifications:
- Add domain-specific analysis types
- Include standard quality control procedures
- Customize visualization styles
- Add project-specific file organization conventions
