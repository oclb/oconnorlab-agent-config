---
name: module-codebase-review
description: Manual-only workflow for codebase audit, codebase review, release-check, paper-freeze-check, and module-scoped multi-agent read-only bug/code-smell audits that produce a prioritized notebook report and TODO-ready findings. Use this instead of maintain-project for requests that ask to audit or review code behavior.
---

# Module Codebase Review

Use this skill when the user asks to thoroughly review, audit, release-check, or paper-freeze-check a repository using subagents broken down by module.

## Intake

Before spawning reviewers, align with the user on the finding profile. Ask which of these they want unless they already specified it:

1. Bugs only.
2. Major bugs only: only high-severity bugs that could affect scientific conclusions.
3. Bugs plus code smells.
4. Whether to include test coverage gaps.

For code smells, clarify which kinds count for this audit: unnecessarily entangled or complex logic, duplication, awkward constructions, or inconsistent local patterns such as most of the codebase using pattern X while one area uses pattern Y. Record the chosen profile in the audit report and pass it verbatim to every reviewer.

## Workflow

1. Read `AGENTS.md`, `notebook/INDEX.md`, existing module maps, README/docs, package metadata, test layout, and recent relevant notebook entries.
2. Define review packets by conceptual module. Prefer 4-10 logical modules. Use per-file packets only for dense public APIs or when the user asks for maximum depth.
3. Run cheap baseline validation where practical: parse/compile, focused tests, CLI help, package/build smoke checks.
4. Spawn parallel reviewers, one per packet. Each reviewer must return only evidence-backed findings, follow the chosen finding profile, and may return zero findings.
5. Synthesize findings across reviewers. Deduplicate, preserve provenance, rank by severity, and split findings into `decision needed`, `safe fix`, `coverage gap`, and `cleanup`.
6. Save a notebook entry for the audit. Create TODO candidates only after synthesis, with `Backed by:` links to the audit entry.

## Reviewer Bar

Accept findings only when they match the chosen finding profile and identify a concrete risk. Possible categories are correctness, numerical behavior, failure/status contracts, API/CLI/package/docs drift, schema/path/provenance drift, missing tests at high-risk seams, dead duplicate code, security, performance, and the explicitly requested code-smell categories. Architecture findings must be tied to observed bugs, repeated drift, or concrete maintenance risk.

Every finding must include severity, module, file/line evidence, risk, suggested fix, and test/validation implication. Reviewers should prefer zero findings over weak findings.

## Output

Use sections: Summary, Finding Profile, Verification, High-Priority Findings, Other Findings, Coverage Gaps, Files Reviewed With No Material Findings, Suggested Sequencing, References.
