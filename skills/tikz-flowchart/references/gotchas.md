# TikZ Flowchart Gotchas

## Critical: Multiline text in nodes

In `align=center` nodes, **never wrap `\\`-delimited content in braces**:

```latex
% BROKEN - silent failure, cryptic "Undefined control sequence" error
\node[draw, align=center] {Title\\{\footnotesize sub1,\\sub2}};

% BROKEN - same issue with \\[3pt] optional argument
\node[draw, align=center] {\textbf{Title}\\[3pt]{\footnotesize sub}};

% WORKS - apply font change per-line, no wrapping braces
\node[draw, align=center] {\textbf{Title}\\\footnotesize sub1,\\\footnotesize sub2};

% BEST - use a \newcommand wrapper applied per-line
\newcommand{\sub}[1]{\fontsize{7}{9}\selectfont\sffamily\color{gray}#1}
\node[draw, align=center] {\textbf{Title}\\\sub{sub1,}\\\sub{sub2}};
```

The `\\` parser inside TikZ node text cannot handle being inside a brace group. This causes errors like:
- `Undefined control sequence. \pgfutil@next`
- `\begin{tikzpicture} ended by \end{scope}`
- Empty/tiny PDF output with exit code 0

## BasicTeX (macOS) missing packages

BasicTeX (`brew install --cask basictex`) does not include `standalone.cls`. Workaround:

```latex
\documentclass{article}
\usepackage[paperwidth=20cm,paperheight=10cm,margin=0.4cm]{geometry}
\pagestyle{empty}
```

Adjust `paperwidth`/`paperheight` to fit content. This produces a cropped single-page PDF similar to `standalone`.

## Arrow tips

`>=Stealth` requires `\usetikzlibrary{arrows.meta}`. Without it, use `>=stealth` (lowercase, from legacy `arrows` library) or `>=latex`.

## Loop-back arrows

Use `rounded corners` on the draw command, not as a node style:

```latex
% From node B bottom, down, left, up to node A bottom
\draw[->, thick, rounded corners=8pt]
  ([yshift=-2pt]B.south) -- ++(0,-0.9) -| ([yshift=-2pt]A.south);
```

The `-|` path operator draws horizontal-then-vertical; `|-` draws vertical-then-horizontal.

## Self-loops

Offset the start/end x-coordinates to create a visible loop:

```latex
\draw[->, rounded corners=8pt]
  ([xshift=5pt]node.south) -- ++(0,-0.9) -| ([xshift=-5pt]node.south);
```

## Multi-panel figures

Use `\begin{scope}[shift={(X,0)}]` with `local bounding box=name` for each panel. Reference panel bounds for titles:

```latex
\begin{scope}[shift={(13,0)}, local bounding box=panelB]
  % ... nodes and arrows ...
\end{scope}
\node[font=\bfseries\large] at ($(panelB.north)+(0,0.35)$) {(b) Title};
```

## Background layers

For elements behind others (dashed boundaries, shading):

```latex
\usetikzlibrary{backgrounds}
\begin{scope}[on background layer]
  \node[draw=black!30, dashed, fit=(nodeA)(nodeB)] {};
\end{scope}
```
