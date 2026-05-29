---
name: document-module
description: Manual-only workflow for documenting a module, file, package, or subsystem for developer understanding.
---

# Document Module

Create a developer-facing module document that helps the current engineer understand the codebase right now. Treat the user's argument as the scope: a file, module, function, etc., at the requested level of detail.

**Goal: to help the developer understand the target module, maximizing signal-to-noise ratio.**

## Intake

1. Identify the requested target and any constraints in the user's argument, such as desired level of detail. If the target is ambiguous, ask one concise clarification. If the desired level of detail is ambiguous, assume it should be "default".
2. Choose the output format: Markdown by default; LaTeX when equations would materially improve the document, like when the target module implements a statistical inference procedure. If using LaTeX, it should be compiled.
3. Read the target code; identify nearby tests, docs, key dependencies, and external call sites; read these adjacent items if needed in order to understand the purpose or functionality of the target.

## Required Document Structure

The document must have exactly these three top-level content sections.

### 1. Public Surface

Describe the module's externally visible API or other surface. This does not need to enumerate every option or flag unless the user requests a high level of detail, but it must identify the surface that other code depends on:
- exported functions, commands, configurations, file formats, etc. which are used outside of the module
- common call patterns
- return values, side effects, errors, and invariants
- where non-trivial or core logic lives

### 2. Relationships

Describe relationships with the rest of the codebase:
- where this module fits within a workflow or pipeline
- what the module depends upon
- what depends upon it
- if it wraps around core logic that is implemented elsewhere, a concise description of that logic
- the reason for any non-obvious relationships
- any relationships that seem unnecessary or overly entangled
- external dependencies that are tied to this module specifically but not generic dependencies like NumPy

Do not recursively unfold the full dependency graph. Stop at the level needed to understand why this module calls a dependency or why another module calls it. 

### 3. Internal Logic

Describe how the module works internally. The default level of detail should be enough for a developer to safely change the module and for a scientist or analyst to understand scientifically relevant computations.

Include:

- the internal structure: main control flow, key internal functions, key objects or data structures
- core algorithms or equations
- scientifically relevant parameters, data filters, heuristics, or thresholds
- any logic which may be hard to understand
- the apparent reasons for non-obvious design choices, such as computational optimizations

Avoid line-by-line paraphrase. Focus on conceptual structure and decision points that affect behavior.

### 4. Testing

Describe what tests assert about the behavior of the module or about its integration into the larger codebase. 

## Style And Evidence

- You are writing for the human developer of a largely AI-written codebase
- Use point-first style: put more important content first, except when logical ordering takes precedence
- Include clickable links to key files
- Distinguish observed facts from inferences
- Preserve repo terminology; avoid software engineering jargon that may be unfamiliar to a scientific programmer

## Validation

Before finishing:

1. Re-scan the target's exports/imports and at least one external call-site search to catch missed public surface.
2. Check the generated Markdown or LaTeX for broken links or compilation issues.
3. Save the artifact in the notebook if it exists.
3. Open the artifact (.md or .pdf) using system default viewer.
