---
name: map-modules
description: Manual-only workflow for decomposing a repository, package, or subsystem into high-level modules, boundaries, and relationships for developer understanding.
---

# Map Modules

Create a concise module map for a repository, package, or subsystem. Optimize for how a developer should understand the system: main workflow first, module boundaries second, inventory and evidence later.

## Intake

1. Treat the user's argument as the scope; if none is provided, use the current project.
2. Inspect project instructions, especially `project AGENTS.md`, for a `Module Boundaries` or equivalent section.
3. Use project-defined boundaries as authoritative unless code evidence clearly contradicts them.
4. Infer boundaries only when they are not already defined. Label inferred boundaries and offer a concise `project AGENTS.md` update.
5. Save Markdown by default. Prefer `notebook/<scope>-module-map.md` when a notebook exists.

## Investigation

Read enough code to make the map reliable: entry points, package structure, docs, imports, core call sites, and tests/examples that show intended use. Skip generated, vendored, cache, build, and dependency directories unless directly relevant. Use subagents only when the user explicitly asks for parallel agent work.

While reading, identify the important objects and transformations that cross module boundaries. These usually explain the system better than a file-by-file catalog.

## Boundary Selection

A module is a conceptual unit of responsibility. It may be a file, directory, symbol group, or cross-file subsystem.

Choose boundaries that make the system easier to reason about:

- Prefer a small number of production modules with clear ownership.
- Demote narrow helpers, kernels, schemas, fixtures, and tests into subcomponents unless they are independent design surfaces.
- Avoid huge mixed-purpose modules and tiny modules that create a noisy dependency graph.
- Separate workflow orchestration, external adapters, core domain transforms, inference/business logic, persistence/output, and support tooling when the code supports that split.
- For each suggested boundary, say what it owns and what it should not own.

Every listed module must include representative file links, important responsibilities, and boundary status: `Observed` or `Inferred`.

## Relationships

List modules in call-by order: workflow-level callers first, dependencies after the modules that depend on them.

Key relationships are intentional design dependencies, such as orchestration, parsing, transformation, validation, rendering, storage, adapters, schemas, protocols, or kernels. Use the format:

`A -> B`: A uses B to parse input records before model fitting.

Secondary relationships are dependencies that exist in code but may be weak, incidental, historical, misleading, or awkward. For each one, include direction, evidence, why it is secondary, and whether it is harmless, suspicious, or worth revisiting.

## Required Output

Use these top-level sections:

### 1. Primary Walkthrough

Start with the main call/data/dependency flow in the order a developer should read it. Reference the code as needed. Explain the key cross-boundary objects and transformations. Keep deep internals for `document-module`.

### 2. Suggested Module Boundaries

Give the recommended boundary decomposition. For each boundary include representative files, what it owns, what it should not own, and boundary status.

### 3. Module Map

Summarize the modules in call-by order. Keep this shorter than the walkthrough and boundary section; avoid repeating full evidence.

### 4. Key Relationships

List intentional module relationships in `A -> B` form.

### 5. Secondary Relationships And Boundary Drift

List weak, stale, private, circular, overly broad, or suspicious dependencies with evidence and a refactor hint when useful.

### 6. Suggested Reading Order

Give a short practical path through the project. Prefer useful orientation over complete inventory.

### 7. Scope, Evidence, And Project Instruction Update

State the inspected scope, project boundary source, files read, and important skips. If modules were inferred, include a concise `Module Boundaries` snippet suitable for `project AGENTS.md`; if boundaries already existed, say whether the map revealed drift.

## Validation

Before finishing, re-scan the main modules' imports/exports or call sites, verify caller-before-dependency ordering where possible, confirm every module has responsibilities and ownership boundaries, and check Markdown links.
