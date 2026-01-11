# Revise Scientific Writing Plugin

A Claude Code skill that applies O'Connor lab scientific writing principles to revise and improve scientific papers, manuscripts, and documents.

## What It Does

When you ask Claude to revise, review, or improve scientific writing, this skill automatically applies a systematic framework based on proven writing principles focused on clarity, structure, and persuasiveness.

## Core Philosophy

**Write for your readers.** Your paper should convince reviewers, satisfy aficionados, and engage casual readers.

## Key Principles Applied

### 1. Point-First Style
Don't save up for a big reveal. Put the conclusion first, then support with evidence.

**Before**: "We performed simulations with 1000 replicates. Effect sizes varied from 0.01 to 0.1. We found the estimator is unbiased."

**After**: "The estimator is unbiased in simulations (N=1000 replicates, effect sizes 0.01-0.1)."

### 2. Judicious Emphasis
Emphasize important content through position, repetition, naming, and structure. Don't emphasize routine or self-serving content.

**Remove**: "We performed a sophisticated analysis..." "Our innovative method..."
**Keep**: Key findings, important distinctions, novel concepts

### 3. Cohesiveness
Sentences and paragraphs connect logically through keyword repetition, linking words, and referencing previous concepts.

### 4. Embedding in Literature
Connect to prior work throughout the paper, not just in Introduction.

## When to Use

This skill activates when you ask to:
- "Revise this manuscript"
- "Review my introduction"
- "Edit this abstract"
- "Improve the clarity of this section"
- "Critique this draft"
- "Apply writing principles to this text"

## What Gets Revised

### Complete Manuscripts
- Title
- Abstract
- Introduction
- Results
- Discussion
- Methods
- Display items (figures/tables)
- Supplement

### Individual Sections
Can revise any section independently with section-specific guidance.

### Conference Abstracts
Special formatting for ASHG, ISMB, etc.

## Section-Specific Guidance

### Title
Three approaches:
1. **Subject matter**: "The common-variant effect size distribution"
2. **Methodological advance**: "Partitioning heritability without eQTLs"
3. **Key finding**: "Polygenicity explained by negative selection"

Optionally include one key detail or surprising element.

### Abstract
Standard structure:
1. Broad context (1 sentence)
2. Specific gap/question (1-2 sentences)
3. What you did (1-2 sentences)
4. What you found (2-4 sentences)
5. Implications (1-2 sentences)

Written for broad audience, self-contained.

### Introduction
**Standard (short) structure:**
- Para 1: Context → gap/question
- Para 2 (optional): Significance/rationale
- Para 3-N: Prior literature
- Para N+1 (optional): Why challenge is nontrivial
- Final para: Roadmap (what you DID)

**Long introduction**: Can build narrative, trace historical evolution.

### Results
**Organization:**
- Subsection 1: Study design/methodological overview
- Subsection 2 (optional): Validation
- Subsection 3: Topline results (written for 100% of audience)
- Subsections 4-N: Secondary questions
- Final subsection (optional): Coda

**Key**: 4-7 subsections, point-first throughout, each addresses one question.

### Display Items
- One finding per panel
- Visually self-evident
- Professional production quality
- Factual captions (not narrative)
- "Comparison of methods" not "Our method wins"

### Discussion
**Structure:**
- Para 1: Review + new context
- Para 2-N: Implications, limitations, connections
- Last para (optional): Specific limitations (3-5 items)

Open-ended, can raise questions. Each paragraph could prompt journal club discussion.

### Methods
**Purpose**: Enable exact replication.

**Includes:**
- Data description
- Mathematical models
- Algorithms/procedures
- Simulation details
- Existing method parameters
- Justification of choices

### Supplement
- Supplementary note: Mathematical derivations, proofs
- Supplementary figures: Secondary analyses (can have long captions)
- Supplementary tables: All numerical results

## Example Usage

### Revising an Introduction

```
You: Review my introduction section

Claude: [Applies revise-scientific-writing skill]

SECTION: Introduction

STRENGTHS:
- Good command of relevant literature
- Clear gap statement in first paragraph

SUGGESTIONS:

1. First paragraph takes too long to get to the point
   Current: "GWAS have revolutionized our understanding of complex
   traits over the past 15 years, enabling the discovery of thousands
   of associated variants across hundreds of traits. However, despite
   these successes, many challenges remain."

   Revised: "GWAS have identified thousands of variants, but the
   mechanisms by which variants influence traits remain unclear."

   Reason: Get to the gap immediately (point-first style)

2. Roadmap paragraph emphasizes findings over approach
   Current: "Here we show that genes mediate 34% of heritability..."
   Revised: "Here we develop the Gene Mediation Model to decompose
   gene-mediated heritability. We apply this approach to..."

   Reason: Roadmap should emphasize what you DID

[continues with more suggestions]
```

