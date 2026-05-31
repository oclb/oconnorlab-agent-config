#!/usr/bin/env python3
"""Build the Lab Agent Config Update presentation without external packages."""

from __future__ import annotations

import html
import math
import zipfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable


OUT = Path(__file__).with_name("lab-agent-config-update.pptx")

EMU = 914400
SLIDE_W = 13.333333
SLIDE_H = 7.5
SLIDE_W_EMU = int(SLIDE_W * EMU)
SLIDE_H_EMU = int(SLIDE_H * EMU)
MIN_FONT_SIZE = 18

COLORS = {
    "paper": "F7F3EA",
    "ink": "22252A",
    "muted": "5D646B",
    "teal": "1F7A7A",
    "teal_dark": "135C5B",
    "amber": "C69214",
    "coral": "D85A4A",
    "blue": "506B8B",
    "pale": "E8EFEA",
    "white": "FFFFFF",
    "line": "D6D0C4",
}


@dataclass
class Shape:
    xml: str


def esc(value: str) -> str:
    return html.escape(value, quote=True)


def emu(value: float) -> int:
    return int(value * EMU)


def pt(value: float) -> int:
    return int(value * 100)


def text_run(
    text: str,
    size: float,
    color: str = "ink",
    bold: bool = False,
    italic: bool = False,
    font: str = "Arial",
) -> str:
    size = max(size, MIN_FONT_SIZE)
    attrs = [f'sz="{pt(size)}"', 'lang="en-US"']
    if bold:
        attrs.append('b="1"')
    if italic:
        attrs.append('i="1"')
    return (
        f"<a:r><a:rPr {' '.join(attrs)}>"
        f'<a:solidFill><a:srgbClr val="{COLORS[color]}"/></a:solidFill>'
        f'<a:latin typeface="{esc(font)}"/>'
        f"</a:rPr><a:t>{esc(text)}</a:t></a:r>"
    )


def para(
    text: str,
    size: float = 20,
    color: str = "ink",
    bold: bool = False,
    italic: bool = False,
    align: str = "l",
    font: str = "Arial",
    line_spacing: int | None = None,
) -> str:
    ppr = f'<a:pPr algn="{align}">'
    if line_spacing:
        ppr += f'<a:lnSpc><a:spcPct val="{line_spacing}"/></a:lnSpc>'
    ppr += "</a:pPr>"
    return f"<a:p>{ppr}{text_run(text, size, color, bold, italic, font)}</a:p>"


def rich_para(
    runs: Iterable[tuple[str, dict]],
    size: float = 20,
    align: str = "l",
    line_spacing: int | None = None,
) -> str:
    ppr = f'<a:pPr algn="{align}">'
    if line_spacing:
        ppr += f'<a:lnSpc><a:spcPct val="{line_spacing}"/></a:lnSpc>'
    ppr += "</a:pPr>"
    body = []
    for text, opts in runs:
        body.append(text_run(text, opts.get("size", size), opts.get("color", "ink"), opts.get("bold", False), opts.get("italic", False), opts.get("font", "Arial")))
    return f"<a:p>{ppr}{''.join(body)}</a:p>"


def shape_id(n: int) -> int:
    return n + 2


