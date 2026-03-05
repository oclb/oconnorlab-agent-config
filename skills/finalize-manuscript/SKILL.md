---
name: finalize-manuscript
description: "Comprehensive pre-submission and resubmission manuscript checklist for academic papers. Runs 20 parallel checks covering citations, display items, factual claims, writing quality, data/code availability, cover letters, and response to reviewers. Supports AJHG and Nature Genetics policies. Use when: user says 'finalize manuscript', 'check manuscript', 'pre-submission check', 'manuscript checklist', 'ready to submit', or when preparing a paper for journal submission or resubmission."
disable-model-invocation: true
---

# Finalize Manuscript

Run 20 parallel checks on a manuscript before journal submission or resubmission. Produce a structured report with findings tagged as ISSUE, WARNING, or SUGGESTION.

## Phase 0: Setup & Conversion

Execute this phase before spawning check agents.

### Step 1: Gather Information

Ask the user:
1. **Target journal**: AJHG, Nature Genetics, or other
2. **Submission type**: Initial submission or revision?
3. **Files**: All manuscript files and their roles:
   - Main text (required)
   - Supplementary figures/tables
   - Supplementary note
   - Cover letter
   - Response to reviewers (revision only)

### Step 2: Convert Documents

Create working directory: `$TMPDIR/manuscript-check/`

For each `.docx` file:
```bash
pandoc input.docx -t markdown --extract-media=$TMPDIR/manuscript-check/media -o $TMPDIR/manuscript-check/output.md
```

For `.pdf` files: use the Read tool directly.
For `.xlsx` files: do not convert up front; load only when a check requires them.

Organize converted files as:
```
$TMPDIR/manuscript-check/
├── main-text.md
├── supplementary-note.md      # if applicable
├── response-to-reviewers.md   # if revision
├── cover-letter.md            # if applicable
├── media/                     # extracted images
└── report.md                  # created at end
```

If images extract as EMF/WMF, convert to PNG with LibreOffice:
```bash
libreoffice --headless --convert-to png image.emf
```

### Step 3: Build Structural Summary

Read the converted main text and extract:
- **Sections**: All headings and subheadings (especially Methods subsections)
- **Main display items**: All main figures and tables with captions
- **Supplementary display items**: All supplementary figures and tables
- **Extended data items**: If applicable
- **Author list**: Authors, affiliations, corresponding author(s), emails
- **Reference list**: All cited references; note format (numbered vs. author-year)
- **Preprints**: Any cited preprints (bioRxiv, medRxiv, arXiv, SSRN, etc.)

View each main figure image with the Read tool. Note what each figure shows (plot type, key labels).

Produce a `structural-summary` text block to include in every subagent prompt.

### Step 4: Assess Citation Formatting

Examine the converted reference list. If it appears garbled (pandoc artifacts, unreadable markup), ask the user whether to proceed with citation checks or skip them. "Garbled" means clearly unreadable, not a few formatting inconsistencies.

### Context Management

- **Always load into context**: Main text, main figure images, supplementary figure images
- **Load on demand**: Supplementary tables (often large .xlsx), supplementary note
- **For revisions**: Also load response to reviewers

## Dispatching Check Agents

After Phase 0, spawn one `general-purpose` agent per check, all in parallel with `run_in_background: true`. Each agent's prompt includes:
1. The working directory path
2. The structural summary
3. The check instructions (from the descriptions below)
4. The report format: tag each finding as `**ISSUE**`, `**WARNING**`, or `**SUGGESTION**`

### Model Assignments

| Category | Model | Checks |
|----------|-------|--------|
| Structural | sonnet | 4, 5, 6, 7, 9, 10, 20 |
| Citations | sonnet | 2, 3, 17 |
| Writing quality | opus | 18, 19 |
| Metadata | sonnet | 8, 15 |
| External verification | opus | 1, 13, 14 |
| Revision-specific | sonnet | 11, 12 |
| Interactive (deferred) | opus | 16 |

