# Teaching Mode Plugin

A Claude Code skill that transforms Claude into an educational assistant, explaining not just **what** is being done but **how to replicate** it and **why** choices were made.

## What It Does

When activated, teaching mode augments any task with comprehensive educational explanations:

- **Software tasks**: Explain code structure, design patterns, library choices
- **Analysis tasks**: Provide replication steps, justify statistical decisions
- **Research tasks**: Define jargon, provide background, build understanding progressively
- **Focused topics**: Deep-dive explanations of specific concepts

## Core Philosophy

### Learning by Doing
Demonstrate + Explain. Don't just complete the task—teach the user to do it themselves.

### Focus on Decision Points
Explain **why** choices were made, not just what was done.

### Calibrate to Audience
Assume beginner-level unless context suggests otherwise.

### Distinguish Obvious from Non-Obvious
- ❌ Don't explain: for-loops, basic syntax
- ✅ Do explain: algorithm choices, library trade-offs, design patterns

## When to Use

Activate teaching mode by asking:
- "Teach me how to..."
- "Explain how to do this"
- "Show me how to replicate..."
- "Walk me through this"
- "Enable teaching mode and write..."
- "I want to learn about..."

## What Gets Explained

### Software Development

#### Code Structure (non-trivial)
```python
# TEACHING MODE EXPLANATION:
# Separated into three functions:
# 1. load_data() - handles I/O (separates concerns)
# 2. process_data() - pure transformation (easier to test)
# 3. main() - orchestrates workflow
# Follows Single Responsibility Principle
```

#### Design Patterns (non-obvious)
```python
# TEACHING MODE EXPLANATION:
# Using decorator pattern to add logging without modifying
# the original function. Keeps concerns separated.

@log_execution_time
def expensive_computation(data):
    ...
```

#### Library Choices (non-default)
```python
# TEACHING MODE EXPLANATION:
# Using polars instead of pandas because:
# 1. 5-10x faster for 100M rows
# 2. Lazy evaluation enables query optimization
# 3. Better memory efficiency with Arrow format
```

#### Algorithm Choices
```python
# TEACHING MODE EXPLANATION:
# Using set instead of list for lookups:
# - List: O(n) lookup
# - Set: O(1) lookup
# With 10,000 items, 0.001s vs 10s difference
```

### Data Analysis

#### Complete Replication Instructions
```
HOW TO REPLICATE:

Required:
- Python 3.9+, pandas 2.0+, scipy 1.9+
Install: pip install pandas scipy

Data format:
- CSV with columns: sample_id, condition, measurement

Steps:
1. Load: df = pd.read_csv('data.csv')
2. Filter: cases = df[df['condition'] == 'case']['measurement']
3. Test: t_stat, p = ttest_ind(cases, controls, equal_var=False)
4. Interpret: [guide to interpretation]
```

#### Decision Justification
```
DECISION: Welch's t-test (not Student's)

Reasoning:
- Variances unequal (Levene's p=0.03)
- Welch's doesn't assume equal variance
- More conservative (reduces false positives)
- Trade-off: Slightly less power if variances equal

When to use which:
- Equal variances + equal n → Student's
- Unequal variances or n → Welch's (safer default)
```

#### Interpretation Guidance
```
RESULT: t=3.45, p=0.001, d=0.67

Meaning:
- Significant: p<0.05 (unlikely by chance)
- Effect size d=0.67: Medium-large effect
- Practical: ~8 points difference on 100-point scale

Limitations:
- Assumes normality (checked, OK)
- Assumes independence (violated if repeated measures)
- Correlation ≠ causation
```

### Research Tasks

#### Progressive Explanation (Beginner-Friendly)
```
LEVEL 1 (Simple):
GWAS finds genetic variants associated with traits.

LEVEL 2 (More Detail):
GWAS tests millions of variants. Most don't directly cause
the trait—they're correlated (LD) with causal variants.

LEVEL 3 (Technical):
Test statistic depends on:
1. Variant's own effect
2. Effects of nearby causal variants (via LD)
3. LD structure

This creates: E[χ²] = 1 + N·h²·ℓ
(Basis of LD Score Regression)
```

