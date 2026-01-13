---
name: perform-analysis
description: This skill should be used when the user asks to "perform an analysis", "run an experiment", "analyze data", "test a hypothesis", "compute statistics", "run a model", "compare groups", "generate figures", "run a pipeline", or requests any data analysis task that involves using data and methods to answer a scientific or analytical question.
version: 1.0.0
---

# Perform Analysis Skill

Systematic framework for data analyses and experiments, from planning through execution to results presentation.

## Behavior Flags

At the start, check `~/.claude/behavior.conf` for behavior flags.

### Environment Detection

Check the `Environment` flag:
- **Environment=O2**: You are on the O2 cluster. For resource-intensive tasks (>16GB RAM, >4 hours, GPUs), use the **use-o2** skill to submit SLURM jobs.
- **Environment=local**: Run analyses locally.

### AFK Mode

Check the `AFK` flag.

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

## Lab Notebook System

Every analysis is recorded in a **lab notebook** (`notebook/analyses/`) for reproducibility and project tracking.

### Notebook Structure

```
project/
├── CLAUDE.md                         # Curated active context (key findings only)
├── notebook/
│   ├── analyses/                     # Analysis logs and scripts
│   │   └── <analysis-name>/
│   │       ├── README.md             # Analysis log (all versions)
│   │       ├── <script>.py           # Analysis scripts (git tracks history)
│   │       └── outputs/              # Results, figures, data
│   ├── data/                         # Dataset documentation
│   │   └── <dataset-name>.md         # Location, source, characteristics, issues
│   ├── software/                     # External software documentation
│   │   └── <software-name>.md        # Installation, docs URL, issues
│   └── methods/                      # Methodological changes to codebase
│       └── YYYY-MM-DD-<description>.md
```

### Analysis Naming

Generate **specific, descriptive names** using acronyms freely. Names should uniquely identify the analysis within the project.

**BAD names** (too generic):
- `survival-analysis`
- `differential-expression`
- `clustering`

**GOOD names** (specific and descriptive):
- `km-survival-by-tp53-mutation-hnscc-tcga`
- `deseq2-hnscc-vs-normal-paired-rnaseq`
- `umap-clustering-immune-infiltrate-subtypes`

If user provides a name, use it. Otherwise, auto-generate based on the analysis details.

### Version Management

- All versions recorded in the **same README.md file**
- Scripts can be modified between versions - git tracks history
- Version numbers increment linearly (v0, v1, v2, ...)

**When to increment version vs. create new analysis:**
- Same question, any revision → new version
- Different question or major methodological change → new analysis

**Piloting:** For analyses expected to take >1 minute, pilot on subset data first. You may need multiple pilots:
- v0: pilot → v1: fix bug, pilot again → v2: full run
- v2: unexpected results → v3: new approach, pilot → v4: full run

### Incremental Writing

**Write to the notebook after EACH step**, not at the end. This captures context while it's fresh and provides recovery points.

## Analysis Process

Follow these 8 steps systematically.

### Step 0: Setup (Notebook & Git)

**Before starting the analysis:**

1. **Retrieve related analyses from notebook:**

   ```bash
   ls -ltr notebook/analyses/ 2>/dev/null || echo "No notebook yet"
   ```

   Scan the list and identify **0-3 related analyses** based on:
   - Similar data types or sources
   - Similar methods or statistical approaches
   - Same research question or related hypotheses
   - Recent analyses in the same project area

   **For each related analysis, read its README.md** to gather:

   | Context Type | How It Helps |
   |--------------|--------------|
   | **Scripts** | Copy/adapt similar analysis scripts |
   | **Resource usage** | Predict O2 partition, memory, runtime |
   | **Findings** | Check if new results are concordant with previous |
   | **Challenges** | Avoid repeating past mistakes |

   If no related analyses exist (new project area), proceed without prior context.

   **Note in your working context** which analyses you referenced and why.

2. **Determine analysis name:**
   - Use user-provided name if given
   - Otherwise, generate specific descriptive name

3. **Initialize notebook directory:**
   ```
   notebook/analyses/<analysis-name>/
   ├── README.md      # Create with header and Motivation section
   └── outputs/       # Create empty directory
   ```

4. **Initialize README.md:**
   ```markdown
   # <Analysis Name (Human Readable)>

   **Analysis ID:** `<analysis-name>`
   **Started:** YYYY-MM-DD
   **Status:** In Progress

   ## Motivation
   [To be filled in Step 1]

   ## Expected Results
   [To be filled in Step 2]

   ## Related Analyses
   [List 0-3 related analyses consulted, or "None - new project area"]
   - `<related-analysis-name>` - [what was useful: script template, resource estimates, etc.]

   ---

   ## v0 (YYYY-MM-DD)

   ### Plan
   [To be filled in Step 4]

   ### Execution Notes
   [To be filled in Step 5]

   ### Findings
   [To be filled in Step 6]

   ### Output Files
   [To be filled in Step 8]

   ### Scripts
   [To be filled in Step 8]
   ```

**Notebook write:** Create the initial README.md structure.

### Step 1: Understand the Motivation

**Before doing anything else, understand WHY this question is being asked.**

- What decision or insight depends on the answer?
- How will the results be used or interpreted?
- What would different possible outcomes mean?

**If unclear (and AFK=false):** Ask "What are you hoping to learn?" or "How will you use these results?"