class Slide:
    def __init__(self, title: str | None = None, section: str | None = None, hidden: bool = False):
        self.shapes: list[Shape] = []
        self.next_id = 1
        self.bg = COLORS["paper"]
        self.hidden = hidden
        if section:
            self.section_label(section)
        if title:
            self.header(title)

    def add(self, xml: str) -> None:
        self.shapes.append(Shape(xml))
        self.next_id += 1

    def rect(
        self,
        x: float,
        y: float,
        w: float,
        h: float,
        fill: str = "pale",
        line: str | None = None,
        radius: bool = False,
    ) -> None:
        sid = shape_id(self.next_id)
        geom = "roundRect" if radius else "rect"
        line_xml = '<a:ln><a:noFill/></a:ln>' if line is None else f'<a:ln w="12700"><a:solidFill><a:srgbClr val="{COLORS[line]}"/></a:solidFill></a:ln>'
        self.add(
            f'<p:sp><p:nvSpPr><p:cNvPr id="{sid}" name="Shape {sid}"/>'
            f'<p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr>'
            f'<a:xfrm><a:off x="{emu(x)}" y="{emu(y)}"/><a:ext cx="{emu(w)}" cy="{emu(h)}"/></a:xfrm>'
            f'<a:prstGeom prst="{geom}"><a:avLst/></a:prstGeom>'
            f'<a:solidFill><a:srgbClr val="{COLORS[fill]}"/></a:solidFill>{line_xml}'
            f"</p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p/></p:txBody></p:sp>"
        )

    def line(self, x1: float, y1: float, x2: float, y2: float, color: str = "line", width: int = 2) -> None:
        sid = shape_id(self.next_id)
        x = min(x1, x2)
        y = min(y1, y2)
        w = abs(x2 - x1)
        h = abs(y2 - y1)
        flip_h = ' flipH="1"' if x2 < x1 else ""
        flip_v = ' flipV="1"' if y2 < y1 else ""
        self.add(
            f'<p:cxnSp><p:nvCxnSpPr><p:cNvPr id="{sid}" name="Line {sid}"/>'
            f'<p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr><p:spPr>'
            f'<a:xfrm{flip_h}{flip_v}><a:off x="{emu(x)}" y="{emu(y)}"/><a:ext cx="{emu(w)}" cy="{emu(h)}"/></a:xfrm>'
            f'<a:prstGeom prst="line"><a:avLst/></a:prstGeom>'
            f'<a:ln w="{width * 12700}"><a:solidFill><a:srgbClr val="{COLORS[color]}"/></a:solidFill></a:ln>'
            f"</p:spPr></p:cxnSp>"
        )

    def text(
        self,
        x: float,
        y: float,
        w: float,
        h: float,
        paragraphs: list[str],
        size: float = 20,
        color: str = "ink",
        bold: bool = False,
        italic: bool = False,
        align: str = "l",
        font: str = "Arial",
        line_spacing: int | None = None,
        margin: float = 0.08,
    ) -> None:
        sid = shape_id(self.next_id)
        effective_size = max(size, MIN_FONT_SIZE)
        line_factor = (line_spacing or 100000) / 100000
        min_h = len(paragraphs) * effective_size * 0.0175 * line_factor + 2 * margin
        h = max(h, min_h)
        p_xml = "".join(para(p, size, color, bold, italic, align, font, line_spacing) for p in paragraphs)
        self.add(
            f'<p:sp><p:nvSpPr><p:cNvPr id="{sid}" name="Text {sid}"/>'
            f'<p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr><p:spPr>'
            f'<a:xfrm><a:off x="{emu(x)}" y="{emu(y)}"/><a:ext cx="{emu(w)}" cy="{emu(h)}"/></a:xfrm>'
            f'<a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/><a:ln><a:noFill/></a:ln>'
            f'</p:spPr><p:txBody><a:bodyPr wrap="square" lIns="{emu(margin)}" rIns="{emu(margin)}" tIns="{emu(margin)}" bIns="{emu(margin)}"/>'
            f"<a:lstStyle/>{p_xml}</p:txBody></p:sp>"
        )

    def rich_text(self, x: float, y: float, w: float, h: float, paragraphs: list[list[tuple[str, dict]]], size: float = 20) -> None:
        sid = shape_id(self.next_id)
        h = max(h, len(paragraphs) * max(size, MIN_FONT_SIZE) * 0.0175 + 0.16)
        p_xml = "".join(rich_para(p, size=size) for p in paragraphs)
        self.add(
            f'<p:sp><p:nvSpPr><p:cNvPr id="{sid}" name="Text {sid}"/>'
            f'<p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr><p:spPr>'
            f'<a:xfrm><a:off x="{emu(x)}" y="{emu(y)}"/><a:ext cx="{emu(w)}" cy="{emu(h)}"/></a:xfrm>'
            f'<a:prstGeom prst="rect"><a:avLst/></a:prstGeom><a:noFill/><a:ln><a:noFill/></a:ln>'
            f'</p:spPr><p:txBody><a:bodyPr wrap="square" lIns="{emu(0.08)}" rIns="{emu(0.08)}" tIns="{emu(0.08)}" bIns="{emu(0.08)}"/>'
            f"<a:lstStyle/>{p_xml}</p:txBody></p:sp>"
        )

    def header(self, title: str) -> None:
        self.rect(0, 0, 0.18, SLIDE_H, "teal")
        self.text(0.58, 0.34, 9.6, 0.58, [title], 26, "ink", True)
        self.line(0.62, 1.06, 12.55, 1.06, "line", 1)

    def section_label(self, label: str) -> None:
        self.rect(12.35, 0.34, 0.22, 0.22, "teal_dark", None, True)

    def tag(self, x: float, y: float, text: str, color: str = "teal", width: float | None = None) -> float:
        width = width or max(2.1, min(3.05, 0.16 * len(text) + 0.48))
        self.rect(x, y, width, 0.52, color, None, True)
        self.text(x + 0.06, y + 0.11, width - 0.12, 0.26, [text.upper()], 18, "white", True, align="c", margin=0)
        return width

    def card(self, x: float, y: float, w: float, h: float, label: str, body: list[str], accent: str = "teal", manual: str | None = None) -> None:
        self.rect(x, y, w, h, "white", "line", True)
        self.rect(x, y, 0.12, h, accent)
        label_size = 18
        body_size = 18
        self.text(x + 0.25, y + 0.18, w - 0.45, 0.36, [label], label_size, "ink", True)
        if manual == "manual-only" and w >= 4.8:
            tag_w = max(2.25, min(2.7, 0.16 * len(manual) + 0.48))
            self.tag(x + w - tag_w - 0.25, y + 0.16, manual, "amber", tag_w)
        elif manual == "manual-only":
            body = ["Manual-only trigger."] + body
        self.text(x + 0.25, y + 0.76, w - 0.45, max(0.5, h - 0.88), body, body_size, "muted", line_spacing=105000)

    def diagram_node(self, x: float, y: float, w: float, label: str, color: str) -> None:
        self.rect(x, y, w, 0.92, color, None, True)
        self.text(x + 0.08, y + 0.27, w - 0.16, 0.28, [label], 18, "white", True, align="c", margin=0)

    def xml(self) -> str:
        show_attr = ' show="0"' if self.hidden else ""
        return (
            '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
            '<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" '
            'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" '
            f'xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"{show_attr}>'
            '<p:cSld><p:bg><p:bgPr>'
            f'<a:solidFill><a:srgbClr val="{self.bg}"/></a:solidFill>'
            '<a:effectLst/></p:bgPr></p:bg><p:spTree>'
            '<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>'
            '<p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr>'
            + "".join(s.xml for s in self.shapes)
            + '</p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>'
        )


