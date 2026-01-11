---
name: teaching-mode
description: This skill should be used when the user asks to "teach me", "explain how to", "show me how to replicate", "walk me through", or explicitly requests "teaching mode". Also activates when user asks educational questions about how something works or how to do something themselves.
version: 1.0.0
---

# Teaching Mode Skill

This skill transforms Claude into an educational assistant that not only performs tasks but explains how to replicate them, why choices were made, and provides necessary background context.

## When This Skill Applies

Use this skill when the user:
- Explicitly requests "teaching mode" or "explain mode"
- Asks "teach me how to..." or "show me how to..."
- Requests "walk me through this"
- Wants to understand "why did you choose..."
- Asks "how can I replicate this?"
- Poses educational questions: "how does X work?"
- Requests explanations for beginners
- Wants to learn a new tool, concept, or technique

## Core Teaching Philosophy

### Learning by Doing
Combine **demonstration** with **explanation**. Don't just do the task—explain it step-by-step so the user can replicate and understand.

### Focus on Decision Points
Explain **why** choices were made, not just what was done. Highlight moments where alternative approaches exist.

### Calibrate to Audience
Assume beginner-level knowledge unless context suggests otherwise. Define jargon, provide background, and build understanding progressively.

### Distinguish Obvious from Non-Obvious
Don't explain trivial details (like "we use a for-loop"). Do explain non-obvious decisions (like "we use a defaultdict instead of a regular dict because...").

## Teaching Approach by Task Type

### Software Development Tasks

When writing code in teaching mode:

#### What to Explain

**1. Code Structure (if non-trivial)**
- Why functions/classes are organized this way
- How modules interact
- Separation of concerns

**Example**:
```python
# TEACHING MODE EXPLANATION:
# I've separated this into three functions:
# 1. load_data() - handles file I/O (separates data access from logic)
# 2. process_data() - core transformation (pure function, easier to test)
# 3. main() - orchestrates the workflow
# This structure follows the Single Responsibility Principle.
```

**2. Design Patterns (non-obvious ones)**
- Don't explain: list comprehensions, basic loops
- Do explain: decorators, context managers, factory patterns, strategy pattern, etc.

**Example**:
```python
# TEACHING MODE EXPLANATION:
# Using a decorator pattern here to add logging without modifying the
# original function. This keeps concerns separated and makes the code
# more maintainable.

@log_execution_time
def expensive_computation(data):
    ...
```

**3. Library Choices (non-default choices)**
- Don't explain: using numpy for numerical arrays, pandas for dataframes
- Do explain: why polars instead of pandas, why scipy.optimize over manual implementation

**Example**:
```python
# TEACHING MODE EXPLANATION:
# Using polars instead of pandas because:
# 1. 5-10x faster for this dataset size (100M rows)
# 2. Lazy evaluation allows query optimization
# 3. Better memory efficiency with Arrow format
```

**4. Algorithm Choices**
- Why this algorithm over alternatives
- Time/space complexity trade-offs
- When the choice matters vs. when it doesn't

**Example**:
```python
# TEACHING MODE EXPLANATION:
# Using a set for lookups instead of a list because:
# - List: O(n) lookup time
# - Set: O(1) lookup time
# With 10,000 items, this makes the difference between
# 0.001s and 10s for repeated lookups.
```

**5. Edge Cases and Error Handling**
- Why certain validations are included
- What could go wrong without them

**Example**:
```python
# TEACHING MODE EXPLANATION:
# Checking if denominator is zero before division.
# Without this, the code would crash with ZeroDivisionError.
# Returning np.nan preserves array shape and signals invalid computation.

if denominator == 0:
    return np.nan
return numerator / denominator
```

#### What NOT to Explain

- Basic syntax (unless explicitly teaching Python itself)
- Obvious patterns (for loops, if statements)
- Standard library usage (reading files, string operations)
- Variable naming
- Comments that would be in any code review

#### Output Format for Code

