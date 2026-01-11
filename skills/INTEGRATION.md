# Plugin Integration Guide

This document explains how the three custom skills work together to provide a comprehensive data analysis workflow.

## The Three Skills

### 1. learn-tool
**Purpose**: Install and understand new tools/libraries/frameworks
**Triggers**: "Learn [tool]", "Try out [library]", "Set up [package]"

### 2. sanity-check-data
**Purpose**: Validate and explore datasets
**Triggers**: "Check this data", "Download dataset", "Validate data"

### 3. perform-analysis
**Purpose**: Systematic data analysis from planning to results
**Triggers**: "Perform analysis", "Run experiment", "Test hypothesis"

## Integration Flow

Here's how the skills automatically work together:

```
USER REQUEST
     |
     v
┌────────────────────────────────────────┐
│     PERFORM-ANALYSIS (Main Skill)      │
│  Systematic 8-step analysis framework  │
└────────────────────────────────────────┘
     |
     | Step 3: Verify Resources
     |
     v
┌─────────────────────┐  ┌──────────────────────┐
│ sanity-check-data   │  │    learn-tool        │
│ (if new data)       │  │ (if unfamiliar tool) │
└─────────────────────┘  └──────────────────────┘
     |                            |
     | Returns:                   | Returns:
     | - Data validated          | - Tool installed
     | - Format understood       | - Tool tested
     | - Issues identified       | - Examples provided
     |                            |
     v                            v
┌────────────────────────────────────────┐
│    PERFORM-ANALYSIS (Continues)        │
│  Steps 4-8: Plan, Execute, Report     │
└────────────────────────────────────────┘
```

## Real-World Example Workflows

### Example 1: Simple Analysis with Familiar Data

**User**: "Test if treatment improves outcomes in results.csv"

**Flow**:
```
perform-analysis activates
  ↓
Step 1: Understand motivation ✓
Step 2: Set expectations ✓
Step 3: Verify resources
  - results.csv: Found ✓
  - Python/scipy: Installed ✓
  - Data is familiar (used before) ✓
  → No need to invoke other skills
Step 4-8: Plan and execute analysis ✓
```

**Skills used**: perform-analysis only

---

### Example 2: Analysis with New Data

**User**: "Analyze the expression data I just downloaded"

**Flow**:
```
perform-analysis activates
  ↓
Step 1-2: Motivation and expectations ✓
Step 3: Verify resources
  - Data file: Found
  - First time seeing this data? YES
  ↓
  Invokes: sanity-check-data
    ↓
    sanity-check-data runs:
    1. Locate: expression_data.csv
    2. Format: CSV, tab-delimited
    3. Load: pandas ✓
    4. Stats: 15,000 genes × 48 samples
    5. Checks: ✓ All pass
    6. Visual: Distribution plot
    7. Report: Quality=Good
    8. Tools: ✓ Compatible
    ↓
  Returns to perform-analysis:
  - Data validated ✓
  - 15,000 × 48, no issues
  - Ready for analysis
Step 4-8: Continue with validated data ✓
```

**Skills used**: perform-analysis → sanity-check-data

---

### Example 3: Analysis Requiring New Tool

**User**: "Run differential expression with DESeq2"

**Flow**:
```
perform-analysis activates
  ↓
Step 1-2: Motivation and expectations ✓
Step 3: Verify resources
  - Data: Found ✓
  - DESeq2: Not installed ✗
  ↓
  Invokes: learn-tool
    ↓
    learn-tool runs:
    1. Search: DESeq2 documentation
    2. Install: Via Bioconductor
    3. Test: Load library, run example
    4. Verify: ✓ Works correctly
    ↓
  Returns to perform-analysis:
  - DESeq2 installed ✓
  - Basic usage understood
Step 4-8: Continue with DESeq2 ✓
```

**Skills used**: perform-analysis → learn-tool

---

### Example 4: New Data + New Tool

**User**: "Download GEO dataset GSE12345 and run limma analysis"