Skip revision checks (11, 12) for initial submissions. Skip cover letter checks (15, 16) if no cover letter.

---

## Check Descriptions

Include the relevant description verbatim in each subagent's prompt.

---

### Check 1: Fact-Check Literature Claims (Opus)

Read the main text. For every factual statement about existing literature or software, verify it:

- **Literature claims** (e.g., "Smith et al. showed X"): Search PubMed for the cited paper. Read the abstract; if insufficient, download the PDF to `{workdir}/tmp/` and inspect it. PubMed: `https://pubmed.ncbi.nlm.nih.gov/?term=QUERY`. Try PubMed Central for full text of paywalled papers.
- **Software claims** (e.g., "Tool X implements algorithm Y"): Find the tool's repository or documentation and verify.
- **Numerical claims** (e.g., "X% of Y in [citation]"): Verify specific numbers against the source.

Do not report claims that couldn't be verified — only report claims that are demonstrably incorrect or significantly misleading.

Tag: **ISSUE** (incorrect), **WARNING** (potentially misleading).

---

### Check 2: Citation Formatting (Sonnet)

Read the reference list. Check:
- Consistent format across all citations (author names, year, journal, volume, pages/DOI)
- In-text citation style is consistent (all numbered, or all author-year)
- No duplicate references
- No obvious formatting errors (missing fields, inconsistent punctuation)

If the reference list is garbled from pandoc conversion (unreadable markup, not just a few errors), report a single WARNING and stop — do not check individual citations.

Tag: **ISSUE** (formatting errors), **WARNING** (garbled list).

---

### Check 3: Citation Completeness (Sonnet)

Read the main text. For every mention of a method, software tool, database, or dataset:

1. **First mention**: Must be cited. Skip well-known concepts that don't need citation (e.g., "linear regression"). Specific implementations, tools, and datasets should be cited.
2. **Subsequent mentions**: A recitation is appropriate when:
   - It would not be clear without it what method/paper is being referred to
   - The method/paper is highly important in context and a reader would likely want to look it up from that sentence

Exception: Discussion section need not recite figure panels already cited in Results.

Tag: **ISSUE** (missing first citation), **SUGGESTION** (recommended recitation).

---

### Check 4: Display Items Cited in Order (Sonnet)

Read the main text. Extract every reference to main figures, main tables, supplementary figures, supplementary tables, and extended data figures.

Check:
1. **All cited**: Every display item from the structural summary is cited at least once in the main text or methods
2. **Cited in order**: First citations appear in ascending numerical order, checked separately per category (main figures, supplementary figures, etc.)

Tag: **ISSUE** (uncited items), **WARNING** (out-of-order).

---

### Check 5: Correct Figure/Table Cited (Sonnet)

Read the main text. For each citation of a supplementary figure or table, view the cited item (Read the image or load the table) and verify its content matches the text context.

Example: If the text says "significant enrichment in immune pathways (Supplementary Table 3)", verify Supplementary Table 3 actually contains pathway enrichment results.

Tag: **ISSUE** (clear mismatch — wrong number cited).

---

### Check 6: Main Figures Cite Supplementary Tables (Sonnet)

Read figure captions. For each main figure presenting numerical results graphically (bar charts, scatter plots, heatmaps with data), check that the caption cites a supplementary table with underlying numerical results.

Skip purely schematic, conceptual, or workflow figures.

Tag: **WARNING** (figure with numerical results but no supplementary table citation).

---

### Check 7: Author Affiliations & Emails (Sonnet)

Read the front matter. Check:
1. Every author has at least one affiliation
2. At least one corresponding author is designated
3. Every corresponding author has an email address

Tag: **ISSUE** (missing affiliations or emails).

---

### Check 8: Data & Code Availability (Sonnet)

Read the main text and find the data/code availability section. Read `references/journal-policies.md` from the skill directory for the target journal's policy.

