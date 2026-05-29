# Finalize Manuscript

Pre-submission and resubmission checklist for academic manuscripts. Runs 20 parallel checks and produces a structured report.

## Usage

Ask for a manuscript check through the `artifacts` skill, for example:

```text
finalize this manuscript
```

Provide manuscript files (docx, pdf, xlsx) when prompted.

## Checks

| # | Check | Depth |
|---|-------|-------|
| 1 | Fact-check literature claims | Deep review |
| 2 | Citation formatting | Standard review |
| 3 | Citation completeness (first mention + recitation) | Standard review |
| 4 | Display items cited in order, all referenced | Standard review |
| 5 | Correct figure/table cited | Standard review |
| 6 | Main figures cite supplementary tables | Standard review |
| 7 | Author affiliations & emails | Standard review |
| 8 | Data & code availability vs. journal policy | Standard review |
| 9 | Acknowledgments & funding | Standard review |
| 10 | Methods/supplementary note cross-references | Standard review |
| 11 | Response to reviewers — accuracy (revision) | Standard review |
| 12 | Major changes described in response (revision) | Standard review |
| 13 | Recent missed references | Deep review |
| 14 | Preprints now published | Deep review |
| 15 | Cover letter checks | Standard review |
| 16 | Suggested reviewers (interactive) | Deep review |
| 17 | Unsupported statements | Standard review |
| 18 | Spelling, grammar & writing quality | Deep review |
| 19 | Terminology consistency | Deep review |
| 20 | Undefined acronyms | Standard review |

## Supported Journals

Embedded policies for:
- **AJHG** (American Journal of Human Genetics)
- **Nature Genetics**

Other journals: policies looked up at runtime.

## Workflow

1. **Phase 0 (Setup)**: Gather files, convert docx to markdown via pandoc, extract images, build structural summary
2. **Checks 1-20**: Run in parallel as subagents
3. **Report**: Compile findings as ISSUE / WARNING / SUGGESTION
4. **Interactive**: Batch follow-up questions (suggested reviewers, citation formatting issues, etc.)

## Document Handling

- `.docx` → markdown via `pandoc --extract-media`
- `.pdf` → read directly
- `.xlsx` → loaded on demand (supplementary tables)
- Images extracted as PNG for multimodal inspection
