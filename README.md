# Lab Agent Config For Scientific Research

This repository configures Claude Code and Codex for scientific software development and other scientific research tasks. The goal is to integrate AI agents more deeply into our scientific work while mitigating foreseeable pitfalls of doing so. See [ADVICE.md](ADVICE.md) for Luke's big-picture advice about AI collaboration.

The project has three major features:
1. **O2 Cluster bridge.** It includes guidance for using an external bridge application that enables an agent to interact with O2, submit jobs, and read results. Its interactions are sandboxed to avoid accidental data deletion.
2. **Project notebook.** It configures agents to keep a project notebook that tracks the current state of the project and logs substantive work. This notebook is intended both to provide context to the agent and to mitigate non-reproducibility, which is a potential pitfall when you rely on agents without long-term memory.
3. **Skills.** It provides several specialized prompts for recurring workflows or tasks. Some are meant to be invoked explicitly, while others remain eligible for automatic use. Together they aim to improve the AI's performance at tasks like project setup, coding, documentation, and scientific analysis.


## Quick Start

### 0. Install Claude Code Or Codex

Use whichever agent you prefer. This repository supports both Claude Code and Codex; you can install configuration for one or both.

### 1. Clone This Repository

```bash
git clone https://github.com/oclb/claude.git lab-agent-config
cd lab-agent-config
```

### 2. Install Agent Configuration

For Claude Code:

```bash
bin/config-agent-tool install --agent claude
```

For Codex:

```bash
bin/config-agent-tool install --agent codex
```

### 3. Choose Global Workflows

These commands make the main project workflows available globally. You can choose fewer if you want a lighter setup.

For Claude Code:

```bash
bin/config-agent-tool link-skills --agent claude --global --add init-project work-cycle documentation defer notebook-entry artifacts
```

For Codex:

```bash
bin/config-agent-tool link-skills --agent codex --global --add init-project work-cycle documentation defer notebook-entry artifacts
```

Codex also has a guided setup workflow. Start Codex in this repository and say:

```text
$set-me-up
```

### 4. Initialize A Project

Open your own project and start your agent there.

For Claude Code, say:

```text
/init-project
```

For Codex, say:

```text
$init-project
```

The agent should inspect the project, create or update project-local instructions, set up a `notebook/`, offer to install project-scope workflows, offer to configure GitHub remotes, and offer to set up the O2 bridge.


## Features

### Project Notebook

A major limitation of existing AI models is their lack of long-term memory. One problem this creates is that you often need to guess what context the AI will need for a task and include it manually. Even when the AI can gather context itself - e.g., by reading your project's source code - you may not trust that it will do so reliably. Another problem is irreproducibility: when the AI performs analyses, even with your close guidance, it can be easy to forget exactly what was done and why. If the AI also forgets those details, then they are simply lost.

To address these problems, the agent is instructed to maintain a notebook which acts as a stable repository for long-term memory. All substantive work done with the agent is recorded in this notebook. The notebook is indexed, and the agent is instructed to search for relevant entries when it determines that it needs context. The notebook lives in a GitHub repository that should be separate from your project repository and is managed entirely by the agent. Entries of this notebook include text files which describe what was done or learned, code which can be used to reproduce results, and results files.

Many other systems for AI memory exist, including built-in user-level memory systems. This one is designed for simplicity and completeness. If at any point in the future you wish to transition notebook-based memories into some other system, an agent should be able to do this for you.

The notebook also includes a to-do list, whose usage is optional. I often use it to defer to-do items that come up organically but that I don't want to do just yet.

### Remote O2 Access

Remote O2 access uses the external `remote-bridge` CLI to let an agent interact with O2 through an SSH-backed session. After installing this repository, use `/init-project` or `$init-project` for first-time bridge setup, then use `/use-o2` or `$use-o2` for job submission, monitoring, SLURM guidance, and bridge operations.

The bridge maintains a connection over `ssh` with an O2 login node. It exposes controlled file-inspection, git, and job-submission operations against paths allowed by the local bridge configuration. Containerized job submission can add some dependency-management friction, but it limits writes to the directories configured for the sandbox.

### Workflows

Claude Code and Codex can use specialized prompts for recurring workflows. Installing one makes it available to the agent; manual workflows still need to be invoked explicitly with `/name` in Claude Code or `$name` in Codex.

The most useful global workflows are:

1. `init-project`: initialize a project workspace with project instructions, notebook scaffolding, optional remotes, and optional O2 setup.
2. `work-cycle`: use a planning-centric workflow for substantial software, analysis, artifact, documentation, and configuration work.
3. `documentation`: create and update documentation so the human and agent maintain a shared understanding of a codebase.
4. `defer`: capture deferred work as notebook-backed TODOs.
5. `notebook-entry`: create durable project notebook entries.
6. `artifacts`: work on DOCX, PDF, PPTX, TikZ diagrams, manuscript figures, and manuscript submission checks.

Project-specific workflows include `use-o2`, `dx-jobs`, and `run-graphld-o2`.


## Maintenance

Use `config-agent-tool` to update installed files after pulling repository changes:

```bash
config-agent-tool update --agent claude
config-agent-tool update --agent codex
```

You can also list, add, or remove workflows later:

```bash
config-agent-tool list-skills --agent claude --global
config-agent-tool link-skills --agent claude --global --add artifacts
config-agent-tool link-skills --agent claude --global --remove artifacts

config-agent-tool list-skills --agent codex --global
config-agent-tool link-skills --agent codex --global --add artifacts
config-agent-tool link-skills --agent codex --global --remove artifacts
```

Claude Code stores its global configuration under `~/.claude`. Codex stores its global configuration under `~/.codex`. The setup tool manages the generated files and links non-destructively.


## Contributing

Please contribute back to this project to help improve it for others: ask your agent to create a GitHub Issue, make a pull request with a fix or improvement, or create a new workflow and contribute it to the repo using a pull request.
