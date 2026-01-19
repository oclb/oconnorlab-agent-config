# Test Specifications

Detailed specifications for behavioral tests of the Claude Code configuration.

## Overview

| Spec File | Tests | Description |
|-----------|-------|-------------|
| [01-memory-retrieval.md](01-memory-retrieval.md) | 6 | INDEX.md reading and entry retrieval |
| [02-memory-creation.md](02-memory-creation.md) | 8 | Entry format, naming, structure |
| [03-todo-management.md](03-todo-management.md) | 7 | TODO.md and DONE.md management |
| [04-behavior-flags.md](04-behavior-flags.md) | 7 | AFK mode and behavior.conf |
| [05-skill-auto-detection.md](05-skill-auto-detection.md) | 9 | Skill trigger phrase detection |
| [06-skill-workflows.md](06-skill-workflows.md) | 8 | Skill step compliance |
| [07-feedback-logging.md](07-feedback-logging.md) | 4 | Feedback file creation |
| [08-edge-cases.md](08-edge-cases.md) | 8 | Error handling, edge cases |
| [09-explicit-skills.md](09-explicit-skills.md) | 5 | /skill-name invocation |
| **Total** | **62** | |

## Test Categories by Priority

### Critical (Run on Every Change)

These tests validate core functionality:

| Test | Spec | Reason |
|------|------|--------|
| 2.1 Entry Created for Substantive Work | 02 | Core memory system |
| 2.4 INDEX.md Updated | 02 | Index integrity |
| 4.1 AFK Mode No Questions | 04 | Key behavior toggle |
| 5.1 Detect perform-analysis | 05 | Primary skill |
| 6.2 Entry with Required Sections | 06 | Entry format |

### High (Run Frequently)

| Category | Tests |
|----------|-------|
| Memory Retrieval | 1.1, 1.3, 1.5 |
| Memory Creation | 2.2, 2.3, 2.5 |
| TODO Management | 3.1, 3.5, 3.7 |
| Skill Detection | 5.2, 5.3, 5.5 |

### Medium (Run Periodically)

| Category | Tests |
|----------|-------|
| Behavior Flags | 4.2, 4.3, 4.5 |
| Skill Workflows | 6.1, 6.3, 6.7 |
| Edge Cases | 8.1, 8.2, 8.6 |

### Low (Run on Related Changes)

| Category | Tests |
|----------|-------|
| Feedback | 7.1, 7.2, 7.3 |
| Explicit Skills | 9.1-9.5 |

## Model Selection Guide

| Model | When to Use | Examples |
|-------|-------------|----------|
| **haiku** | Simple tool usage checks, output pattern matching, trivial tasks | 1.2, 2.2, 3.5, 8.3, 8.4 |
| **sonnet** | Semantic matching, multi-step workflows, skill behavior | 1.5, 1.6, 2.1, 5.1, 6.1 |

**Cost Optimization:**
- Use haiku for tests that just check "did it read this file" or "does output contain X"
- Use sonnet for tests requiring reasoning about skill workflows or semantic retrieval

## Config Flag Reference

| Flag | Default | Usage |
|------|---------|-------|
| `AFK=true` | false | Set for tests where Claude should proceed autonomously |
| `AFK=false` | - | Default; Claude asks questions |

Tests that set `AFK=true`:
- 2.1, 2.2, 2.3, 2.4, 2.5, 2.7, 2.8 (memory creation)
- 3.6, 3.7 (TODO with context)
- 4.1, 4.2, 4.3, 4.5 (AFK behavior tests)
- 5.1, 5.2, 5.3, 5.4 (skill detection)
- 6.1, 6.2, 6.3, 6.4, 6.5, 6.6 (skill workflows)

## Fixture Requirements

### Minimal Fixtures (Most Tests)
```
notebook/
├── INDEX.md          # Empty or with 1-2 entries
└── entries/          # Empty or single entry
```

### Extended Fixtures (Retrieval Tests)
```
notebook/
├── INDEX.md          # Multiple entries
└── entries/
    ├── entry-a.md
    ├── entry-b.md
    └── entry-c.md
```

### Data Files
Most tests use simple CSV files with 3-10 rows. Examples:
- `data/samples.csv` - sample metadata
- `data/expression.csv` - gene expression values
- `data/gwas_results.csv` - GWAS summary stats

## Multi-Turn Tests

Some tests require multiple conversation turns:
- 7.1 Auto-Log on User Correction
- 7.2 Ask About Feedback on Skepticism
- 7.3 Feedback Not in INDEX.md
- 7.4 Feedback on Missed Skill Detection

For these, the framework needs to support:
1. Send prompt 1
2. Wait for response
3. Send prompt 2
4. Check assertions on final state

## Evaluator Prompts

Many tests use LLM-based evaluation for complex criteria. Guidelines:
- Keep evaluator prompts simple (yes/no questions)
- Include the actual content to evaluate ({{output}}, {{file_content:path}})
- Be specific about what passes vs. fails
- Use haiku for evaluators (fast, cheap)

## What's NOT Tested

Per user requirements:
- Git commit operations (hard to set up fixtures)
- Setting AFK flag via "(afk)" keyword (blocked)
- O2 cluster connectivity (external dependency)
- Hook shell script execution (tested separately)

## Converting Specs to YAML

Each test translates to YAML like this:

**Spec:**
```markdown
## Test 2.1: Entry Created for Substantive Work
Model: sonnet
Config flags: AFK=true
Prompt: Analyze the expression data...
Assertions:
1. file_created: notebook/entries/*.md
2. output_contains: BRCA1.*5\.0|TP53
```

**YAML:**
```yaml
name: memory-entry-created
description: Entry created for substantive work

config:
  model: sonnet
  replicates: 3
  pass_threshold: 0.67
  max_turns: 10
  behavior_flags:
    AFK: "true"
  allowed_tools:
    - Read
    - Write
    - Edit
    - Bash

setup:
  - command: "mkdir -p notebook/entries"
  - command: "echo '# Index\n\n| Date | Name | Summary |\n|--|--|--|' > notebook/INDEX.md"
  - command: "mkdir -p data && echo 'gene,sample1,...' > data/expression.csv"

prompt: |
  Analyze the expression data in data/expression.csv. Calculate the mean expression for each gene.

assertions:
  - type: file_created
    path: "notebook/entries/*.md"
    content_pattern: "## Summary"

  - type: output_contains
    pattern: "BRCA1.*5\\.0|TP53.*8\\.1|mean"
    case_insensitive: true

teardown:
  - command: "rm -rf notebook data"
```

## Recommended Test Order

When implementing, start with:
1. **2.1** - Core entry creation
2. **1.1** - Basic retrieval
3. **4.1** - AFK mode
4. **5.1** - perform-analysis detection

These four tests exercise the most important code paths.