#### Jargon Defined Before Use
❌ "We used LDSC to partition heritability"

✅ "We used LD Score Regression (LDSC), a method that estimates
heritability from GWAS summary statistics by relating association
strength to linkage disequilibrium patterns."

## Example Outputs

### Software Task: Fibonacci Function

```
TEACHING MODE: Fibonacci Implementation

APPROACH OPTIONS:
1. Recursive: Elegant but O(2^n) - slow
2. Dynamic Programming: O(n) - good balance
3. Matrix exponentiation: O(log n) - fastest for huge n

DECISION: Using memoization
Why: Best balance of clarity and efficiency

IMPLEMENTATION:
[Code with explanatory comments at decision points]

IMPORTANT PITFALL:
Mutable default argument (memo={}) persists across calls.
This is DESIRED here (caching), but usually dangerous:

def bad_function(items=[]):  # DON'T DO THIS
    items.append(1)
    return items

bad_function()  # [1]
bad_function()  # [1, 1] - unexpected!

EXECUTION TRACE:
fibonacci(5)
├─ fibonacci(4) [computed]
│  ├─ fibonacci(3) [computed]
│  │  └─ fibonacci(2) [computed]
│  └─ fibonacci(2) [from cache] ← speedup!
└─ fibonacci(3) [from cache] ← speedup!

WHEN TO USE WHICH:
- Small n (<100): Either approach fine
- Medium (100-1000): Memoization or iterative
- Large (>1000): Iterative (avoid stack limit)
```

### Analysis Task: Differential Expression

```
TEACHING MODE: Differential Expression Analysis

BACKGROUND:
- RNA-seq: Count reads mapping to genes
- DE: Genes with different expression between conditions
- Why not t-test: Counts aren't normally distributed
- Proper: Use count-specific methods (DESeq2)

PIPELINE:

Step 1: Load data
  counts = pd.read_csv('counts.csv')
  Why separate files: Keeps data organized

Step 2: Filter low counts
  keep = (counts >= 10).sum(axis=1) >= 10

  DECISION: Why filter?
  - Low-count genes have unreliable estimates
  - Rarely achieve significance anyway
  - Reduces multiple testing burden

  Parameters: ≥10 counts in ≥10 samples
  Alternatives: Some use ≥5 or ≥15
  Result: Kept 15,432/20,000 genes (77%)

[Steps 3-6 with similar detail...]

RESULTS:
- 1,247 DE genes (8.1%)
- 623 up, 624 down (balanced)

HOW TO REPLICATE:
[Complete working script]
[Expected runtime: 2-3 min]

COMMON ISSUES:
1. "Size factors all 1" → Check normalization
2. "No convergence" → Usually OK, estimation issues
3. "All p=NA" → Over-filtering

NEXT STEPS:
- Gene set enrichment
- Clustering patterns
- RT-PCR validation
```

### Research Task: LDSC Explained

```
TEACHING MODE: LD Score Regression

LEVEL 1 - INTUITIVE:
Imagine measuring how much of height is genetic.
Even "non-significant" variants contain information.
If trait is heritable, you see inflation in test statistics.
LDSC measures this inflation → estimates heritability.

LEVEL 2 - STATISTICAL:
Problem: Can't just count significant hits because:
1. False positives (chance)
2. LD means one causal variant → many significant variants
3. Sample size affects power

Solution: Relate test statistic (χ²) to LD Score
LD Score = how much LD this variant has with others

Key formula: E[χ²] = 1 + (N × h² / M) × ℓ

LEVEL 3 - TECHNICAL:
[Mathematical details, worked example, assumptions...]

HOW TO RUN:
[Installation, input files, commands, interpretation...]

COMMON QUESTIONS:
Q: Works with case-control?
A: Yes, but h² is on liability scale...

EXTENSIONS:
1. Partitioned h²: Which regions enriched?
2. Genetic correlation: How related are two traits?
```