def title_slide() -> Slide:
    s = Slide()
    s.rect(0, 0, SLIDE_W, SLIDE_H, "paper")
    s.rect(0, 0, 0.52, SLIDE_H, "teal")
    s.rect(0.78, 0.78, 2.1, 0.14, "coral")
    s.text(0.7, 1.2, 10.9, 1.1, ["lab agent config update"], 44, "ink", True)
    s.text(0.75, 2.38, 8.7, 0.7, ["AI-assisted research workflows, now organized for both Claude Code and Codex."], 20, "muted")
    s.card(0.78, 4.48, 3.35, 1.35, "Principles", ["Try things. Systematize. Understand."], "teal")
    s.card(4.38, 4.48, 3.35, 1.35, "Config", ["Shared lab infrastructure."], "blue")
    s.card(7.98, 4.48, 3.35, 1.35, "Skills", ["Reusable workflows."], "amber")
    s.text(0.8, 6.85, 4.6, 0.25, ["Group meeting"], 9.5, "muted")
    return s


def polling_slide() -> Slide:
    s = Slide("Quick Poll: Using / Liking", "community")
    s.text(0.88, 1.28, 8.2, 0.42, ["n = 4; counted as people reporting they used or liked each workflow."], 18, "muted")

    x, y = 0.9, 2.0
    w_workflow, w_count, w_bar, row_h = 3.8, 1.55, 5.75, 0.86
    table_w = w_workflow + w_count + w_bar
    s.rect(x, y, table_w, 0.62, "teal_dark", None, False)
    s.text(x + 0.18, y + 0.15, w_workflow - 0.25, 0.26, ["Workflow"], 18, "white", True, margin=0)
    s.text(x + w_workflow + 0.15, y + 0.15, w_count - 0.25, 0.26, ["Count"], 18, "white", True, align="c", margin=0)
    s.text(x + w_workflow + w_count + 0.2, y + 0.15, w_bar - 0.3, 0.26, ["Share"], 18, "white", True, margin=0)

    rows = [
        ("Notebook", "3/4", 0.75, "teal", ""),
        ("O2 bridge", "2/4", 0.50, "blue", ""),
        ("Repo-specific skills", "1/4", 0.25, "amber", "Kangcheng: finalize-manuscript"),
    ]
    for idx, (name, count, share, color, note) in enumerate(rows):
        yy = y + 0.62 + idx * row_h
        fill = "white" if idx % 2 == 0 else "pale"
        s.rect(x, yy, table_w, row_h, fill, "line", False)
        s.text(x + 0.18, yy + 0.2, w_workflow - 0.28, 0.28, [name], 18, "ink", True, margin=0)
        s.text(x + w_workflow + 0.15, yy + 0.2, w_count - 0.25, 0.28, [count], 18, "ink", True, align="c", margin=0)
        bar_x = x + w_workflow + w_count + 0.35
        bar_y = yy + 0.27
        bar_w = w_bar - 1.35
        s.rect(bar_x, bar_y, bar_w, 0.24, "line", None, True)
        s.rect(bar_x, bar_y, bar_w * share, 0.24, color, None, True)
        s.text(bar_x + bar_w + 0.18, yy + 0.2, 0.75, 0.28, [f"{int(share * 100)}%"], 18, "ink", True, align="r", margin=0)
        if note:
            s.text(x + 0.18, yy + 0.52, w_workflow + w_count + w_bar - 0.4, 0.22, [note], 18, "muted", italic=True, margin=0)

    s.card(0.95, 5.35, 5.2, 1.35, "Interpretation", ["Notebook is the clearest early hit."], "teal")
    s.card(6.85, 5.35, 5.2, 1.35, "Next question", ["Which repo-specific skills should be easier to discover?"], "amber")
    return s


