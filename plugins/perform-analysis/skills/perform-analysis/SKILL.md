---
name: perform-analysis
description: This skill should be used when the user asks to "perform an analysis", "run an experiment", "analyze data", "test a hypothesis", "compute statistics", "run a model", or requests any data analysis task that involves using data and methods to answer a scientific or analytical question.
version: 1.0.0
---

# Perform Analysis Skill

This skill provides a systematic framework for performing data analyses and experiments, ensuring thoroughness from planning through execution to presentation of results.

## When This Skill Applies

Use this skill when the user wants to:
- Perform a data analysis or statistical test
- Run a computational experiment
- Test a hypothesis with data
- Generate results to answer a research question
- Compute metrics or statistics from datasets
- Run models or simulations

An analysis is a specific experiment that involves:
- **Data**: One or more datasets
- **Methods**: Tools, scripts, models, or statistical procedures
- **Question**: A specific question to answer or hypothesis to test

## Analysis Process

Follow these steps systematically for every analysis:

### Step 1: Understand the Motivation

**Before doing anything else, ensure you understand WHY this question is being asked.**

Ask yourself (and clarify with the user if needed):
- What is the scientific or practical motivation for this analysis?
- What decision or insight depends on the answer?
- How will the results be used or interpreted?
- What would different possible outcomes mean?

Understanding the motivation helps you:
- Choose appropriate methods
- Set correct significance thresholds
- Interpret edge cases correctly
- Prioritize what to report

**If the motivation is unclear, ask the user:**
- "What are you hoping to learn from this analysis?"
- "How will you use these results?"
- "What motivated this particular question?"

### Step 2: Set Expectations

**Determine if there is an expected result and your confidence in it.**

Before running the analysis, establish:
- **Is there a predicted outcome?** (e.g., "We expect to see a positive correlation")
- **How confident should we be?** (e.g., "This should be very strong" vs. "This is exploratory")
- **What would a null result mean?** (e.g., "No effect might indicate a problem with the data")

This helps with:
- **Quality control**: Unexpected results might indicate bugs or data issues
- **Interpretation**: Knowing if results are surprising or expected
- **Statistical power**: Understanding if the analysis is adequately powered

**State your expectations explicitly:**
```
Based on [prior results/theory/domain knowledge], I expect to see [predicted result].
Confidence: [High/Medium/Low]
Reasoning: [Why you expect this]
```

**If uncertain, ask the user:**
- "Do you have any expectations about what the results should show?"
- "Have similar analyses been done before? What did they find?"
- "Is this exploratory or confirmatory?"

### Step 3: Verify Resources

**Ensure you have access to all required data and tools.**

#### Data Requirements

For each dataset needed:
1. **Identify the dataset**: Name and purpose
2. **Locate the file**: Ask for the path if unknown
3. **Verify access**: Check the file exists and is readable
4. **Understand format**: File type, structure, size
5. **Check validity**: Quickly inspect (first few lines, dimensions, etc.)

**If you don't know where data is located:**
- Search common locations (data/, results/, input/, etc.)
- Use Grep/Glob to find files matching expected patterns
- **Ask the user**: "Where is the [dataset name] located?"

**For new or unfamiliar data:**
- Use the **sanity-check-data skill** (if available)
- Perform basic validation (check for missing values, outliers, expected ranges)
- Verify data matches your understanding of what it should contain

#### Tool Requirements

For each tool/method needed:
1. **Identify required tools**: Software, packages, scripts
2. **Check availability**: Verify installation/location
3. **Verify versions**: Ensure compatible versions
4. **Test functionality**: Quick test run if uncertain

**Common tool categories:**
- **Programming languages**: Python, R, Julia, MATLAB, etc.
- **Packages/libraries**: numpy, pandas, ggplot2, etc.
- **Custom scripts**: Project-specific analysis code
- **System utilities**: awk, sed, jq, etc.

**For new or unfamiliar tools:**
- Use the **learn-tool skill** to install and understand them
- Run sanity checks to verify they work correctly
- Check documentation for the specific methods you'll use

**If unsure about tools or data:**
```
I need the following for this analysis:

Data:
- [Dataset 1]: [Purpose] - Location: [path or UNKNOWN]
- [Dataset 2]: [Purpose] - Location: [path or UNKNOWN]

Tools:
- [Tool 1]: [Purpose] - Status: [installed/needs installation/UNKNOWN]
- [Tool 2]: [Purpose] - Status: [installed/needs installation/UNKNOWN]

Can you confirm these locations and let me know if I should install any missing tools?
```

### Step 4: Make a Plan

**Create a plan to answer the question. This can range from very simple to very complex.**

#### Simple Plan (Single Command)
For straightforward analyses:
```
Plan:
1. Run: grep "pattern" data.txt | wc -l
2. Report the count to user
```

#### Moderate Plan (Script Execution)
For standard analyses:
```
Plan:
1. Load data from data.csv
2. Run t-test using scipy.stats.ttest_ind
3. Compute effect size
4. Create violin plot
5. Report results with interpretation
```