### Revising an Abstract

```
You: Edit this abstract for Nature Genetics submission

Claude: [Applies section-specific abstract guidelines]

SUGGESTIONS:

1. Opening too narrow
   Current: "Linkage disequilibrium (LD) is a key confound..."
   Revised: "GWAS associations reflect both causal effects and LD.
   However, existing methods to account for LD are..."

   Reason: Start broader for general audience

2. Too much methodology detail
   [Suggests condensing methods section]

3. Missing broader implications
   [Suggests adding 1-2 sentences on impact]
```

## Common Issues Detected

| Issue | Detection | Fix |
|-------|-----------|-----|
| Burying the lede | Main finding mid-paragraph | Restructure: conclusion first |
| Over-emphasis | "We performed comprehensive..." | Remove self-praise |
| Lack of cohesion | Jarring transitions | Add linking sentences |
| Narrative figure captions | "We computed X and found Y" | Factual: "X values for Y dataset" |
| Missing point-first | Evidence before conclusion | Flip order |
| Vague roadmap | "We found several interesting..." | Specific: what you DID |

## Revision Output Format

When you receive revision feedback, it's organized as:

### 1. Summary
Overall assessment, main strengths, key improvements needed

### 2. Section-by-Section Feedback
For each section:
- Specific suggestions
- Before/after examples
- Reasoning

### 3. Prioritized Action Items
- **High priority**: Structural issues, missing point-first
- **Medium priority**: Wordiness, missing connections
- **Low priority**: Minor word choices

### 4. Checklist Status
Section-specific criteria: what's working, what needs work

## Installation

Already included if you're using the claude-config repository with `pluginDirs` configured.

## Customization

Edit `skills/revise-scientific-writing/SKILL.md` to:
- Adjust principles for your field
- Add field-specific terminology
- Include lab-specific conventions
- Modify emphasis on certain principles

## Best Practices

### Before Asking for Revision

1. **Be specific** about what you want reviewed:
   - "Revise the introduction" (one section)
   - "Review the entire manuscript" (comprehensive)
   - "Edit this abstract for Nature" (context-specific)

2. **Provide context**:
   - Journal target (Nature, Nature Genetics, PLoS Genetics, etc.)
   - Audience (broad vs. specialized)
   - Stage (early draft vs. pre-submission)

3. **Have the full text ready**:
   - Paste section or provide file path
   - Include surrounding context if revising one section

### During Revision

- Review suggestions critically
- Preserve scientific content
- Maintain your voice
- Balance principles (sometimes trade-offs exist)

### After Revision

- Apply high-priority suggestions first
- Check if changes maintain cohesiveness
- Re-read revised version for flow
- Iterate if needed

## Examples by Document Type

### Journal Manuscript
Full systematic review of all sections with detailed feedback

### Conference Abstract
- Focus on immediate point delivery
- More detail than journal abstract allowed
- Remember: read back-to-back with dozens of others

### Revision Letter Response
- Point-first responses to each reviewer comment
- Clear structure: concern → your response → changes made

### Grant Proposal
- Similar principles apply
- Emphasis on significance and innovation
- Clear specific aims

## Integration with Other Skills

Works well with:
- **perform-analysis**: Revise Results section after analysis complete
- **sanity-check-data**: Ensure Methods section describes data accurately

## Version

Current version: 1.0.0

Based on O'Connor Lab Scientific Writing Guide

## Tips

### For Maximum Benefit

1. **Apply early and often**: Revise at draft stage, not just before submission
2. **One section at a time**: Easier to apply targeted feedback
3. **Iterate**: Revise, re-read, revise again
4. **Teach the principles**: Understanding WHY helps you apply them

### Common Misconceptions

**Myth**: Point-first means no narrative
**Reality**: Narrative is fine, but main argument stays in foreground

**Myth**: All emphasis is bad
**Reality**: JUDICIOUS emphasis is key; emphasize important content

**Myth**: Writing principles are formulaic
**Reality**: They're guidelines that improve clarity while preserving your voice

## Troubleshooting

**Skill not activating?**
- Use explicit phrases: "revise", "review", "edit"
- Restart Claude Code session to load plugins

**Feedback too detailed?**
- Ask for "high-level feedback only"
- Request "focus on structure not wording"

**Feedback not specific enough?**
- Paste the actual text you want revised
- Specify which principles to emphasize

## Future Enhancements

Planned additions:
- Field-specific variations (biology, physics, CS)
- Templates for common document types
- Before/after examples library
- Integration with writing progress tracking
