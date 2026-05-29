# Methods-First Mode

Use methods-first mode when the user specifies a key algorithm or statistical method should be aligned to a Methods artifact, such as a paper Methods section or supplementary note.

If the artifact already exists, use it as the specification. During planning, identify the artifact path and call out any conflict between the artifact, the code, and the user's request. Identify scientifically relevant gaps in the specification: "scientifically relevant" means that the specification is insufficient to reproduce the results. For example, a scientifically relevant gap is that the artifact refers to "lead SNPs" but does not say how these were defined; an irrelevant gap is that it gives the expression A^-1 B x without stating that this is evaluated as A \ (B * x) rather than (A \ B) * x.

If the artifact does not exist, create it with the user before implementation planning, consulting the $artifacts skill. This artifact-creation phase should not occur in Plan Mode; if necessary, ask the user to leave Plan Mode, draft the artifact, and iterate until it can serve as the specification. Use LaTeX unless the user specifies otherwise or does not have LaTeX installed.