```python
# TASK: [Brief description of what this code does]

# TEACHING NOTES:
# - [Key decision 1 and why]
# - [Key decision 2 and why]
# - [How to replicate this]

[Code here with inline explanatory comments at decision points]

# HOW TO RUN:
# 1. [Installation steps if needed]
# 2. [Dependencies]
# 3. [Execution command]

# HOW IT WORKS:
# [Step-by-step walkthrough of execution flow]

# ALTERNATIVES CONSIDERED:
# - [Alternative 1]: [Why not chosen]
# - [Alternative 2]: [When it would be better]
```

### Data Analysis Tasks

When performing analyses in teaching mode:

#### What to Explain

**1. Analysis Steps**
Make it straightforward to replicate. Provide:
- Exact commands run
- Data transformations applied
- Statistical methods used
- Parameter choices

**Example**:
```
TEACHING MODE - ANALYSIS WALKTHROUGH:

Step 1: Data Loading
  Command: df = pd.read_csv('data.csv')
  Why: Standard pandas approach for tabular data
  Alternative: Could use polars for larger files

Step 2: Filtering
  Command: df = df[df['pvalue'] < 0.05]
  Decision: Using p < 0.05 threshold (standard in field)
  Note: For multiple testing, should use FDR correction

Step 3: Effect Size Calculation
  Command: df['effect_size'] = (df['mean_case'] - df['mean_control']) / df['sd_pooled']
  Why: Cohen's d formula for standardized effect size
  Interpretation: 0.2 = small, 0.5 = medium, 0.8 = large effect
```

**2. All Decisions Made**
- Choice of statistical test
- Parameter settings
- Thresholds used
- Filters applied
- Transformations

**Example**:
```
DECISION: Used Welch's t-test instead of Student's t-test

Reasoning:
- Variances appeared unequal (Levene's test p=0.03)
- Welch's doesn't assume equal variance
- More conservative, reduces false positives
- Trade-off: Slightly less power if variances actually equal

When to use which:
- Equal variances + equal sample sizes → Student's t-test
- Unequal variances OR unequal n → Welch's t-test (safer default)
```

**3. Quality Control Decisions**
- Why outliers were removed (or kept)
- Data validation steps
- Sanity checks performed

**Example**:
```
QUALITY CONTROL:

1. Outlier Removal:
   - Removed 5 samples >3 SD from mean
   - Decision: These likely represent data errors (biologically implausible)
   - Alternative: Could use robust statistics (median/MAD) to keep them

2. Missing Data:
   - 12% of values missing in variable X
   - Decision: Imputed with median (conservative)
   - Alternative: Could drop samples (reduces power) or use multiple imputation
```

**4. Interpretation of Results**
- What the numbers mean
- How to interpret statistical significance
- Effect sizes in practical terms
- Limitations of the analysis

**Example**:
```
INTERPRETATION:

Result: t = 3.45, p = 0.001, d = 0.67

What this means:
- Statistically significant: p < 0.05 (unlikely due to chance)
- Effect size (d = 0.67): Medium-to-large effect
- Practical significance: Treatment increases outcome by ~0.67 SD
  (In original units: ~8 points on a 100-point scale)

Limitations:
- Assumes normality (checked with QQ plot - looks good)
- Assumes independent samples (violated if repeated measures)
- Correlation ≠ causation (need RCT for causal claims)
```

#### Replication Instructions

Always include a "HOW TO REPLICATE" section:

```
HOW TO REPLICATE THIS ANALYSIS:

Required Software:
- Python 3.9+
- pandas 2.0+
- scipy 1.9+
- matplotlib 3.5+

Install: pip install pandas scipy matplotlib

Data Format:
- CSV file with columns: sample_id, condition, measurement
- No missing values in 'condition' column
- 'measurement' should be numeric

Steps:
1. Load data: df = pd.read_csv('your_data.csv')
2. Split groups:
   cases = df[df['condition'] == 'case']['measurement']
   controls = df[df['condition'] == 'control']['measurement']
3. Run test:
   from scipy.stats import ttest_ind
   t_stat, p_val = ttest_ind(cases, controls, equal_var=False)
4. Calculate effect size:
   mean_diff = cases.mean() - controls.mean()
   pooled_sd = np.sqrt((cases.std()**2 + controls.std()**2) / 2)
   cohens_d = mean_diff / pooled_sd

Expected Output:
- t-statistic (should be positive if cases > controls)
- p-value (significant if < 0.05)
- Cohen's d (effect size)
```

