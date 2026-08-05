---
name: artifacts
description: Use when creating, editing, converting, polishing, checking, or inspecting public-facing artifacts such as DOCX, PDF, PPTX, TikZ diagrams, manuscript figures, or manuscripts being prepared for journal submission.
recommended_scope: global
---

# Artifacts

Choose the appropriate internal subskill for the artifact type, then read only that subskill's `SKILL.md` and supporting files:

- `subskills/docx/` for Word documents.
- `subskills/pdf/` for PDF files and forms.
- `subskills/pptx/` for PowerPoint presentations.
- `subskills/tikz-flowchart/` for TikZ/LaTeX flowcharts, schematics, and diagrams.
- `subskills/polished-manuscript-figure/` for manuscript figure styling, critique, and journal-ready visual presentation.
- `subskills/finalize-manuscript/` for pre-submission and resubmission manuscript checklists; use when the user says "finalize manuscript", "check manuscript", "pre-submission check", "manuscript checklist", "ready to submit", or asks to prepare a manuscript for journal submission.

Artifacts should have a high signal-to-noise ratio: prioritize the message, evidence, and user's concrete purpose over decoration or exhaustive process notes.

If an artifact changed since you last read it, reread it and inspect the diff before editing; the changes may belong to the user and must not be discarded or overwritten.

Scientific writing should be factual rather than defensive: describe the analysis performed and state material caveats in proportion to their scientific importance, keeping alternatives that are irrelevant to the reader out of the prose. Prefer "we did X" to "we did X, not Y"; for example, "Confidence intervals were obtained by dividing the numerator's interval by the denominator, treating the more precisely estimated denominator as fixed" usually states the method and caveat fully.

Artifacts meant for public consumption should avoid path dependence. Do not refer to previous versions, user prompts, or the process that produced the artifact; those references would not make sense to a reader besides the user.