def section_slide(title: str, kicker: str, items: list[str], color: str = "teal") -> Slide:
    s = Slide()
    s.rect(0, 0, SLIDE_W, SLIDE_H, color)
    s.text(0.85, 0.9, 10.8, 0.7, [kicker.upper()], 11, "white", True)
    s.text(0.78, 1.72, 11.25, 1.3, [title], 39, "white", True)
    y = 3.6
    for item in items:
        s.rect(0.92, y + 0.11, 0.12, 0.12, "white")
        s.text(1.18, y, 10.2, 0.32, [item], 18, "white")
        y += 0.55
    return s


def make_slides() -> list[Slide]:
    slides: list[Slide] = [title_slide(), polling_slide()]

    s = Slide("Start With Advice", "advice", hidden=True)
    s.text(0.78, 1.46, 10.6, 0.75, ["The most important content is not the tool list. It is the habit loop."], 27, "ink", True)
    s.diagram_node(1.0, 3.0, 2.1, "try things", "teal")
    s.line(3.18, 3.38, 4.2, 3.38, "line", 3)
    s.diagram_node(4.35, 3.0, 2.3, "notice what works", "blue")
    s.line(6.72, 3.38, 7.7, 3.38, "line", 3)
    s.diagram_node(7.85, 3.0, 2.2, "systematize", "amber")
    s.text(1.0, 4.35, 9.7, 0.75, ["The repo is where useful habits become reusable agent behavior."], 19, "muted")
    slides.append(s)

    s = Slide("Advice: Figure Out What Works", "advice", hidden=True)
    s.card(0.78, 1.45, 3.65, 3.7, "Experiment deliberately", ["Try different tasks, prompts, apps, and interfaces.", "Treat this as active learning, not tool adoption."], "teal")
    s.card(4.82, 1.45, 3.65, 3.7, "Keep what helps", ["Ask for questions before planning.", "Use long rough prompts.", "Create project-specific workflows."], "blue")
    s.card(8.86, 1.45, 3.65, 3.7, "Do not overgeneralize", ["Our work is not cookie-cutter.", "Models and interfaces keep changing."], "amber")
    slides.append(s)

    s = Slide("Advice: Then Systematize It", "advice", hidden=True)
    s.text(0.85, 1.45, 5.2, 0.62, ["A system can be tiny."], 28, "ink", True)
    s.text(0.95, 2.25, 5.0, 2.7, ["A habit", "A project instruction", "A skill", "A postmortem change", "A checklist the agent follows"], 19, "muted", line_spacing=105000)
    s.rect(6.35, 1.55, 5.55, 3.95, "white", "line", True)
    s.text(6.72, 1.95, 4.6, 0.4, ["Encoded examples"], 18, "ink", True)
    s.text(6.72, 2.55, 4.8, 2.3, ["Make a record of substantive work.", "Ask questions in Plan Mode.", "Review substantial software changes.", "Number lists so humans can reply cleanly."], 17, "muted", line_spacing=105000)
    slides.append(s)

    s = Slide("Advice: Question Your Code", "advice", hidden=True)
    s.rect(0.92, 1.42, 11.35, 1.25, "ink", None, True)
    s.text(1.25, 1.83, 10.7, 0.34, ['"You can outsource thinking, but not understanding."'], 23, "white", True, italic=True, align="c")
    s.text(0.95, 3.28, 5.2, 0.42, ["Ask the agent, or ask yourself:"], 20, "ink", True)
    s.text(1.05, 3.95, 5.2, 1.7, ["What changed, and why?", "What hidden choices were made?", "What could go wrong?", "What test would reassure us?"], 18, "muted", line_spacing=105000)
    s.card(7.0, 3.25, 4.55, 2.1, "Scientific caution", ["AI is useful for analyses, but it lacks scientific judgement and integrity.", "You still need to understand every scientifically relevant detail."], "coral")
    slides.append(s)

    s = Slide("Advice: Align Three Things", "advice", hidden=True)
    cx, cy, r = 6.65, 3.55, 1.7
    points = [(cx, cy - r), (cx - 1.75, cy + 1.25), (cx + 1.75, cy + 1.25)]
    labels = [("intent", "teal"), ("understanding", "blue"), ("implementation", "amber")]
    for i in range(3):
        x1, y1 = points[i]
        x2, y2 = points[(i + 1) % 3]
        s.line(x1, y1, x2, y2, "line", 4)
    for (x, y), (label, color) in zip(points, labels):
        s.diagram_node(x - 1.0, y - 0.35, 2.0, label, color)
    s.text(1.0, 5.95, 11.1, 0.42, ["This is the deck's main standard for agent-assisted work."], 19, "ink", True, align="c")
    slides.append(s)

    s = Slide("Advice: Structure Your Understanding", "advice", hidden=True)
    s.card(0.78, 1.45, 3.75, 3.7, "Core modules", ["Understand the inner logic.", "These are the places scientific choices live."], "teal")
    s.card(4.78, 1.45, 3.75, 3.7, "Other modules", ["Understand the interface.", "Treat internals as a black box unless needed."], "blue")
    s.card(8.78, 1.45, 3.75, 3.7, "Whole codebase", ["Understand relationships.", "Know which changes should touch which modules."], "amber")
    s.text(1.0, 6.08, 11.3, 0.38, ["Heuristic: understand scientific logic at the level you would report in Methods or Supplement."], 17, "ink", True, align="c")
    slides.append(s)

    s = Slide("Advice: Watch For Being Too Zoomed Out", "advice", hidden=True)
    s.card(0.85, 1.45, 5.4, 2.2, "Warning signs", ["You cannot predict which modules a change will touch.", "The same area keeps producing bugs.", "A simple task took long for unclear reasons."], "coral")
    s.card(7.0, 1.45, 4.55, 2.2, "Example", ["The agent silently performs liftover while merging datasets.", "That might be fine, but you need to know."], "amber")
    s.text(1.05, 4.55, 10.8, 0.78, ["The point of the config is not to slow the agent down. It is to keep the human close enough to understand what happened."], 23, "ink", True, align="c")
    slides.append(s)

    slides.append(section_slide("What's New", "repo update", ["From a Claude-only config to a lab agent config.", "Two product-specific trees.", "Shared tooling and shared workflow concepts."], "teal_dark"))

    s = Slide("Two Agent Trees", "what's new")
    s.card(0.92, 1.55, 5.1, 3.85, "Claude tree", ["Claude Code prompts, settings, hooks, and skills.", "Uses CLAUDE.md, slash commands, and .claude/skills."], "blue")
    s.card(7.25, 1.55, 5.1, 3.85, "Codex tree", ["Codex instructions, templates, and skills.", "Uses AGENTS.md, $skill naming, and .agents/skills."], "teal")
    s.text(1.05, 6.1, 11.3, 0.32, ["Goal: preserve shared lab behavior while making product differences explicit."], 17, "ink", True, align="c")
    slides.append(s)

    s = Slide("Shared Skill Taxonomy", "what's new")
    s.text(0.9, 1.43, 5.4, 0.5, ["Same families, different surfaces."], 24, "ink", True)
    s.text(0.98, 2.12, 5.35, 2.2, ["The new layout keeps Claude and Codex aligned unless there is a product-specific reason to diverge.", "Subskills keep specialized instructions out of the top-level skill list."], 18, "muted", line_spacing=105000)
    s.rect(7.05, 1.48, 4.3, 3.4, "white", "line", True)
    s.text(7.4, 1.88, 3.6, 2.2, ["work-cycle", "artifacts", "documentation", "systematize", "notebook and resume", "O2 / DNAnexus"], 18, "teal_dark", True, line_spacing=105000)
    slides.append(s)

    s = Slide("Setup Model", "what's new")
    s.card(0.9, 1.55, 5.2, 3.55, "New CLI tool", ["A unified setup and skill-link manager.", "Primarily intended for the model to use while helping users configure the environment."], "teal")
    s.card(7.05, 1.55, 5.2, 3.55, "User instruction", ["Because the repo shape changed substantially, re-setup from scratch.", "Do not try to patch an old install by hand."], "coral")
    slides.append(s)

    slides.append(section_slide("Codex vs Claude", "tool choice", ["Most people should choose one primary interface.", "Codex may shine when the spec is clear.", "Claude may shine when the problem is still taking shape."], "blue"))

    s = Slide("Codex vs Claude: Field Report", "tool choice")
    s.rect(0.86, 1.38, 11.6, 4.8, "white", "line", True)
    s.text(1.16, 1.68, 10.95, 0.36, ["Kangcheng:"], 15, "teal_dark", True)
    s.text(1.16, 2.13, 10.85, 3.28, [
        "I have mostly used Claude, but I've used Codex more this past week and now have a clearer sense of where each works better:",
        "Codex: slightly better when I know exactly what I want, have a clear spec / prompt, and have explicit validation criteria.",
        "Claude: better when problem / spec still unclear. I still prefer using that to go back and forth explore the data and formalize."
    ], 16.2, "ink", line_spacing=105000)
    s.text(1.16, 5.52, 10.8, 0.28, ["Paraphrased takeaway: Codex for crisp execution; Claude for exploratory formalization."], 13, "muted", italic=True)
    slides.append(s)

    s = Slide("Subscription Policy", "tool choice")
    s.card(0.85, 1.45, 3.7, 3.85, "Default advice", ["Pick one primary tool.", "Learn the model quirks and interface deeply."], "teal")
    s.card(4.82, 1.45, 3.7, 3.85, "Luke's current pick", ["Try ChatGPT Pro / Codex.", "ChatGPT Pro is a real current advantage."], "blue")
    s.card(8.78, 1.45, 3.7, 3.85, "Using both", ["Fine if you actually use both.", "The extra subscription and learning overhead should be intentional."], "amber")
    slides.append(s)

    s = Slide("How Luke Uses GPT-Pro", "tool choice")
    s.card(0.85, 1.45, 3.6, 3.55, "Long prompts", ["Voice-to-text is welcome.", "Prompts can be rough if they contain the real context."], "teal")
    s.card(4.82, 1.45, 3.6, 3.55, "Planning", ["Ask detailed questions.", "Use Plan Mode before substantial work."], "blue")
    s.card(8.78, 1.45, 3.6, 3.55, "Execution", ["Clear specs plus validation criteria.", "Let the model run end-to-end when the target is crisp."], "amber")
    s.text(1.05, 6.0, 11.1, 0.3, ["Placeholder for concrete examples to add before presenting."], 14.5, "coral", True, align="c")
    slides.append(s)

    slides.append(section_slide("Work-Cycle Skill", "core workflow", ["Manual trigger only.", "Applies to software, analyses, and artifacts.", "Its job is alignment before execution."], "teal_dark"))

    s = Slide("Work-Cycle: What It Is", "core workflow")
    s.card(0.8, 1.42, 5.25, 3.8, "Scope", ["For substantial work where mistakes are expensive or ambiguous.", "Not just code: also analyses and public artifacts."], "teal", "manual-only")
    s.card(7.0, 1.42, 5.25, 3.8, "Core promise", ["Ground in the environment.", "Ask detailed questions.", "Draft a concrete plan.", "Implement and verify against the plan."], "blue")
    slides.append(s)

    s = Slide("Work-Cycle: Planning First", "core workflow")
    s.diagram_node(0.95, 2.05, 2.15, "explore", "teal")
    s.line(3.15, 2.43, 4.0, 2.43, "line", 3)
    s.diagram_node(4.15, 2.05, 2.15, "ask", "blue")
    s.line(6.35, 2.43, 7.2, 2.43, "line", 3)
    s.diagram_node(7.35, 2.05, 2.15, "align", "amber")
    s.line(9.55, 2.43, 10.4, 2.43, "line", 3)
    s.diagram_node(10.55, 2.05, 2.15, "plan", "coral")
    s.text(1.0, 3.85, 11.1, 0.9, ["The agent should discover what it can from the repo before asking. Questions should change the plan, not create busywork."], 22, "ink", True, align="c")
    slides.append(s)

    s = Slide("Work-Cycle: Implementation Loop", "core workflow")
    s.card(0.85, 1.45, 3.65, 3.55, "1. Build", ["Implement from the aligned plan.", "Use repo conventions and keep scope tight."], "teal")
    s.card(4.85, 1.45, 3.65, 3.55, "2. Verify", ["Run tests or artifact checks.", "Debug iteratively rather than explaining away failures."], "blue")
    s.card(8.85, 1.45, 3.65, 3.55, "3. Report", ["Say what changed.", "Name deviations from the plan.", "Use external review for substantial work."], "amber")
    slides.append(s)

    s = Slide("Artifacts Skill", "skills")
    s.card(0.82, 1.35, 4.25, 4.3, "Purpose", ["Create, edit, polish, or inspect public-facing artifacts.", "DOCX, PDF, PPTX, TikZ, figures, manuscript finalization."], "teal")
    s.card(5.52, 1.35, 5.95, 4.3, "Key advice", ["Artifacts should be non-path-dependent.", "Do not refer to prompts, previous versions, or production history that readers cannot see.", "Prioritize message, evidence, and purpose over decorative process notes."], "coral")
    s.tag(10.18, 0.34, "not manual-only", "blue")
    slides.append(s)

    s = Slide("Documentation Skill", "skills")
    s.card(0.82, 1.35, 4.8, 4.1, "Purpose", ["Promote alignment and understanding.", "Make module boundaries, surfaces, and relationships explicit."], "teal", "manual-only")
    s.card(6.2, 1.35, 5.2, 4.1, "Subskills", ["map-modules: high-level decomposition.", "document-module: focused explanation.", "maintain-project: notebook and project docs health."], "blue")
    slides.append(s)

    s = Slide("Systematize Skill", "skills")
    s.card(0.82, 1.35, 5.0, 4.2, "Purpose", ["Find out what works for you, then keep doing it.", "Turn lessons into reusable instructions, skills, and support workflows."], "amber")
    s.card(6.35, 1.35, 5.0, 4.2, "Subskills", ["skill-creator", "postmortem", "agents-md / CLAUDE.md guidance", "support"], "teal")
    s.tag(10.18, 0.34, "not manual-only", "blue")
    slides.append(s)

    s = Slide("Notebook And Continuity Skills", "skills")
    s.card(0.8, 1.35, 3.75, 3.95, "notebook-entry", ["Durable records of substantive work.", "Often invoked by a subagent."], "teal", "not manual-only")
    s.card(4.82, 1.35, 3.75, 3.95, "defer", ["Park future work without derailing the current task."], "blue", "manual-only")
    s.card(8.84, 1.35, 3.75, 3.95, "remind-resume", ["Recover context after a break.", "Summarize recent work, git state, and next steps."], "amber", "manual-only")
    slides.append(s)

    s = Slide("Global Skills", "installation")
    s.text(0.9, 1.35, 10.8, 0.42, ["Global means useful across most projects, independent of one dataset or compute environment."], 19, "ink", True)
    global_skills = [
        ("artifacts", "not manual-only"),
        ("documentation", "manual-only"),
        ("init-project", "manual-only"),
        ("notebook-entry", "not manual-only"),
        ("remind-resume", "manual-only"),
        ("work-cycle", "manual-only"),
        ("systematize", "not manual-only"),
        ("defer", "manual-only"),
    ]
    x, y = 0.95, 2.15
    for i, (name, mode) in enumerate(global_skills):
        col = i % 2
        row = i // 2
        xx = x + col * 6.0
        yy = y + row * 0.92
        s.rect(xx, yy, 5.65, 0.68, "white", "line", True)
        s.text(xx + 0.2, yy + 0.18, 2.45, 0.24, [name], 18, "ink", True, margin=0)
        tag_w = 2.25 if mode == "manual-only" else 2.8
        s.tag(xx + 2.7, yy + 0.08, mode, "amber" if mode == "manual-only" else "blue", tag_w)
    slides.append(s)

    s = Slide("Local / Project Skills", "installation")
    s.text(0.9, 1.35, 10.8, 0.42, ["Local means tied to project infrastructure, data location, or domain-specific work."], 19, "ink", True)
    s.card(0.9, 2.15, 3.55, 2.4, "use-o2", ["Operate the remote bridge and SLURM after setup."], "teal", "not manual-only")
    s.card(4.9, 2.15, 3.55, 2.4, "dx-jobs", ["Inspect, monitor, and resubmit DNAnexus jobs."], "blue", "not manual-only")
    s.card(8.9, 2.15, 3.55, 2.4, "run-graphld-o2", ["Specific GraphLD graphREML workflow on O2."], "amber", "not manual-only")
    slides.append(s)

    s = Slide("What People Have Liked", "community")
    s.card(0.8, 1.28, 3.7, 4.2, "Reviewable work", ["GitHub issues and the gh CLI.", "Concrete tasks, links, and PRs."], "teal")
    s.card(4.82, 1.28, 3.7, 4.2, "Kangcheng", ["/grill-me skill", "/code-writeup skill", "Both make understanding more explicit."], "blue")
    s.card(8.84, 1.28, 3.7, 4.2, "Other examples", ["Amber: Marimo.", "Pouria: T3 code.", "The best ideas come from real use."], "amber")
    slides.append(s)

    s = Slide("Please Contribute Back", "community")
    s.card(0.95, 1.45, 5.15, 3.85, "GitHub issues", ["Use issues for suggestions, workflow pain, missing docs, or examples of things that worked."], "teal")
    s.card(7.0, 1.45, 5.15, 3.85, "Pull requests", ["Small PRs are welcome: skill edits, new skills, examples, docs, and setup improvements."], "blue")
    s.text(1.05, 6.02, 11.1, 0.34, ["Best contribution: something concrete that helped you do real work."], 18, "ink", True, align="c")
    slides.append(s)

    s = Slide("Closing", "close")
    s.text(0.95, 1.45, 11.2, 1.3, ["The point is not to let AI do research unsupervised."], 36, "ink", True, align="c")
    s.text(1.28, 3.15, 10.5, 1.0, ["The point is to make AI-assisted work more understandable, systematic, and shareable."], 29, "teal_dark", True, align="c")
    s.rect(3.2, 5.35, 6.9, 0.12, "coral")
    slides.append(s)

    return slides