**Notebook write:** Update the Motivation section in README.md with the context gathered.

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

**Notebook write:** Update the Expected Results section in README.md.

### Step 3: Verify Resources

**Ensure access to all required data and tools.**

#### Data
1. Identify needed datasets
2. Locate files (search common locations if unknown)
3. Verify access and format
4. **For new/unfamiliar data:** Use the **new-data** skill

#### Tools
1. Identify required tools/packages
2. Check availability and versions
3. **For unfamiliar tools:** Use the **new-software** skill

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

**Piloting for long analyses:**

If the analysis is expected to take **>1 minute**, plan a pilot first:
- Use a subset of data (e.g., 100 samples instead of 10,000)
- Or use a small synthetic example
- You may need multiple pilot iterations before the full run
- This catches errors before committing to long runtimes

**If AFK=false and plan is complex:** Ask "Does this approach make sense?"

**Notebook write:** Update the Plan section under the current version in README.md.

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

**For O2 cluster (when Environment=O2):**
- Use **use-o2** skill for resource-intensive tasks (>16GB RAM, >4 hours runtime, or GPUs)
- Submit SLURM jobs and monitor progress periodically

**Error handling:**
- If AFK=true: Attempt autonomous fix (max 2 attempts per problem), then stop and report
- If AFK=false: Report error and ask how to proceed

**Save all outputs:**
- Scripts (even one-liners)
- Intermediate data
- Results, figures, tables

**Script archiving:** Save analysis scripts to `notebook/analyses/<analysis-name>/`. Scripts can be modified between versions - git commits preserve history for reproducibility.

**Notebook write:** Update Execution Notes with any issues encountered, deviations from plan, or important observations during execution.

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

**Check concordance with related analyses:**

If you referenced related analyses in Step 0, compare current findings:
- Are results consistent with previous observations?
- If discordant, note possible explanations (different data, methods, or genuine new finding)
- Flag unexpected discordances for user attention

```
CONCORDANCE: [Consistent with / Differs from] <related-analysis>
[Brief explanation if discordant]
```

**Notebook write:** Update the Findings section with results, key takeaway, concordance notes, and any figures/tables.

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

**Notebook write:** Add choices and challenges to the Execution Notes section.

### Step 8: Finalize and Commit

**Complete the notebook entry and git workflow.**

1. **List all created files:**
   ```
   Files Created:

   Scripts:
   - notebook/analyses/<name>/run_analysis.py - Main analysis script

   Outputs:
   - notebook/analyses/<name>/outputs/results.csv - Statistical results
   - notebook/analyses/<name>/outputs/figure.png - Main visualization
   ```

2. **Update notebook README.md:**
   - Fill in Output Files section with paths and descriptions
   - Fill in Scripts section with script filenames
   - Update Status from "In Progress" to "Complete"

3. **Git commit:**
   ```bash
   git add notebook/analyses/<analysis-name>/
   git commit -m "analysis: <analysis-name> v0 - <brief description of what was done>"
   ```

4. **Evaluate for CLAUDE.md update:**

   Ask: Is this finding important enough for CLAUDE.md?

   **Add to CLAUDE.md if:**
   - Important finding that changes project understanding
   - Something now works that didn't before
   - Key methodological decision affecting future work

   **Don't add if:**
   - Routine analysis (notebook is sufficient)
   - Intermediate or exploratory result
   - Not actionable for future work

   If adding, write a concise summary (2-4 sentences) of the key finding and its implications.

**Notebook write:** Final updates to Output Files and Scripts sections. Mark Status as Complete.

## Creating a New Version

When revising an analysis (pilot → full run, fixing a mistake, updating parameters):

1. **Modify scripts as needed** - git tracks history for reproducibility

2. **Add new version section to README.md:**
   ```markdown
   ---

   ## v1 (YYYY-MM-DD)

   ### What Changed
   [Why this revision? Pilot → full run? Fixed error? New parameters?]

   ### Execution Notes
   [New execution details]

   ### Findings
   [Updated results]

   ### Output Files
   [Updated paths]
   ```

3. **Git commit:**
   ```bash
   git add notebook/analyses/<analysis-name>/
   git commit -m "analysis: <analysis-name> v1 - <what changed>"
   ```

**Common version patterns:**
- v0: pilot → v1: full run
- v0: pilot → v1: fix bug, pilot again → v2: full run
- v0: full run → v1: unexpected results, change approach → v2: pilot new approach → v3: full run

## Best Practices

1. **Understand before acting** - Don't jump to code without scientific context
2. **Be explicit about assumptions**
3. **Validate early and often**
4. **Save everything** - Disk is cheap, redoing analyses is expensive
5. **Communicate clearly** - Use STEP headers to show progress
6. **Provide context for results** - Explain what numbers mean
7. **Write to notebook incrementally** - Don't wait until the end

## Integration with Other Skills

- **new-data**: Invoke when encountering new/unfamiliar data in Step 3
- **new-software**: Invoke when needing unfamiliar software
- **use-o2**: Invoke when analysis requires cluster computing
- **update-notebook**: Invoke to sync notebook when work was done outside Claude Code

## References

- **[references/examples.md](references/examples.md)**: Complete example workflow and special cases (exploratory vs confirmatory, negative results, reproductions, multi-stage analyses)