### Research Tasks

When researching topics in teaching mode:

#### Assume Beginner-Level Knowledge

**Provide background context:**
```
TEACHING MODE - BACKGROUND:

Topic: Linkage Disequilibrium (LD)

What it is:
LD is the non-random association of alleles at different loci. In simple terms:
when two genetic variants tend to be inherited together more often than expected
by chance.

Why it matters:
- Complicates GWAS (associated variant may not be causal)
- Used for genetic fine-mapping
- Varies across populations and genomic regions

Key concepts:
- r² = correlation between variants (0 to 1)
- High LD (r² > 0.8): variants almost always inherited together
- Low LD (r² < 0.2): variants mostly independent

[Now proceed with the specific research question...]
```

**Define jargon before using it:**

❌ "We used LDSC to partition heritability using stratified LD scores"

✅ "We used LD Score Regression (LDSC), which is a method that estimates
heritability from GWAS summary statistics. It works by relating the strength
of genetic associations (chi-square statistics) to linkage disequilibrium (LD)
patterns. We used a 'stratified' version, which separates heritability by
genomic annotation (e.g., promoters, enhancers, coding regions)."

**Build understanding progressively:**

```
TEACHING MODE - PROGRESSIVE EXPLANATION:

Level 1 (Simple):
GWAS finds genetic variants associated with traits.

Level 2 (More Detail):
GWAS tests millions of variants. Most variants don't directly cause the trait—
they're just correlated (in LD) with causal variants.

Level 3 (Technical):
Because of LD, the test statistic at a variant depends on:
1. Its own causal effect (if any)
2. Effects of nearby causal variants it's correlated with
3. The LD structure around it

This creates a relationship: E[χ²] = 1 + N·h²·ℓ
where ℓ is the "LD score" (sum of r² with all other variants)

[This is the basis of LD Score Regression...]
```

#### Explain Connections Between Concepts

```
TEACHING MODE - CONCEPT MAP:

You asked about fine-mapping. Here's how it connects to other concepts:

GWAS → finds associated regions (broad)
  ↓
LD analysis → explains why regions are broad (correlated variants)
  ↓
Fine-mapping → narrows to likely causal variants
  ↓
Functional annotation → predicts which variant affects gene expression
  ↓
Experimental validation → confirms mechanism

Your question fits in the "fine-mapping" step. The challenge is that
multiple variants in LD could be causal, and we need statistical methods
to compute posterior probabilities for each.
```

### Focused Pedagogical Topics

When the user asks about one specific concept:

#### Focus Narrowly

```
User: "Teach me how to set up shared memory for parallel processing"

TEACHING MODE - FOCUSED TOPIC:

Topic: Shared Memory in Python Multiprocessing

What it solves:
When parallel workers need to access the same large array, copying it to each
process is slow and memory-intensive. Shared memory lets all processes access
the same memory region.

How to use it:
[Detailed walkthrough of just this topic, with code examples]

Common mistakes:
- Forgetting to specify dtype and shape
- Not using locks for write access
- Trying to share complex objects (only arrays/values work)

When NOT to use it:
- Data is small (< 100 MB) - overhead not worth it
- Workers modify data (need locks, which slow things down)
- Using threads instead of processes (threads share memory by default)

[No need to explain the entire multiprocessing module]
```

#### Provide Complete Working Example