Check:
1. Section exists
2. Conforms to the journal's required format and content
3. Accession numbers, DOIs, and repository URLs are present where required
4. Code availability is addressed (not just data)

If the target journal is not AJHG or Nature Genetics, search the web for that journal's policy.

Tag: **ISSUE** (missing section or major non-compliance), **WARNING** (minor gaps), **SUGGESTION** (improvements).

---

### Check 9: Acknowledgments & Funding (Sonnet)

Read the main text. Check:
1. Acknowledgments section exists
2. At least one funding source is mentioned (grant number, funding agency)

Tag: **WARNING** (missing acknowledgments), **SUGGESTION** (no funding mentioned).

---

### Check 10: Methods & Supplementary Note Cross-References (Sonnet)

Read the main text. Find every reference to "Methods", "the Methods section", "see Methods", "Supplementary Note", etc.

**Methods references:**
- Methods subsections should correspond ~1:1 with the order they're cited from the main text
- For each reference, verify it's clear which methods subsection is being cited based on sentence context and subsection titles
- Flag ambiguous references (multiple subsections could match)

**Supplementary Note references:**
- Each reference should make clear which part of the supplementary note is meant
- If unclear from context and subsection titles, suggest citing a specific subsection

Read `{workdir}/supplementary-note.md` if it exists to check subsection titles.

Tag: **WARNING** (ambiguous references), **SUGGESTION** (add specific subsection citations).

---

### Check 11: Response to Reviewers — Accuracy (Sonnet, revision only)

Read the response to reviewers and the main text. For each reviewer comment:
1. **Response exists**: Every comment has a response
2. **Action described**: Most responses describe what was changed; flag those that don't
3. **Quoted text accurate**: Any text quoted from the manuscript must match the current manuscript (minor formatting differences around citations are acceptable)
4. **Page/line numbers correct**: Verify any referenced page, line, or section numbers
5. **Figure/table references correct**: Verify any referenced display items exist and match context

Tag: **ISSUE** (incorrect quotes or references), **WARNING** (missing responses or actions).

---

### Check 12: Major Changes Described in Response (Sonnet, revision only)

Read the main text and response to reviewers. Identify major changes:
- Added or removed figures/tables
- Added or removed paragraphs
- New or changed scientific findings/conclusions
- Added or removed analyses

Check each is described somewhere in the response.