## What Gets Explained vs. Skipped

### Always Explain

✅ Design decisions
- "Why this data structure?"
- "Why this algorithm?"

✅ Non-obvious patterns
- Decorators, context managers
- Design patterns beyond basic

✅ Library choices
- "Why polars not pandas?"
- "Why scipy.optimize?"

✅ Edge cases
- Error handling rationale
- Input validation needs

✅ Trade-offs
- Performance vs. clarity
- Memory vs. speed

### Never Explain (Unless Teaching Python Itself)

❌ Basic syntax
- for-loops, if statements
- Variable assignment

❌ Obvious patterns
- List comprehensions
- String formatting

❌ Standard library
- File reading
- Basic string ops

❌ Self-evident code
- Variable names
- Simple calculations

## Integration with Other Skills

### With perform-analysis
```
User: "Perform heritability analysis in teaching mode"

Claude:
[8-step analysis framework]
+ Explanation at each step
+ Justify statistical choices
+ Provide replication commands
+ Interpret results for beginners
```

### With learn-tool
```
User: "Learn LDSC in teaching mode"

Claude:
[Install LDSC]
+ Why this installation method
+ What each dependency does
+ Common installation issues
+ Basic usage with explanation
+ When to use LDSC vs alternatives
```

## Output Structure

Teaching mode outputs follow this template:

```
[TASK HEADER]
Teaching Mode: ON

[BACKGROUND] (if needed)
Prerequisites: [Concepts needed]
Key terms: [Definitions]

[APPROACH]
Strategy: [High-level plan]
Why: [Reasoning]
Alternatives: [Other options + when to use]

[STEP-BY-STEP]
Step 1: [Action]
  Command: [Exact command]
  Why: [Reasoning]
  Note: [Important detail]

[DECISION POINTS]
Decision: [What was decided]
Reasoning: [Why]
Trade-off: [What was sacrificed]

[RESULTS]
Output: [What was produced]
Interpretation: [What it means]

[REPLICATION]
How to replicate: [Exact steps]
Required tools: [Installation]

[LEARNING RESOURCES]
To learn more: [Links/references]

[NEXT STEPS]
What to try: [Suggestions]
```

## Common Use Cases

### Learning a New Tool
```
User: "Teach me how to use argparse"

Claude provides:
- What argparse is and when to use it
- Complete working example
- Common patterns and pitfalls
- When to use alternatives (click, fire)
```

### Understanding an Analysis
```
User: "Explain this GWAS analysis in teaching mode"

Claude provides:
- Background on GWAS for beginners
- Each step of the analysis explained
- Why each statistical choice was made
- How to interpret results
- Common misconceptions
```

### Replicating a Workflow
```
User: "Show me how to set up multiprocessing with shared memory"

Claude provides:
- Complete working example
- Explanation of each component
- Common mistakes and how to avoid
- When to use vs. not use
- Performance considerations
```

## Tips for Best Results

### Be Specific About Your Level
- "I'm new to Python" → More basic explanations
- "I know Python but not genetics" → Skip coding basics, explain biology
- "Explain like I'm a graduate student" → Appropriate technical level

### Ask for Focus
- "Focus on the why, not the how" → Emphasize decisions
- "I want to replicate this" → Emphasize exact commands
- "Explain the math" → Technical derivations

### Request Specific Depth
- "High-level overview" → Skip details
- "In-depth explanation" → Include everything
- "Explain just the X part" → Focused teaching

## Installation

Already included if using claude-config repository with `pluginDirs`.

## Activation

After restarting Claude Code, activate by:
- "Teach me how to..."
- "Enable teaching mode and..."
- "Explain this in detail"
- "I want to learn about..."

## Version

Current version: 1.0.0

## Customization

Edit `skills/teaching-mode/SKILL.md` to:
- Adjust explanation depth defaults
- Add field-specific terminology
- Include common patterns in your domain
- Modify what's considered "obvious" vs "non-obvious"