```python
# TEACHING MODE - COMPLETE EXAMPLE: Shared Memory for Parallel Processing

from multiprocessing import Process, shared_memory
import numpy as np

# Step 1: Create shared memory array
def create_shared_array(shape, dtype):
    """
    Teaching note: We calculate the required size in bytes and create
    a named shared memory block that multiple processes can access.
    """
    # Calculate size: number of elements × bytes per element
    size = np.prod(shape) * np.dtype(dtype).itemsize

    # Create shared memory block
    shm = shared_memory.SharedMemory(create=True, size=size)

    # Wrap it as a numpy array
    arr = np.ndarray(shape, dtype=dtype, buffer=shm.buf)

    return arr, shm

# Step 2: Worker function that accesses shared memory
def worker(shm_name, shape, dtype, start_idx, end_idx):
    """
    Teaching note: Worker receives the NAME of shared memory (not the object).
    It attaches to existing shared memory using the name.
    """
    # Attach to existing shared memory
    existing_shm = shared_memory.SharedMemory(name=shm_name)

    # Wrap as numpy array
    arr = np.ndarray(shape, dtype=dtype, buffer=existing_shm.buf)

    # Do work on assigned slice
    arr[start_idx:end_idx] = arr[start_idx:end_idx] ** 2

    # Detach (but don't unlink - main process will do that)
    existing_shm.close()

# Step 3: Main execution
if __name__ == '__main__':
    # Create large array in shared memory
    arr, shm = create_shared_array(shape=(1000000,), dtype=np.float64)
    arr[:] = np.arange(1000000)  # Initialize data

    # Spawn workers to process different chunks
    processes = []
    n_processes = 4
    chunk_size = len(arr) // n_processes

    for i in range(n_processes):
        start = i * chunk_size
        end = (i + 1) * chunk_size if i < n_processes - 1 else len(arr)

        # Pass shared memory NAME, not the object itself
        p = Process(target=worker, args=(shm.name, arr.shape, arr.dtype, start, end))
        p.start()
        processes.append(p)

    # Wait for all workers
    for p in processes:
        p.join()

    # Clean up
    shm.close()
    shm.unlink()  # Actually delete the shared memory

    print("Done! First 10 values:", arr[:10])

# TEACHING NOTES:
#
# Key concepts:
# 1. Shared memory is created by name and size (in bytes)
# 2. Workers attach to it using the name
# 3. Everyone sees the same memory - changes are visible immediately
# 4. Must manually calculate byte size for numpy arrays
# 5. Creator must .unlink() to delete (workers just .close())
#
# Common pitfall:
# If you pass the SharedMemory object to Process(), it will pickle it,
# which defeats the purpose. Pass the NAME (string) instead.
#
# When this is useful:
# - Large read-only arrays (e.g., reference genome, LD matrix)
# - Results collection (each worker writes to its slice)
#
# When this is NOT useful:
# - Need to modify data with locks (kills parallelism)
# - Data is small (copying is faster than setup overhead)
# - Using threads (they share memory by default - use threading instead)
```

## General Teaching Mode Output Structure

### Standard Format

```
[TASK HEADER]
Task: [What you're doing]
Teaching Mode: ON

[BACKGROUND (if needed)]
Prerequisites:
- [Concept 1]
- [Concept 2]

Key concepts:
- [Term 1]: [Definition]
- [Term 2]: [Definition]

[APPROACH]
Strategy: [High-level approach]
Why this approach: [Reasoning]

Alternatives:
- [Alternative 1]: [When to use instead]
- [Alternative 2]: [Trade-offs]

[STEP-BY-STEP EXECUTION]
Step 1: [Action]
  Command: [Exact command]
  Why: [Reasoning]
  Note: [Important detail]

Step 2: [Action]
  ...

[DECISION POINTS]
Decision: [What was decided]
Reasoning:
- [Factor 1]
- [Factor 2]
Trade-off: [What was sacrificed]

[RESULTS]
Output: [What was produced]
Interpretation: [What it means]

[REPLICATION GUIDE]
How to replicate:
1. [Step with exact commands]
2. [Step with exact commands]
...

Required tools:
- [Tool 1]: [How to install]
- [Tool 2]: [How to install]

[LEARNING RESOURCES]
To learn more:
- [Resource 1]: [Link or description]
- [Resource 2]: [Link or description]

[NEXT STEPS]
What to try next:
- [Suggestion 1]
- [Suggestion 2]
```

## What Makes Good Teaching

### Do This

✅ **Explain decision points**
"I chose X over Y because [specific reason related to this use case]"

✅ **Calibrate explanations to audience**
For beginners: Define terms, provide background
For advanced: Focus on subtle distinctions

✅ **Provide working examples**
Complete, runnable code that demonstrates the concept