**Flow**:
```
perform-analysis activates
  ↓
Step 1-2: Motivation and expectations ✓
Step 3: Verify resources
  - Data: Need to download
  - limma: Not installed
  ↓
  Invokes: sanity-check-data
    ↓
    sanity-check-data runs:
    1. Acquire: Download from GEO ✓
    2. Format: Tab-delimited, gzipped
    3. Load: pandas
    4. Stats: 54,675 probes × 12 samples
    5. Checks: ✓ All pass
    6. Visual: Distribution
    7. Report: Quality=Excellent
    8. Tools: Need limma
       ↓
       Invokes: learn-tool (nested call)
         ↓
         learn-tool runs:
         1. Search: limma documentation
         2. Install: Via Bioconductor
         3. Test: Load, run example
         4. Verify: ✓ Works
         ↓
       Returns: limma ready ✓
    ↓
  Returns to perform-analysis:
  - Data downloaded and validated ✓
  - limma installed and tested ✓
Step 4-8: Continue with analysis ✓
```

**Skills used**: perform-analysis → sanity-check-data → learn-tool (nested)

---

### Example 5: Standalone Data Validation

**User**: "Check the quality of my sequencing data"

**Flow**:
```
sanity-check-data activates (standalone)
  ↓
1. Locate: data.fastq.gz
2. Format: FASTQ, gzipped
3. Load: Need seqtk
   ↓
   Invokes: learn-tool
     ↓
     Install seqtk ✓
   ↓
   Returns: seqtk ready
4. Stats: 2.5M reads, 150bp
5. Checks: Quality encoding ✓, distribution ✓
6. Visual: Quality per base plot
7. Report: Quality=Good, ready for alignment
8. Tools: ✓ Compatible with BWA, bowtie2
```

**Skills used**: sanity-check-data → learn-tool

---

### Example 6: Standalone Tool Learning

**User**: "Help me learn ripgrep"

**Flow**:
```
learn-tool activates (standalone)
  ↓
1. Search: ripgrep documentation
2. Install: brew install ripgrep ✓
3. Test: rg --version, run searches
4. Examples: Common usage patterns
```

**Skills used**: learn-tool only

## When Each Skill Activates

### perform-analysis
**Triggers**:
- "Perform analysis"
- "Run experiment"
- "Test if X correlates with Y"
- "Analyze this data to..."
- "Compute statistics for..."

**Automatically invokes**:
- sanity-check-data (Step 3, if new/unfamiliar data)
- learn-tool (Step 3, if unfamiliar tool)

---

### sanity-check-data
**Triggers**:
- "Download this dataset"
- "Check this data"
- "Validate data"
- "Sanity check..."
- "Explore this dataset"

**Can be invoked by**:
- perform-analysis (during resource verification)
- User directly

**Automatically invokes**:
- learn-tool (Step 3, if specialized reader needed)

---

### learn-tool
**Triggers**:
- "Learn [tool]"
- "Try out [library]"
- "Set up [package]"
- "Install and test [tool]"

**Can be invoked by**:
- perform-analysis (when tool missing)
- sanity-check-data (when reader needed)
- User directly

**Does NOT invoke other skills**:
- Leaf node in the dependency graph

## Dependency Graph

```
User Input
    |
    ├─→ perform-analysis (main workflow)
    |        |
    |        ├─→ sanity-check-data
    |        |        |
    |        |        └─→ learn-tool
    |        |
    |        └─→ learn-tool
    |
    ├─→ sanity-check-data (standalone)
    |        |
    |        └─→ learn-tool
    |
    └─→ learn-tool (standalone)
```

**Key points**:
- perform-analysis is the top-level orchestrator
- sanity-check-data can be top-level or invoked
- learn-tool is always a supporting skill
- All invocations are automatic - user just makes one request

## Benefits of Integration

### 1. Seamless Workflow
- User makes one request
- Claude handles all dependencies automatically
- No need to manually install tools or validate data

### 2. Automatic Validation
- Data is validated before analysis
- Tools are tested before use
- Issues caught early

### 3. Complete Documentation
- Every step documented
- Tools and data provenance tracked
- Reproducible workflows

### 4. Intelligent Reuse
- If data already validated, skip re-validation
- If tool already installed, skip re-installation
- Efficient and avoids redundancy

## Example: Complete Analysis Workflow

**User**: "Download data from GEO GSE99999 and perform differential expression analysis"

**Complete Flow**:

