---
name: tikz-flowchart
description: >
  Create publication-quality TikZ/LaTeX flowcharts, architecture diagrams, and schematic figures.
  Use when the user asks to create a diagram, flowchart, schematic, figure with boxes and arrows,
  or architecture diagram for a paper or writeup. Trigger words: "diagram", "flowchart", "figure",
  "schematic", "TikZ", "tikz", "boxes and arrows", "architecture diagram".
---

# TikZ Flowchart Skill

Create publication-quality flowcharts and schematic diagrams using TikZ/LaTeX.

## Workflow

1. Start from `assets/template.tex` -- copy to target location and customize
2. Read `references/gotchas.md` before writing any TikZ code (critical pitfalls)
3. Compile with `pdflatex -interaction=nonstopmode <file>.tex`
4. Inspect the PDF output with the Read tool, iterate

## Quick patterns

### Node with title + sublabels

```latex
\newcommand{\sub}[1]{\fontsize{7}{9}\selectfont\sffamily\color{sublabel}#1}
\node[mystyle] (id) at (x,y) {\textbf{Title}\\\sub{line 1}\\\sub{line 2}};
```

CRITICAL: Never wrap multiline `\\` content in braces. See `references/gotchas.md`.

### Forward arrow

```latex
\draw[arr] (nodeA) -- (nodeB);
```

### Loop-back arrow (underneath)

```latex
\draw[arr, rounded corners=8pt]
  ([yshift=-2pt]B.south) -- ++(0,-0.9) -| ([yshift=-2pt]A.south);
```

### Self-loop

```latex
\draw[arr, rounded corners=8pt]
  ([xshift=5pt]N.south) -- ++(0,-0.9) -| ([xshift=-5pt]N.south);
```

### Multi-panel

```latex
\begin{scope}[shift={(13,0)}, local bounding box=panelB]
  % nodes and arrows
\end{scope}
\node[font=\bfseries\large] at ($(panelB.north)+(0,0.35)$) {(b) Title};
```

### Dashed boundary (background)

```latex
\begin{scope}[on background layer]
  \node[draw=black!30, dashed, rounded corners=8pt, inner sep=10pt,
        line width=0.8pt, fit=(nodeA)] (box) {};
\end{scope}
```

## Page sizing

No `standalone` on BasicTeX. Use:
```latex
\documentclass{article}
\usepackage[paperwidth=Wcm,paperheight=Hcm,margin=0.4cm]{geometry}
\pagestyle{empty}
```

Set `paperwidth`/`paperheight` to fit content with minimal whitespace.

## Color scheme convention

Define fill + border pairs for each semantic category:
```latex
\definecolor{catA}{RGB}{210,228,248}    % fill
\definecolor{catAline}{RGB}{90,135,195} % border
```

Use soft pastels for fills, deeper saturated versions for borders.