#### Complex Plan (New Development)
For analyses requiring new code:
```
Plan:
1. Add new feature to existing_tool.py:
   - Implement method X
   - Add unit tests
   - Verify correctness with toy example
2. Run analysis using new feature on full dataset
3. Generate summary statistics
4. Create visualization
5. Compare to baseline results
```

#### Very Complex Plan (Multi-Step Pipeline)
For comprehensive analyses:
```
Plan:
1. Data preparation:
   - Merge datasets A and B
   - Filter outliers
   - Normalize values
2. Quality control:
   - Check for batch effects
   - Validate assumptions (normality, homoscedasticity)
3. Primary analysis:
   - Fit model
   - Assess significance
4. Sensitivity analysis:
   - Test with different parameters
   - Check robustness
5. Visualization:
   - Main effect plot
   - Supplementary diagnostic plots
6. Reporting:
   - Summarize findings
   - Document all parameters used
```

**Plan considerations:**
- Break into logical steps
- Include validation/testing for new code
- Note dependencies between steps
- Identify potential failure points

**State the plan clearly and ask for confirmation if complex:**
```
Here's my plan to answer your question:
[List steps]

Does this approach make sense, or would you like me to adjust anything?
```

### Step 5: Perform the Analysis

**Execute the plan systematically.**

#### General Execution Guidelines

1. **Follow the plan step-by-step**
2. **Document what you're doing** as you go
3. **Save intermediate results** if the analysis is multi-step
4. **Check results at each step** before proceeding
5. **Handle errors gracefully** and explain issues

#### For Quick Analyses (< 30 seconds)
- Run immediately
- Show commands and output
- Proceed directly to results

#### For Long-Running Analyses (> 30 seconds)

**Provide time estimates:**
```
This analysis will take approximately [X minutes/hours] because:
- [Reason 1: dataset size, computation complexity, etc.]
- [Reason 2]

I'll run this in the background and check on progress periodically.
```

**For jobs that take minutes:**
- Run in background if appropriate
- Use sleep commands to wait
- Check progress periodically
- Update user on status

**Example:**
```bash
# Start analysis
python long_running_analysis.py &
PID=$!

# Wait approximately expected time
echo "Analysis started (PID: $PID). Expected completion in ~5 minutes."
sleep 300

# Check if complete
if ps -p $PID > /dev/null; then
    echo "Still running, waiting longer..."
    wait $PID
fi
```

#### For O2 Cluster Jobs

**If the analysis should run on the O2 compute cluster:**
- Use the **submit-O2-job skill** (if available)
- Create appropriate SLURM submission script
- Specify resources (memory, time, CPUs)
- Submit job and monitor status
- Retrieve results when complete

**Indicators that O2 should be used:**
- User mentions "O2", "cluster", "submit", "SLURM"
- Analysis requires > 16GB memory
- Analysis requires > 4 hours runtime
- Analysis needs GPUs
- User has previously used O2 for similar tasks

#### Using Other Skills During Analysis

**When you encounter new, unfamiliar data:**
- Invoke the **sanity-check-data skill**
- Verify data integrity
- Understand data structure
- Check for obvious issues

**When you need a tool you haven't used:**
- Invoke the **learn-tool skill**
- Install and understand the tool
- Run basic tests
- Then use it for the analysis

**Example:**
```
I need to use the 'bedtools' package for this genomic analysis, but I haven't used it before.
Let me first learn how to use bedtools...

[learn-tool skill activates]

Now that bedtools is installed and I understand the basics, I'll proceed with the analysis.
```

#### Saving Results

**Always save important outputs:**
- **Scripts**: Save all code used (even one-liners, put them in a file)
- **Intermediate data**: Save processed/filtered data
- **Results**: Save statistical outputs, model objects, etc.
- **Figures**: Save plots in publication-ready formats
- **Tables**: Save summary tables as CSV/TSV

**Naming conventions:**
- Use descriptive names: `enrichment_analysis_results.txt` not `output.txt`
- Include dates if relevant: `model_fit_2026-01-10.rds`
- Use consistent extensions: `.png` for figures, `.csv` for tables

### Step 6: Display Results

**Create a clear, focused presentation of the results.**

#### Create Display Items

**Choose the appropriate format:**

**For Quantitative Results:**
- **Table**: Multiple metrics, comparisons, or summary statistics
- **Figure**: Trends, distributions, relationships, or comparisons

**For Qualitative Results:**
- **Summary text**: Key findings in prose
- **Lists**: Ranked items, categories, or features

#### Table Format

Use markdown tables for clarity:

```markdown
| Comparison | Statistic | P-value | Effect Size |
|------------|-----------|---------|-------------|
| A vs B     | t = 3.45  | 0.001   | d = 0.67    |
| A vs C     | t = 1.23  | 0.22    | d = 0.24    |
| B vs C     | t = -2.10 | 0.04    | d = -0.43   |
```

#### Figure Guidelines

**Create informative visualizations:**
- Clear labels on axes
- Legible font sizes
- Appropriate colors (colorblind-friendly if possible)
- Informative title or caption
- Save in high resolution (300 DPI for publication)