```
1. perform-analysis activates
   "I'll help you perform differential expression analysis"

2. perform-analysis Step 1: Understand Motivation
   "You want to identify differentially expressed genes..."

3. perform-analysis Step 2: Set Expectations
   "Expected: Some differential genes based on experimental design"

4. perform-analysis Step 3: Verify Resources
   "I need to download and validate the GEO data first..."

   4a. Invokes sanity-check-data:
       "Downloading and validating GSE99999..."

       4a.1. sanity-check-data Step 1: Acquire
             wget from GEO → Success

       4a.2. sanity-check-data Step 2: Format
             Tab-delimited matrix

       4a.3. sanity-check-data Step 3: Load
             "Need DESeq2 to analyze this properly..."

             Invokes learn-tool:
             "Installing DESeq2..."
             - Search Bioconductor docs
             - Install via R
             - Test with example
             - ✓ DESeq2 ready

             Returns to sanity-check-data

       4a.4-8. sanity-check-data continues:
             - Stats: 20,000 genes × 24 samples
             - Checks: ✓ All pass
             - Report: Quality=Good
             - Tools: ✓ DESeq2 compatible

       Returns to perform-analysis:
       - Data validated ✓
       - DESeq2 installed ✓

5. perform-analysis Step 4: Make Plan
   "1. Load count data
    2. Create DESeq2 object
    3. Run differential expression
    4. Apply FDR correction
    5. Create volcano plot
    6. Generate results table"

6. perform-analysis Step 5: Perform Analysis
   [Executes plan with DESeq2]

7. perform-analysis Step 6: Display Results
   [Shows volcano plot and top genes]
   "KEY TAKEAWAY: 1,423 differentially expressed genes..."

8. perform-analysis Step 7: Document Choices
   "Used DESeq2 with FDR < 0.05..."

9. perform-analysis Step 8: List Files
   Scripts: analysis.R
   Data: results_all.csv, results_sig.csv
   Figures: volcano.png, ma_plot.png
```

**Result**: Complete analysis from download to publication-ready results, all handled automatically!

## Best Practices

### For Users

1. **Start with perform-analysis** for most analytical tasks
   - Let it orchestrate other skills as needed

2. **Use sanity-check-data** when you just want to validate data
   - Before committing to analysis
   - When data quality is uncertain

3. **Use learn-tool** when you just want to try a new tool
   - Exploring options
   - Learning for future use

### For Skill Integration

1. **Skills should be self-contained**
   - Each can run independently
   - But work together when needed

2. **Avoid circular dependencies**
   - Current structure: perform → sanity/learn, sanity → learn
   - Never: learn → sanity (would create cycle)

3. **Clear handoffs**
   - Invoking skill states what it needs
   - Invoked skill returns clear status
   - Invoking skill continues with results

## Future Skills

Potential additions that would integrate:

### submit-O2-job
**Purpose**: Submit jobs to O2 compute cluster
**Integration**:
- Invoked by perform-analysis for long-running jobs
- Would handle SLURM submission and monitoring

### create-figure
**Purpose**: Publication-quality figure generation
**Integration**:
- Invoked by perform-analysis (Step 6)
- Invoked by sanity-check-data (Step 6)

### write-methods
**Purpose**: Generate methods section from analysis
**Integration**:
- Invoked by perform-analysis (Step 8)
- Uses analysis log to write methods

## Monitoring Integration

You can see when skills invoke each other:

```
perform-analysis running...
  ↓
  "I see this is new data, let me validate it first..."
  ↓
  [sanity-check-data activates]
  ↓
  "This VCF needs bcftools, let me learn how to use it..."
  ↓
  [learn-tool activates]
  ↓
  "bcftools installed and tested ✓"
  ↓
  [returns to sanity-check-data]
  ↓
  "Data validated ✓"
  ↓
  [returns to perform-analysis]
  ↓
  "Continuing with analysis..."
```

Clear communication at each transition ensures you understand what's happening.

## Summary

The three skills form a powerful, integrated system:

- **learn-tool**: Foundation (installs capabilities)
- **sanity-check-data**: Quality control (validates inputs)
- **perform-analysis**: Orchestrator (end-to-end workflow)

Together, they provide:
✓ Complete automation
✓ Systematic validation
✓ Tool management
✓ Quality assurance
✓ Full documentation
✓ Reproducible science