**Not major** (don't flag): clarity edits (even if touching many sentences), changes to acknowledgments/formatting, minor wording adjustments.

If a previous version is available, diff it. Otherwise, look for paragraphs that address topics no reviewer raised.

Tag: **WARNING** (major change not described in response).

---

### Check 13: Recent Missed References (Opus)

Read the main text to understand the paper's topic and scope. Review the reference list.

Perform **at least 5 different web searches** using varied keyword combinations to find recent papers (last 1-2 years) that might be relevant but aren't cited. Focus on:
- Direct methodological predecessors or competitors
- Recent applications of the same methods to similar data
- Recent papers on the same biological question
- Papers from the same research groups on related topics

Search PubMed, Google Scholar, and bioRxiv.

Tag: **SUGGESTION** with title, authors, year, link, and brief relevance explanation for each.

---

### Check 14: Preprints Now Published (Opus)

Read the reference list. Identify all cited preprints (bioRxiv, medRxiv, arXiv, SSRN, Research Square, etc.).

For each preprint:
1. Search by title on PubMed and Google Scholar
2. Search by first and last author names + keywords
3. Determine if a peer-reviewed version has been published

Tag: **SUGGESTION** with published citation details and link.

---

### Check 15: Cover Letter (Sonnet)

Read the cover letter (if none exists, report WARNING and stop).

Check:
1. **Letterhead**: Appears to be on institutional letterhead or contains institutional header
2. **Signature**: Signed (author name at end, possibly with title)
3. **Date**: Dated with a reasonable (recent) date
4. **Resubmission**: If revision, includes manuscript/tracking number

Tag: **ISSUE** (missing required elements), **WARNING** (no cover letter).

---

### Check 16: Suggested Reviewers (Opus, interactive — deferred)

Run this after all other checks complete.

Read the cover letter. If this is an initial submission and the cover letter doesn't include suggested reviewers, ask the user: "The cover letter does not include suggested reviewers. Would you like me to suggest some based on the manuscript's citations?"

If yes:
1. Identify key citations (most relevant to the paper's contribution)
2. Find the senior/corresponding author of each
3. Search for those authors' recent work to confirm active interest
4. Exclude anyone whose primary affiliation matches a corresponding author's
5. Present 4-6 suggestions with: name, affiliation, rationale, and a representative publication link

Tag: **SUGGESTION** with reviewer list.

---

### Check 17: Unsupported Statements (Sonnet)

Read the main text. Look for statements that should be supported by a citation, figure, or table but aren't:

1. **Results paragraphs**: Every results paragraph stating analysis outcomes should cite at least one display item
2. **Methods citing datasets**: If a methods subsection mentions "we analyzed N traits" or similar, those traits should usually be listed in a supplementary table, cited there
3. **Factual claims**: Statements about what is known or has been shown should cite a source
4. **Exception**: Discussion need not recite figure panels already cited in Results

Tag: **WARNING** (results without display items), **SUGGESTION** (other unsupported statements).

---

### Check 18: Spelling, Grammar & Writing Quality (Opus)

Read the full main text. Check for:

1. **Spelling errors** a word processor wouldn't catch: "compliment"/"complement", "principle"/"principal", "effect"/"affect", field-specific terms
2. **Grammatical errors**: Subject-verb agreement, dangling modifiers, incorrect prepositions
3. **Informal language**: Colloquialisms, contractions, overly casual phrasing
4. **Writing quality variation**: Identify paragraphs or sections where writing is noticeably weaker than the rest

Do not flag stylistic preferences or suggest rewrites. Only flag clear errors or notably weak writing.

Tag: **ISSUE** (clear errors), **SUGGESTION** (weak sections — identify the specific paragraph).

---

### Check 19: Terminology Consistency (Opus)

Read the full main text and view all figure images and table captions.

Check:
1. **Consistent terms**: Same concept always referred to by the same term; flag if two terms are used for one thing
2. **Text-figure consistency**: Terms and abbreviations in figure labels/legends match the text
3. **Text-table consistency**: Same for tables
4. **Capitalization consistency**: Terms capitalized the same way throughout

Tag: **WARNING** (inconsistencies, with specific locations).

---

### Check 20: Undefined Acronyms (Sonnet)

Read the main text. Find every acronym (2+ capital letters, possibly with numbers).

For each:
1. Check if defined at first use (e.g., "genome-wide association study (GWAS)")
2. Check if defined in figure captions where it first appears in a figure
3. Use judgment for universally understood acronyms in the field (DNA, RNA, SNP, etc.)

Tag: **WARNING** (undefined acronyms, noting where they first appear).

---

## Report Assembly

After all check agents complete, compile `{workdir}/report.md`:

```markdown
# Manuscript Finalization Report

**Date:** {date}
**Target Journal:** {journal}
**Submission Type:** {initial/revision}

## Summary
- Issues: {count}
- Warnings: {count}
- Suggestions: {count}

## Issues (must fix)
[All ISSUE findings, grouped by check number and title]

## Warnings (should review)
[All WARNING findings, grouped by check number and title]

## Suggestions (consider)
[All SUGGESTION findings, grouped by check number and title]

## Checks Passed
[List of checks with no findings]
```

Present the report to the user.

## Post-Check Interactive Phase

After presenting the report:
1. **Citation formatting** (Check 2): If references were garbled, ask how to handle
2. **Suggested reviewers** (Check 16): If applicable, run this deferred check
3. **Batch other questions**: Present any questions that arose during checks
4. **Follow-up**: Offer to help fix identified issues