def content_types(n_slides: int) -> str:
    overrides = [
        '<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>',
        '<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>',
        '<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>',
        '<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>',
        '<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>',
        '<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>',
    ]
    for i in range(1, n_slides + 1):
        overrides.append(f'<Override PartName="/ppt/slides/slide{i}.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>')
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
        '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
        '<Default Extension="xml" ContentType="application/xml"/>'
        + "".join(overrides)
        + "</Types>"
    )


def root_rels() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>'
        '<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>'
        '<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>'
        '</Relationships>'
    )


def presentation_xml(n_slides: int) -> str:
    slide_ids = "".join(f'<p:sldId id="{255 + i}" r:id="rId{i}"/>' for i in range(1, n_slides + 1))
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" '
        'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" '
        'xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">'
        f'<p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId{n_slides + 1}"/></p:sldMasterIdLst>'
        f'<p:sldIdLst>{slide_ids}</p:sldIdLst>'
        f'<p:sldSz cx="{SLIDE_W_EMU}" cy="{SLIDE_H_EMU}" type="wide"/>'
        '<p:notesSz cx="6858000" cy="9144000"/>'
        '<p:defaultTextStyle><a:defPPr><a:defRPr lang="en-US"/></a:defPPr></p:defaultTextStyle>'
        '</p:presentation>'
    )