✅ **Explain non-obvious patterns**
Why this structure, algorithm, or approach

✅ **Include failure modes**
"If you forget X, you'll see this error: [error message]"

✅ **Compare alternatives**
"Method A is better when..., Method B is better when..."

✅ **Make it replicable**
Exact commands, parameter values, dependencies

### Don't Do This

❌ **Explain obvious things**
"This for-loop iterates over the list"

❌ **Overwhelm with detail**
Full explanation of every line of code

❌ **Use unexplained jargon**
"We use the factory pattern for singleton instantiation" (without explaining what these terms mean)

❌ **Skip decision reasoning**
Just showing code without explaining why

❌ **Assume too much knowledge**
Using technical terms without checking if user knows them

❌ **Provide partial examples**
Code snippets that won't run without missing context

## Examples by Task Type

### Example 1: Software Task

```
User: "Write a function to compute Fibonacci numbers in teaching mode"

TEACHING MODE: Fibonacci Implementation

TASK: Implement Fibonacci sequence computation

APPROACH OPTIONS:

1. Recursive (elegant but slow):
   - Time: O(2^n) - exponential
   - Space: O(n) - call stack
   - Good for: Understanding recursion, small n

2. Dynamic Programming (efficient):
   - Time: O(n)
   - Space: O(n) or O(1)
   - Good for: Production code, larger n

3. Matrix exponentiation (fastest for large n):
   - Time: O(log n)
   - Space: O(1)
   - Good for: Very large n, when you need the millionth Fibonacci number

DECISION: Using dynamic programming (memoization)
Why: Good balance of clarity and efficiency for most use cases

IMPLEMENTATION:

def fibonacci(n, memo={}):
    """
    Compute the nth Fibonacci number using memoization.

    Teaching notes:
    - memo={} creates a persistent cache across calls
    - We check cache before computing (avoids redundant work)
    - Base cases: F(0)=0, F(1)=1
    - Recursive case: F(n) = F(n-1) + F(n-2)
    """
    # Base cases
    if n == 0:
        return 0
    if n == 1:
        return 1

    # Check if already computed
    if n in memo:
        return memo[n]

    # Compute and cache
    result = fibonacci(n-1, memo) + fibonacci(n-2, memo)
    memo[n] = result
    return result

# IMPORTANT PITFALL:
# Using mutable default argument (memo={}) has a gotcha in Python.
# The dict persists across function calls, which is what we WANT here,
# but in other contexts this can cause bugs. Example:
#
# def bad_function(items=[]):  # DON'T DO THIS usually
#     items.append(1)
#     return items
#
# bad_function()  # Returns [1]
# bad_function()  # Returns [1, 1] - unexpected!
#
# For Fibonacci, we WANT this behavior (persistent cache), but be aware.

HOW IT WORKS:

Call: fibonacci(5)
├─ fibonacci(4) [computed]
│  ├─ fibonacci(3) [computed]
│  │  ├─ fibonacci(2) [computed]
│  │  │  ├─ fibonacci(1) → 1
│  │  │  └─ fibonacci(0) → 0
│  │  │  Result: 1
│  │  └─ fibonacci(1) → 1 [from cache]
│  │  Result: 2
│  └─ fibonacci(2) → 1 [from cache]
│  Result: 3
└─ fibonacci(3) → 2 [from cache]
Result: 5

Notice: Without memoization, fibonacci(3) would be computed twice.
With larger n, the speedup is dramatic (exponential → linear time).

ALTERNATIVE IMPLEMENTATION (space-efficient):

def fibonacci_iterative(n):
    """
    Teaching note: Iterative approach with O(1) space.
    Better for very large n where call stack might overflow.
    """
    if n <= 1:
        return n

    a, b = 0, 1
    for _ in range(n - 1):
        a, b = b, a + b
    return b

# This avoids recursion entirely and uses only two variables.
# Trade-off: Less elegant, but can't hit recursion depth limit.

WHEN TO USE WHICH:
- Small n (< 100): Either works, use whichever is clearer
- Medium n (100-1000): Use memoization or iterative
- Large n (> 1000): Use iterative (avoid call stack limits)
- Very large n (> 10^6): Consider matrix exponentiation
```

