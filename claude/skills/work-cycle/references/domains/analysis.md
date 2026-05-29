# Analysis Domain

Use this reference for analyses. An analysis is a question answered with data, a method, and an interpretation.

## Classify The Analysis

First decide whether the task is a variation of a common workflow or a one-off/novel analysis.

For common workflows, look for relevant project-specific skills and similar notebook entries before planning. If a relevant skill exists, use it. Use previous runs as templates for data locations, scripts, parameters, outputs, resource needs, and known pitfalls.

If no project-specific skill exists but the workflow appears reusable, finish the immediate analysis first unless the user asked for skill-building; then suggest adding a project skill, naming the specific reusable knowledge it would capture.

For one-off or novel analyses, design from first principles. If the analysis requires new reusable code, package changes, pipeline changes, tests, or a substantial reusable script, also consult `software.md`.

## Scientifically Relevant Choices

Identify choices that do not have established defaults and could plausibly affect interpretation of results. Align with the user before execution unless AFK mode applies.

Common interpretation-affecting choices include data subset, inclusion/exclusion filters, comparator methods, parameter settings, covariates, random seeds, number of replicates, correction strategies, model families, and what counts as the primary output.

Do not ask about irrelevant implementation details that do not affect the result's scientific meaning.

## Planning

Before running the analysis, establish:

1. The motivation: what the result will decide, support, or rule out.
2. The data: provenance, freshness, sample definitions, format, missingness, and whether validation is needed.
3. The method: statistical or computational approach, assumptions, expected outputs, and resource needs.
4. The interpretation: expected result, null meaning, caveats, and comparison to prior related runs.

Pilot long or risky analyses before full execution. For resource-intensive work, use the project's remote-compute guidance.

## Execution And Reporting

Record enough environment information to make the run interpretable: repo commit or dirty state, key tool/package versions, seed when relevant, and important command lines.

Store one-off analysis scripts and outputs under the notebook entry by default, unless the project has an established analysis location.

Report the key takeaway with actual numbers, uncertainty when available, caveats, and concordance or discordance with related analyses.

## Notebook

Analyses normally create or update notebook entries, except for tiny checks or disposable exploration. Notebook entries should capture motivation, data, method, execution notes, outputs, and findings.