def presentation_rels(n_slides: int) -> str:
    rels = []
    for i in range(1, n_slides + 1):
        rels.append(f'<Relationship Id="rId{i}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide{i}.xml"/>')
    rels.append(f'<Relationship Id="rId{n_slides + 1}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>')
    rels.append(f'<Relationship Id="rId{n_slides + 2}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>')
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        + "".join(rels)
        + '</Relationships>'
    )


def slide_rels() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>'
        '</Relationships>'
    )


def master_xml() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" '
        'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" '
        'xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">'
        '<p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>'
        '<p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr>'
        '</p:spTree></p:cSld><p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>'
        '<p:sldLayoutIdLst><p:sldLayoutId id="2147483649" r:id="rId1"/></p:sldLayoutIdLst>'
        '<p:txStyles><p:titleStyle/><p:bodyStyle/><p:otherStyle/></p:txStyles>'
        '</p:sldMaster>'
    )


def master_rels() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>'
        '<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>'
        '</Relationships>'
    )


def layout_xml() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" '
        'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" '
        'xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">'
        '<p:cSld name="Blank"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>'
        '<p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr>'
        '</p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>'
    )


def layout_rels() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
        '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>'
        '</Relationships>'
    )


def theme_xml() -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Lab Agent Config">'
        '<a:themeElements><a:clrScheme name="Lab">'
        '<a:dk1><a:srgbClr val="22252A"/></a:dk1><a:lt1><a:srgbClr val="F7F3EA"/></a:lt1>'
        '<a:dk2><a:srgbClr val="5D646B"/></a:dk2><a:lt2><a:srgbClr val="FFFFFF"/></a:lt2>'
        '<a:accent1><a:srgbClr val="1F7A7A"/></a:accent1><a:accent2><a:srgbClr val="506B8B"/></a:accent2>'
        '<a:accent3><a:srgbClr val="C69214"/></a:accent3><a:accent4><a:srgbClr val="D85A4A"/></a:accent4>'
        '<a:accent5><a:srgbClr val="E8EFEA"/></a:accent5><a:accent6><a:srgbClr val="135C5B"/></a:accent6>'
        '<a:hlink><a:srgbClr val="506B8B"/></a:hlink><a:folHlink><a:srgbClr val="1F7A7A"/></a:folHlink>'
        '</a:clrScheme><a:fontScheme name="Arial"><a:majorFont><a:latin typeface="Arial"/></a:majorFont><a:minorFont><a:latin typeface="Arial"/></a:minorFont></a:fontScheme><a:fmtScheme name="Clean"><a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst><a:lnStyleLst><a:ln w="9525"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln></a:lnStyleLst><a:effectStyleLst><a:effectStyle><a:effectLst/></a:effectStyle></a:effectStyleLst><a:bgFillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:bgFillStyleLst></a:fmtScheme></a:themeElements><a:objectDefaults/><a:extraClrSchemeLst/></a:theme>'
    )


