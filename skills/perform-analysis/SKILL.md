---
name: perform-analysis
description: This skill should be used when the user asks to "perform an analysis", "run an experiment", "analyze data", "test a hypothesis", "compute statistics", "run a model", "compare groups", "generate figures", "run a pipeline", or requests any data analysis task that involves using data and methods to answer a scientific or analytical question.
version: 1.0.0
---

# Perform Analysis Skill

Systematic framework for data analyses and experiments, from planning through execution to results presentation.

## AFK Mode Behavior

At the start, check `~/.claude/behavior.conf` for the `AFK` flag.

**When AFK=true:**
- Skip plan approval confirmations - proceed directly with the plan
- Make reasonable methodological choices without asking (test selection, correction method, parameters, etc.)
- Document all choices and reasoning in Step 7
- Respect current tool permissions (sandbox settings) - only use allowed tools
- Attempt autonomous troubleshooting on errors (max 2 attempts), then stop and report what failed
- Only pause for: missing data files, ambiguous core requirements, critical irreversible decisions

**When AFK=false (default):**
- Follow standard workflow with confirmation checkpoints
- Ask for clarification on methodological choices when multiple valid options exist

## When This Skill Applies

Use when the user wants to:
- Perform a data analysis or statistical test
- Run a computational experiment
- Test a hypothesis with data
- Generate results to answer a research question
- Compute metrics or statistics from datasets
- Run models or simulations

An analysis involves **Data** + **Methods** + **Question**.

## Analysis Process

Follow these 8 steps systematically:

### Step 1: Understand the Motivation

**Before doing anything else, understand WHY this question is being asked.**

- What decision or insight depends on the answer?
- How will the results be used or interpreted?
- What would different possible outcomes mean?

**If unclear (and AFK=false):** Ask "What are you hoping to learn?" or "How will you use these results?"

### Step 2: Set Expectations

**Determine if there is an expected result.**

- Is there a predicted outcome?
- How confident should we be?
- What would a null result mean?

State expectations explicitly:
```
Based on [prior results/theory], I expect [predicted result].
Confidence: [High/Medium/Low]
```

### Step 3: Verify Resources

**Ensure access to all required data and tools.**

#### Data
1. Identify needed datasets
2. Locate files (search common locations if unknown)
3. Verify access and format
4. **For new/unfamiliar data:** Use the **sanity-check-data** skill

#### Tools
1. Identify required tools/packages
2. Check availability and versions
3. **For unfamiliar tools:** Use the **learn-tool** skill

### Step 4: Make a Plan

**Create a plan proportional to complexity.**

**Simple** (single command):
```
1. Run: grep "pattern" data.txt | wc -l
2. Report count
```

**Moderate** (script execution):
```
1. Load data
2. Run statistical test
3. Compute effect size
4. Create visualization
5. Report results
```

**Complex** (new development):
```
1. Implement new method with tests
2. Validate on toy example
3. Run on full dataset
4. Generate outputs
```

**If AFK=false and plan is complex:** Ask "Does this approach make sense?"

### Step 5: Perform the Analysis

**Execute systematically.**

- Follow the plan step-by-step
- Document as you go
- Save intermediate results for multi-step analyses
- Check results at each step

**For long-running analyses (>30 seconds):**
- Provide time estimates
- Run in background if appropriate
- Check progress periodically

**For O2 cluster jobs:**
- Use **use-o2** skill when: user mentions O2/cluster, needs >16GB RAM, >4 hours runtime, or GPUs

**Error handling:**
- If AFK=true: Attempt autonomous fix (max 2 attempts per problem), then stop and report
- If AFK=false: Report error and ask how to proceed

**Save all outputs:**
- Scripts (even one-liners)
- Intermediate data
- Results, figures, tables

### Step 6: Display Results

**Create clear presentation of results.**

**Choose format:**
- **Quantitative:** Tables or figures
- **Qualitative:** Summary text or lists

**Highlight the key takeaway:**
```
KEY TAKEAWAY: [One clear sentence with actual numbers/statistics and interpretation]
```

Good: "Treatment A increases response by 34% (p < 0.001), indicating strong effect."
Bad: "The analysis showed some interesting patterns."

### Step 7: Document Choices and Challenges

**Explain decisions made and obstacles overcome.**

```
Choices Made:
- Used Welch's t-test because variances were unequal
- Applied Bonferroni correction for 3 comparisons
- Filtered outliers beyond 3 SD (removed 5/200 points)

Challenges:
- Initial analysis failed due to missing values → imputed with median
- Results differed from expected → verified data version, re-ran with correct file
```

**This is especially important in AFK mode** - document all autonomous decisions with reasoning.

### Step 8: List All Created Files

**Provide complete list of generated files:**

```
Files Created:

Scripts:
- scripts/analysis.py - Main analysis script

Results:
- results/output.csv - Statistical results

Figures:
- figures/plot.png - Main visualization
```

## Best Practices

1. **Understand before acting** - Don't jump to code without scientific context
2. **Be explicit about assumptions**
3. **Validate early and often**
4. **Save everything** - Disk is cheap, redoing analyses is expensive
5. **Communicate clearly** - Use STEP headers to show progress
6. **Provide context for results** - Explain what numbers mean

## Integration with Other Skills

- **sanity-check-data**: Invoke when encountering new/unfamiliar data in Step 3
- **learn-tool**: Invoke when needing unfamiliar software
- **use-o2**: Invoke when analysis requires cluster computing

## References

- **[references/examples.md](references/examples.md)**: Complete example workflow and special cases (exploratory vs confirmatory, negative results, reproductions, multi-stage analyses)
