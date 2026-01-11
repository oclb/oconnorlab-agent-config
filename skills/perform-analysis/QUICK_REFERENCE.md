# Perform Analysis - Quick Reference

## The 8-Step Framework

When you ask Claude to perform an analysis, it will systematically follow these steps:

### 1️⃣ Understand Motivation
**What**: Clarify why the question is being asked
**Output**: Statement of scientific/practical motivation
**Example**: "You want to test if treatment improves outcomes to guide clinical decisions"

### 2️⃣ Set Expectations
**What**: Determine expected results and confidence level
**Output**: Predicted outcome and reasoning
**Example**: "Expected: Moderate positive effect. Confidence: Medium (based on pilot data)"

### 3️⃣ Verify Resources
**What**: Ensure data and tools are available
**Output**: Checklist of data files and tools with status
**Example**:
```
✓ Data: experiment_data.csv (found)
✓ Tool: Python scipy (installed)
? Missing: sample_metadata.csv (need location)
```

### 4️⃣ Make a Plan
**What**: Create step-by-step analysis plan
**Output**: Numbered list of steps
**Example**:
```
1. Load data and merge with metadata
2. Run linear regression
3. Check assumptions (normality, homoscedasticity)
4. Create scatter plot with regression line
5. Report results
```

### 5️⃣ Perform Analysis
**What**: Execute the plan
**Output**: Running code/commands with progress updates
**Note**: Includes time estimates for long-running jobs

### 6️⃣ Display Results
**What**: Present findings with visualization
**Output**:
- Table or figure
- **KEY TAKEAWAY**: One-sentence main finding
**Example**: "KEY TAKEAWAY: Treatment increases recovery rate by 23% (p<0.001, 95% CI: 18-28%)"

### 7️⃣ Document Choices and Challenges
**What**: Explain decisions and obstacles
**Output**:
- **Choices Made**: Why certain methods were chosen
- **Challenges**: Problems encountered and solutions
**Example**:
```
Choices:
- Used robust regression (data had outliers)

Challenges:
- 3 samples had missing values → imputed using group median
```

### 8️⃣ List Files Created
**What**: Catalog all outputs
**Output**: Organized list by type
**Example**:
```
Scripts: scripts/regression_analysis.py
Data: results/cleaned_data.csv
Figures: figures/regression_plot.png
Tables: tables/summary_stats.csv
```

## Trigger Phrases

Use these to activate the skill:
- "Perform an analysis of..."
- "Run an experiment to test..."
- "Analyze the data in..."
- "Test if X is correlated with Y"
- "Compute statistics for..."
- "Run a model to predict..."

## Integration with Other Skills

The perform-analysis skill automatically invokes:

### learn-tool
**When**: Analysis needs unfamiliar tool
**Example**: "I see this needs bedtools. Let me learn how to use it first..."

### sanity-check-data (future)
**When**: Working with new/unfamiliar data
**Example**: "This is new data. Let me validate it first..."

### submit-O2-job (future)
**When**: Analysis requires cluster computing
**Example**: "This will need substantial compute. Submitting to O2..."

## Example Sessions

### Quick Analysis
```
You: Test if variable X predicts Y in data.csv

Claude follows 8 steps:
1. Understand: Checking predictive relationship
2. Expect: Unknown, exploratory analysis
3. Verify: ✓ data.csv found, ✓ Python installed
4. Plan: Load → Linear regression → Plot → Report
5. Perform: [runs analysis]
6. Display: [regression plot] KEY TAKEAWAY: X strongly predicts Y (R²=0.67)
7. Choices: Linear regression (relationship appeared linear)
8. Files: scripts/regression.py, figures/xy_plot.png
```

### Complex Analysis
```
You: Perform differential expression analysis on the RNA-seq data

Claude follows 8 steps:
1. Understand: Identify gene expression changes
2. Expect: Some differential genes based on strong treatment
3. Verify: Need count matrix location → you provide path
            Need DESeq2 → learns and installs it
4. Plan: Filter low counts → DESeq2 → FDR correction → Plots → Tables
5. Perform: ~5 min estimate → runs with updates
6. Display: [volcano plot] KEY TAKEAWAY: 1,247 DE genes, enriched in immune pathways
7. Choices: DESeq2 (best for counts), FDR<0.05 threshold
   Challenges: 2 outlier samples excluded
8. Files: [Lists 8 files: scripts, data, 3 figures, 2 tables, log]
```

## Tips for Best Results

### Be Specific
❌ "Analyze this"
✅ "Test if treatment reduces symptoms compared to placebo"

### Provide Context
✅ "Based on pilot data, we expect small effect size (~0.3)"
✅ "This is exploratory, no strong prior"
✅ "Previous similar studies found X"

### Organize Your Files
- Keep data in `data/` or similar
- Use descriptive filenames
- Include README files

### Review the Plan
For complex analyses, check Step 4 before execution:
- Confirm approach is appropriate
- Suggest modifications
- Verify assumptions

## Common Analysis Types

### Statistics
- T-tests, ANOVA, chi-square
- Correlation, regression
- Non-parametric tests
- Multiple testing correction

### Machine Learning
- Classification/regression
- Cross-validation
- Feature importance
- Model comparison

### Bioinformatics
- Differential expression
- Enrichment analysis
- Sequence analysis
- Variant calling

### Time Series
- Trend detection
- Forecasting
- Change points

## What Makes This Different

Without the skill, Claude might:
- Jump straight to code
- Miss important context
- Not validate data exists
- Skip documentation
- Not explain choices

With the skill, Claude:
- ✅ Understands why before what
- ✅ Sets clear expectations
- ✅ Validates everything first
- ✅ Creates systematic plan
- ✅ Documents entire process
- ✅ Highlights key findings
- ✅ Lists all outputs

## Next Steps After Analysis

The skill sets you up for:
1. **Reproducibility**: All scripts and steps documented
2. **Follow-up**: Clear understanding of what was done
3. **Presentation**: Figures and key takeaways ready
4. **Validation**: Choices explained for peer review
5. **Extension**: Files organized for additional analyses

## Customization

Edit `skills/perform-analysis/SKILL.md` to:
- Add domain-specific steps
- Include standard quality checks
- Specify preferred methods
- Add institutional requirements (e.g., always use certain tools)

## Getting Help

If the analysis doesn't go as expected:
1. Check Step 3: Were all resources verified?
2. Review Step 7: What challenges were encountered?
3. Examine Step 8: Are all files present?
4. Re-run with modified approach based on what was learned