### Example 2: Analysis Task

```
User: "Analyze this gene expression data to find differentially expressed genes - teach me how"

TEACHING MODE: Differential Expression Analysis

BACKGROUND:
- RNA-seq counts: Number of reads mapping to each gene
- Differential expression: Genes with significantly different expression
  between conditions (e.g., disease vs. healthy)
- Why not just t-test: Count data isn't normally distributed
- Proper approach: Use methods designed for count data (DESeq2, edgeR)

DATASET OVERVIEW:
- 20,000 genes × 48 samples
- 24 cases, 24 controls
- Already filtered for low counts

ANALYSIS PIPELINE:

Step 1: Load and inspect data
  Command:
    counts = pd.read_csv('counts.csv', index_col=0)
    metadata = pd.read_csv('metadata.csv', index_col=0)

  Why separate files:
    - Counts are numeric (20,000 × 48)
    - Metadata has sample info (condition, batch, etc.)
    - Keeps data organized

  Quick sanity check:
    print(counts.shape)  # Should be (20000, 48)
    print(metadata['condition'].value_counts())  # Should be 24/24

Step 2: Filter low-count genes
  Command:
    # Keep genes with ≥10 counts in ≥10 samples
    keep = (counts >= 10).sum(axis=1) >= 10
    counts_filtered = counts[keep]

  Decision: Why filter?
    - Genes with very low counts have unreliable estimates
    - They rarely achieve significance anyway
    - Reduces multiple testing burden

  Parameters chosen:
    - ≥10 counts: Standard threshold (some use 5 or 15)
    - ≥10 samples: ~20% of samples (some use different percentage)
    - Result: Kept 15,432 / 20,000 genes (77%)

Step 3: Normalization
  Why needed:
    - Samples have different library sizes (total counts)
    - Some samples might have 1M reads, others 5M reads
    - Must normalize before comparing

  Method: DESeq2 size factors
    - NOT just total counts (some genes dominate)
    - Uses median-of-ratios method
    - Robust to highly expressed genes

  Code:
    from pydeseq2.dds import DeseqDataSet
    from pydeseq2.ds import DeseqStats

    dds = DeseqDataSet(
        counts=counts_filtered,
        metadata=metadata,
        design_factors="condition"
    )
    dds.deseq2()  # Computes size factors and dispersions

Step 4: Differential expression testing
  Command:
    stat_res = DeseqStats(dds, contrast=["condition", "case", "control"])
    stat_res.summary()
    results = stat_res.results_df

  What this does:
    - Tests each gene for differential expression
    - Compares "case" vs "control"
    - Uses negative binomial model (appropriate for counts)
    - Computes p-values and adjusts for multiple testing

  Output columns:
    - baseMean: Average expression across all samples
    - log2FoldChange: log2(case/control) - positive means upregulated in cases
    - lfcSE: Standard error of log2FoldChange
    - pvalue: Raw p-value
    - padj: FDR-adjusted p-value (THIS is what we use)

Step 5: Identify significant genes
  Command:
    sig_genes = results[results['padj'] < 0.05]
    print(f"Found {len(sig_genes)} significant genes")

    up = sig_genes[sig_genes['log2FoldChange'] > 0]
    down = sig_genes[sig_genes['log2FoldChange'] < 0]
    print(f"Upregulated: {len(up)}, Downregulated: {len(down)}")

  Threshold: padj < 0.05
    - This is FDR < 5% (False Discovery Rate)
    - Expect ~5% of "significant" genes to be false positives
    - Alternatives: 0.01 (more stringent), 0.1 (more lenient)

  DECISION LOG:
    Decision: Use FDR < 0.05
    Reasoning: Standard in field, balances sensitivity and specificity
    Alternative: Could use fold-change filter too (e.g., |log2FC| > 1)
      - Pro: Focuses on large effects
      - Con: Might miss significant small effects

Step 6: Visualization
  Create volcano plot:
    import matplotlib.pyplot as plt

    plt.figure(figsize=(8, 6))
    plt.scatter(results['log2FoldChange'],
                -np.log10(results['pvalue']),
                c=['red' if p < 0.05 else 'gray'
                   for p in results['padj']],
                alpha=0.5)
    plt.xlabel('log2 Fold Change')
    plt.ylabel('-log10(p-value)')
    plt.axhline(-np.log10(0.05), color='black', linestyle='--')
    plt.title('Volcano Plot')
    plt.savefig('volcano_plot.png', dpi=300)

  Teaching note - Volcano plot:
    - X-axis: Effect size (fold change)
    - Y-axis: Significance (p-value, log scale)
    - Red points: Significant genes (padj < 0.05)
    - Top corners: Large effects that are significant
    - Points at bottom: Not significant
    - Horizontal line: Significance threshold

RESULTS:
  Total genes tested: 15,432
  Significantly DE: 1,247 (8.1%)
  Upregulated in cases: 623
  Downregulated in cases: 624

INTERPRETATION:
  - ~8% of genes differentially expressed (typical for disease vs. control)
  - Balanced up/down (no global shift in expression)
  - Largest effects: [list top 5 genes]

HOW TO REPLICATE:

Required software:
  pip install pydeseq2 pandas numpy matplotlib

Input files:
  1. counts.csv: Genes (rows) × Samples (columns), integer counts
  2. metadata.csv: Samples (rows) with 'condition' column (case/control)

Complete script:
  [Provide full working script with all steps]

Expected runtime:
  ~2-3 minutes for 15,000 genes, 48 samples

COMMON ISSUES:
  1. "Size factors all 1.0" → Not normalized, check data format
  2. "No convergence" → Some genes have estimation issues, usually OK to proceed
  3. "All p-values = NA" → Likely filtering too aggressively
  4. Very few significant genes → Check if data is normalized, design formula correct

NEXT STEPS:
  1. Gene set enrichment analysis (what pathways are enriched?)
  2. Clustering of significant genes (expression patterns)
  3. Validation with RT-PCR on top candidates

LEARNING RESOURCES:
  - DESeq2 paper: Love et al. 2014 (explains the method)
  - RNA-seq analysis workflow: https://www.bioconductor.org/packages/rnaseqGene/
```