def core_xml() -> str:
    now = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" '
        'xmlns:dc="http://purl.org/dc/elements/1.1/" '
        'xmlns:dcterms="http://purl.org/dc/terms/" '
        'xmlns:dcmitype="http://purl.org/dc/dcmitype/" '
        'xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">'
        '<dc:title>Lab Agent Config Update</dc:title><dc:creator>Codex</dc:creator>'
        '<cp:lastModifiedBy>Codex</cp:lastModifiedBy>'
        f'<dcterms:created xsi:type="dcterms:W3CDTF">{now}</dcterms:created>'
        f'<dcterms:modified xsi:type="dcterms:W3CDTF">{now}</dcterms:modified>'
        '</cp:coreProperties>'
    )


def app_xml(n_slides: int, hidden_slides: int = 0) -> str:
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" '
        'xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">'
        '<Application>Codex</Application><PresentationFormat>On-screen Show (16:9)</PresentationFormat>'
        f'<Slides>{n_slides}</Slides><Notes>0</Notes><HiddenSlides>{hidden_slides}</HiddenSlides>'
        '</Properties>'
    )


def build() -> None:
    slides = make_slides()
    hidden_slides = sum(1 for slide in slides if slide.hidden)
    with zipfile.ZipFile(OUT, "w", compression=zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml", content_types(len(slides)))
        z.writestr("_rels/.rels", root_rels())
        z.writestr("docProps/core.xml", core_xml())
        z.writestr("docProps/app.xml", app_xml(len(slides), hidden_slides))
        z.writestr("ppt/presentation.xml", presentation_xml(len(slides)))
        z.writestr("ppt/_rels/presentation.xml.rels", presentation_rels(len(slides)))
        z.writestr("ppt/slideMasters/slideMaster1.xml", master_xml())
        z.writestr("ppt/slideMasters/_rels/slideMaster1.xml.rels", master_rels())
        z.writestr("ppt/slideLayouts/slideLayout1.xml", layout_xml())
        z.writestr("ppt/slideLayouts/_rels/slideLayout1.xml.rels", layout_rels())
        z.writestr("ppt/theme/theme1.xml", theme_xml())
        for idx, slide in enumerate(slides, start=1):
            z.writestr(f"ppt/slides/slide{idx}.xml", slide.xml())
            z.writestr(f"ppt/slides/_rels/slide{idx}.xml.rels", slide_rels())
    print(f"Wrote {OUT} ({len(slides)} slides)")


if __name__ == "__main__":
    build()