**Common plot types:**
- **Distributions**: Histogram, violin plot, box plot
- **Relationships**: Scatter plot, line plot
- **Comparisons**: Bar plot, grouped plots
- **Trends**: Time series, line plots with confidence intervals

**Always show the figure path:**
```
Created figure: results/comparison_plot_2026-01-10.png
```

#### Highlight the Key Takeaway

**After presenting results, state the single most important finding:**

```
KEY TAKEAWAY: [One clear sentence summarizing the main result]
```

**Make it:**
- **Specific**: Include actual numbers/statistics
- **Clear**: No jargon, straightforward interpretation
- **Actionable**: What does this mean for the question asked?

**Examples:**
- ✅ "Treatment A increases response by 34% compared to control (p < 0.001), indicating a strong therapeutic effect."
- ✅ "No significant correlation was found between X and Y (r = 0.05, p = 0.67), suggesting these factors are independent."
- ❌ "The analysis showed some interesting patterns." (Too vague)
- ❌ "P-value was 0.03." (Not interpreted)

### Step 7: Document Choices and Challenges

**Explain any decisions made or obstacles overcome during the analysis.**

#### Choices Made

**Document methodological decisions:**
- Why you chose method A over method B
- Parameter settings and why they were chosen
- Data filtering/preprocessing decisions
- Statistical test selection rationale

**Example:**
```
Choices Made:
- Used Welch's t-test instead of Student's t-test because variances were unequal (Levene's test: p = 0.02)
- Filtered outliers beyond 3 SD from mean (removed 5/200 data points)
- Used Bonferroni correction for multiple testing (3 comparisons, α = 0.05/3 = 0.017)
```

#### Challenges Overcome

**Describe problems and solutions:**
- Data issues discovered and how they were addressed
- Technical errors and their fixes
- Unexpected results and how they were investigated
- Computational challenges and workarounds

**Example:**
```
Challenges:
- Initial analysis failed because of missing values in column X
  → Solution: Imputed using median value of available data
- Figure generation crashed due to memory limit
  → Solution: Created plot in chunks and combined
- Results differed from expected
  → Investigation: Checked data version, found we were using outdated file
  → Resolution: Re-ran with correct data, results now match expectations
```

#### Alternative Approaches Considered

**If relevant, mention other approaches:**
```
Alternative approaches considered:
- Could have used bootstrapping instead of parametric test
- Considered PCA for dimensionality reduction, but decided against due to interpretability needs
```

**This transparency helps:**
- Future reproducibility
- Understanding analytical choices
- Learning from the process
- Debugging if results are unexpected

### Step 8: List All Created Files

**Provide a complete list of all files generated during the analysis.**

#### File Categories

Organize by type:

```
Files Created:

Scripts:
- scripts/enrichment_analysis.py - Main analysis script
- scripts/prepare_data.sh - Data preprocessing

Data Files:
- data/processed/filtered_samples.csv - QC-filtered dataset (1,234 samples)
- results/enrichment_results.tsv - Complete statistical results

Figures:
- figures/enrichment_volcano_plot.png - Volcano plot of enrichment scores
- figures/diagnostic_qq_plot.png - QQ plot for normality check

Tables:
- tables/summary_statistics.csv - Summary statistics by group
- tables/significant_results.csv - Results passing FDR < 0.05

Logs:
- logs/analysis_2026-01-10.log - Complete analysis log
```

#### File Descriptions

**For each file, include:**
- **Path**: Full or relative path
- **Purpose**: What the file contains
- **Details**: Size, number of rows, format, etc. (if relevant)

#### Make Files Easy to Find

**Use consistent naming and organization:**
- Group by type (scripts/, figures/, results/, data/)
- Use descriptive names
- Include dates if files might be regenerated
- Note which files are inputs vs. outputs

**Provide commands to view key files:**
```
To view the main results:
  cat results/enrichment_results.tsv | head

To open the main figure:
  open figures/enrichment_volcano_plot.png
```

## Complete Example Workflow

Here's how the full process looks for a typical analysis:

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

## Best Practices

### 1. Always Understand Before Acting
Don't jump straight to running code. Understand the scientific context first.

### 2. Be Explicit About Assumptions
State what you're assuming about the data, methods, and expected results.

### 3. Validate Early and Often
Check data and intermediate results before proceeding to expensive computations.

### 4. Save Everything
Scripts, results, figures, logs. Disk space is cheap, redoing analyses is expensive.

### 5. Communicate Clearly
Use clear section headers (STEP 1, STEP 2, etc.) to show progress through the workflow.

### 6. Handle Uncertainty Gracefully
If you're unsure about data location, expected results, or methods, ask before proceeding.

### 7. Provide Context for Results
Don't just show numbers. Explain what they mean and why they matter.

### 8. Document the Journey
The choices and challenges are often as important as the final results.

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

## Integration with Other Skills

This skill works together with:

- **learn-tool**: When you need to use unfamiliar software
- **sanity-check-data**: When working with new or unfamiliar datasets
- **submit-O2-job**: When analysis requires cluster computing

When these situations arise, invoke the appropriate skill rather than trying to handle everything yourself.