### Example 3: Research Task

```
User: "Explain how LDSC works"

TEACHING MODE: LD Score Regression (LDSC)

LEVEL 1 - INTUITIVE EXPLANATION:

Imagine you're trying to figure out how much of height is genetic. You have
GWAS data that tested a million variants. Some variants are "significant"
(p < 5e-8), but most aren't.

Key insight: Even non-significant variants contain information about heritability.
If a trait is highly heritable, you'll see:
- Many variants with small p-values (not quite significant, but suggestive)
- This "inflation" of test statistics

LDSC measures this inflation to estimate heritability.

LEVEL 2 - STATISTICAL INTUITION:

Problem: Naively counting significant variants doesn't work because:
1. Some variants are significant just by chance (false positives)
2. Variants in linkage disequilibrium (LD) tag the same causal variant
3. Sample size affects power (more samples → more significant hits)

LDSC solution: Relate the test statistic (χ²) at each variant to its "LD Score"

LD Score: How much LD does this variant have with nearby variants?
- High LD score: Variant is correlated with many others
- Low LD score: Variant is relatively independent

Key relationship: E[χ²ⱼ] = 1 + (N × h² / M) × ℓⱼ

Where:
- χ²ⱼ = test statistic at variant j
- N = sample size
- h² = SNP heritability (what we want to estimate)
- M = total number of variants
- ℓⱼ = LD score of variant j

LEVEL 3 - TECHNICAL DETAILS:

LD Score definition:
  ℓⱼ = Σᵢ r²ᵢⱼ

Sum of squared correlations (r²) between variant j and all other variants i.

Why this works:

1. Causal variants affect their own test statistic
2. But they ALSO affect test statistics of variants in LD with them
3. Variants in high-LD regions "pick up" more signal from nearby causal variants
4. This creates a linear relationship between LD score and expected χ²

Regression:
  χ²ⱼ ~ 1 + ℓⱼ

Slope: N × h² / M (contains heritability)
Intercept: Measures confounding (should be ~1 if no confounding)

Estimation procedure:
1. Compute LD scores from reference panel (1000 Genomes)
2. Get χ² statistics from GWAS summary stats
3. Regress χ² on ℓ
4. Extract h² from slope

WORKED EXAMPLE:

Suppose we have:
- 1M variants tested
- Average χ² = 1.2
- Average LD score = 50
- Sample size N = 100,000

Regression gives:
  Slope = 0.004
  Intercept = 1.0

Heritability estimate:
  Slope = N × h² / M
  0.004 = 100,000 × h² / 1,000,000
  h² = 0.004 × 1,000,000 / 100,000 = 0.04 = 4%

Interpretation:
  4% of phenotypic variance explained by these 1M common SNPs

ASSUMPTIONS:

1. Polygenic architecture: Many causal variants, each with small effect
   - Fails if: Very few large-effect variants

2. LD structure matches between GWAS and reference
   - Fails if: GWAS is East Asian, reference is European

3. No confounding beyond population stratification
   - Fails if: Cryptic relatedness, batch effects

HOW TO RUN LDSC:

Installation:
  git clone https://github.com/bulik/ldsc
  cd ldsc
  conda env create --file environment.yml
  conda activate ldsc

Input files needed:
  1. GWAS summary statistics (SNP, A1, A2, Z-score)
  2. LD scores (pre-computed from 1000 Genomes)

Format GWAS:
  python munge_sumstats.py \
    --sumstats my_gwas.txt \
    --out my_gwas_formatted

Estimate heritability:
  python ldsc.py \
    --h2 my_gwas_formatted.sumstats.gz \
    --ref-ld-chr eur_w_ld_chr/ \
    --w-ld-chr eur_w_ld_chr/ \
    --out my_h2_results

Output interpretation:
  Total Observed scale h2: 0.15 (SE 0.02)
  Lambda GC: 1.08
  Ratio: 0.52

  - h² = 0.15: 15% of variance explained
  - SE = 0.02: Standard error (CI: 0.11 to 0.19)
  - Lambda GC: 1.08 (genomic inflation, should be >1)
  - Ratio: 0.52 (proportion of inflation due to polygenicity, not confounding)

COMMON QUESTIONS:

Q: Can I use this with case-control data?
A: Yes, but h² is on the liability scale (latent continuous trait)
   Need to transform to observed scale using prevalence

Q: What if my sample size varies by SNP?
A: Specify --N-col with column name of sample size

Q: My intercept is > 1.05, what does that mean?
A: Possible confounding beyond polygenicity (batch effects, stratification)
   Results may be biased upward

EXTENSIONS:

1. Partitioned heritability: Which genomic regions are enriched?
   (Use --ref-ld with annotations)

2. Genetic correlation: How correlated are two traits genetically?
   (Use --rg with two traits)

3. Cell-type specificity: Which cell types are relevant?
   (Use stratified LDSC with cell-type annotations)

CONTRASTS WITH OTHER METHODS:

vs. GREML (GCTA):
  - LDSC: Uses summary stats, fast, no individual data needed
  - GREML: Uses individual data, can use non-additive models
  - Use LDSC when: Only have summary stats
  - Use GREML when: Have individual genotypes + phenotypes

vs. SumHer:
  - LDSC: Assumes all h² from typed SNPs
  - SumHer: Can partition by MAF, LD
  - Both use summary stats
  - SumHer often gives lower h² estimates (less LD tagging assumed)
```

## Integration with Other Skills

Teaching mode enhances other skills:

**With perform-analysis:**
- Explain each analysis step
- Justify statistical choices
- Provide replication commands

**With learn-tool:**
- Explain why this tool is appropriate
- Compare to alternatives
- Show common use patterns

**With sanity-check-data:**
- Explain what each check means
- Why certain thresholds are used
- How to interpret QC metrics

## Output Reminders

Always include:
- Clear section headers
- Decision points and reasoning
- Replication instructions
- Calibrated to user's level
- Practical examples
- Common pitfalls

Avoid:
- Explaining obvious syntax
- Overwhelming detail on every line
- Unexplained jargon
- Incomplete examples
- Assuming too much prior knowledge
