# Finalize Manuscript

Pre-submission and resubmission checklist for academic manuscripts. Runs 20 parallel checks and produces a structured report.

## Usage

```
/finalize-manuscript
```

Provide manuscript files (docx, pdf, xlsx) when prompted.

## Checks

| # | Check | Model |
|---|-------|-------|
| 1 | Fact-check literature claims | Opus |
| 2 | Citation formatting | Sonnet |
| 3 | Citation completeness (first mention + recitation) | Sonnet |
| 4 | Display items cited in order, all referenced | Sonnet |
| 5 | Correct figure/table cited | Sonnet |
| 6 | Main figures cite supplementary tables | Sonnet |
| 7 | Author affiliations & emails | Sonnet |
| 8 | Data & code availability vs. journal policy | Sonnet |
| 9 | Acknowledgments & funding | Sonnet |
| 10 | Methods/supplementary note cross-references | Sonnet |
| 11 | Response to reviewers — accuracy (revision) | Sonnet |
| 12 | Major changes described in response (revision) | Sonnet |
| 13 | Recent missed references | Opus |
| 14 | Preprints now published | Opus |
| 15 | Cover letter checks | Sonnet |
| 16 | Suggested reviewers (interactive) | Opus |
| 17 | Unsupported statements | Sonnet |
| 18 | Spelling, grammar & writing quality | Opus |
| 19 | Terminology consistency | Opus |
| 20 | Undefined acronyms | Sonnet |

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
