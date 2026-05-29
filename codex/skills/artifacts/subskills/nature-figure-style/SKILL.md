---
name: nature-figure-style
description: >-
  Presentation-style subset of a Nature/high-impact journal figure workflow. Use when the user asks to style, polish, critique, audit, or improve manuscript figures, multi-panel scientific plots, scientific slide figures, SVG/PDF/TIFF outputs, figure legends, panel layouts, or journal-ready visual presentation. Focuses on visual argument, hierarchy, typography, color, layout, export readability, and reviewer-risk checks. Not for dashboards, exploratory plots, Illustrator/Figma-first infographics, or choosing/implementing the full plotting backend.
---

# Nature Figure Style

Use this skill to make scientific figures read as a visual argument, not as isolated
pretty plots. Every figure starts from a claim, an evidence hierarchy, and a
review-risk check before aesthetics.

This is a presentation-style subset. It does not own full plotting infrastructure,
backend exclusivity, bundled demo scripts, or journal-specific submission verification.
Use the user's existing plotting stack unless they ask for a new implementation.

## Figure Contract Before Styling

Before editing figure code, captions, colors, or layout, establish the contract below.
Keep it concise.

```text
Core conclusion:
Figure archetype:
Target journal/output:
Final size:
Panel map:
  a:
  b:
  c:
Evidence hierarchy:
  hero evidence:
  validation evidence:
  controls/robustness:
Statistics needed:
Source data needed:
Image-integrity notes:
Reviewer risk:
```

The highest-priority rule is: the chart serves the scientific logic. Aesthetic polish,
template matching, and complex layout are subordinate to making the core conclusion
clear, defensible, and reviewable.

## Core Conclusion Rules

- The core conclusion should be one sentence with a verb: "Treatment X reduces Y by
  restoring Z", not "Treatment results".
- Every panel must answer a unique question. If removing a panel would not weaken the
  argument, remove or merge it.
- Separate primary evidence from supporting evidence. The primary evidence gets the
  hero panel or clearest axis; controls and robustness panels should be visually quieter.
- If the user provides data but no claim, infer a provisional claim from the request and
  ask for confirmation before final styling.

## Archetype Selection

Classify the figure before styling.

| Archetype | Use when | Hero panel | Supporting panels |
|---|---|---|---|
| `quantitative grid` | The claim is mainly numerical comparison | Optional, often a dominant summary metric | Shared axes, aligned scales, compact legends |
| `schematic-led composite` | A workflow, mechanism, device, or experimental design must be understood first | Left or top schematic, 35-60% of area | 2-4 quantitative validation panels |
| `image plate + quant` | Microscopy, imaging, histology, spatial overlays, segmentation, gels, blots, or representative images lead the evidence | Image plate or representative image | Scale bars, overlays, crops, quantification |
| `asymmetric mixed-modality figure` | The figure combines schematic, raster images, heatmaps, and quantitative plots | One panel spans rows/columns | Smaller panels ranked by evidence value |

## Panel Logic

Use this order unless the manuscript story clearly requires another:

1. Establish the system: sample, method, cohort, device, or experimental design.
2. Show the main effect or primary comparison.
3. Show mechanism or localization.
4. Quantify the representative image or qualitative observation.
5. Add robustness, controls, subgroup analysis, or sensitivity analysis.

For Fig. 1 or a method figure, the first panel often defines the visual vocabulary:
colors, symbols, workflow direction, sample classes, and scale. Reuse that vocabulary
through the whole figure and, where possible, through the manuscript.

## Aesthetic Integration

- Prefer one hero panel plus subordinate evidence panels over filling the canvas with
  equal-sized subplots.
- Keep one restrained palette per figure: usually one neutral family, one signal family,
  and one accent family.
- Keep the same condition, method, or sample color across all panels.
- Prefer direct labels over legends when categories are spatially fixed or the legend
  would force unnecessary eye travel.
- Use a shared legend area when repeated legends would waste space.
- Avoid equal-sized panels when the evidence is not equally important.
- Keep schematic colors and quantitative plot colors related. A schematic-led figure
  should look like one integrated argument, not a pasted collage.
- Keep backgrounds white for plots and diagrams; switch to black only for microscopy,
  image plates, or raster content that benefits from it.

## Typography And Export Style

- Use a simple sans-serif font stack such as Arial, Helvetica, DejaVu Sans, Liberation
  Sans, or the journal/template equivalent.
- Maintain a clear hierarchy: panel labels, plot text, axis labels, legends, annotations,
  caption/source notes.
- At final print size, dense figure text should usually remain readable around 5-7 pt;
  panel labels are usually slightly larger and bold.
- For manuscript figures, prefer SVG or PDF with editable text as the primary output.
  Raster previews are useful, but PNG alone is not enough for post-hoc adjustment.
- For matplotlib, keep SVG text editable with `svg.fonttype = "none"` and PDF text
  editable with `pdf.fonttype = 42`.
- For R, prefer `svglite`, `cairo_pdf`, and `ragg`/TIFF exports when publication raster
  output is required.

## Color Policy

- Prefer unified method families across panels over maximal hue separation.
- Avoid rainbow color maps.
- Do not rely on red/green as the only encoding.
- Make grayscale print interpretable when the figure may be reviewed or printed in
  monochrome.
- Reserve green/red mainly for gains, drops, and other directional cues.
- Use hatches, direct labels, shape, line style, or alpha when color alone would be
  fragile.

## Legend And Annotation Economy

- Use direct labels for stable line identities, channels, and fixed spatial regions.
- Move repeated legends into one shared legend strip or a dedicated legend panel.
- Keep annotations short and tied to evidence. Do not surround the hero panel with many
  equal-weight callouts.
- If a figure is dense, crop, split, or elevate the most important panel rather than
  shrinking everything into symmetry.

## Reviewer-Risk Prompts

Before finalizing, ask what a skeptical reviewer would challenge:

- Is the sample size visible in the legend, caption, or source data?
- Are error bars, intervals, and statistical tests defined?
- Are axes comparable across panels that invite comparison?
- Are representative images quantified and traceable to raw files?
- Are crop, contrast, pseudo-color, stitching, reuse, and raw-file provenance recorded
  for image panels?
- Are image adjustments global and documented?
- Could the same conclusion be made from fewer panels?
- Does every panel map back to the core conclusion?

## Delivery Checklist

When returning a style critique or revision, include:

- the revised or proposed core conclusion
- the figure archetype
- the panel hierarchy
- the main layout changes
- color and typography changes
- export/readability checks
- remaining scientific or reviewer risks

Keep the output practical. If the user asked for code, edit the plotting code and export
the requested files. If the user asked for critique, prioritize concrete changes over a
long design essay.
