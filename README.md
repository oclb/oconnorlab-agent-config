# O'Connor lab coding agent config

This repository configures Claude Code or Codex CLI for scientific software development and other scientific research tasks. The goal is to integrate AI agents more deeply into our scientific work while mitigating foreseeable pitfalls of doing so. See [ADVICE.md](ADVICE.md) for Luke's big-picture advice about AI collaboration.

The project has three major features:
1. **O2 Cluster bridge.** It includes guidance for using an external bridge application that enables an agent to interact with O2, submit jobs, and read results. Its interactions are sandboxed to avoid accidental data deletion.
2. **Project notebook.** It configures agents to keep a project notebook that tracks the current state of the project and logs substantive work. This notebook is intended both to provide context to the agent and to mitigate non-reproducibility, which is a potential pitfall when you rely on agents without long-term memory.
3. **Skills.** It provides several specialized prompts for recurring workflows or tasks. Some are meant to be invoked explicitly, while others remain eligible for automatic use. Together they aim to improve the AI's performance at tasks like project setup, coding, documentation, and scientific analysis.


## Quick Start

### 0. Install Claude Code Or Codex CLI
You can install configuration for either or both. It shouldn't matter what interface you use (i.e., terminal interface or an app).

### 1. Clone This Repository

```bash
git clone https://github.com/oclb/oconnorlab-agent-config.git lab-agent-config
cd lab-agent-config
```

### 2. Ask your agent to perform setup
Run your agent of choice inside of the repo directory, ask it to perform setup, and answer its questions.

For Claude Code or Codex, the repo-local setup skill should be discoverable before installation. You can say:

```text
set me up
```

### 3. Initialize A Project

Open your own project and start your agent there.

For Claude Code:

```text
/init-project
```

For Codex:

```text
$init-project
```

The agent should inspect the project, create or update project-local instructions, set up a `notebook/`, offer to install project-scope workflows, offer to configure GitHub remotes, and offer to set up the O2 bridge.


## Features

### Project Notebook

A major limitation of AI models is their lack of long-term memory. This creates friction when communicating with the AI, and it can push you toward long-running conversations, which is known to be bad practice. Another problem is irreproducibility: when the AI performs analyses, even with your close guidance, it can be easy to forget exactly what was done and why. If the AI also forgets those details, then they are lost.

To address these problems, the agent is instructed to maintain a notebook which acts as a stable repository for long-term memory. All substantive work done with the agent is recorded in this notebook. The notebook is indexed, and the agent is instructed to search for relevant entries when it determines that it needs context. The notebook lives in a GitHub repository separate from your project repository. It includes entries which describe what was done or learned, scripts to reproduce results, results files, a to-do list, and an index.

Many other systems for AI memory exist, including built-in user-level memory systems. This one is designed for simplicity and completeness. It lacks advanced features like semantic search. If at any point in the future you wish to transition notebook-based memories into some other system, an agent should be able to do this for you.

The notebook's to-do list is optional. I often use it as a convenient way to track to-do items that come up organically but that I don't want to do just yet (see below for the `defer` skill).

### O2 bridge

The `remote-bridge` CLI allows an agent interact with the O2 cluster through an SSH-backed session within a sandbox. The bridge maintains a connection over `ssh` with an O2 login node. It allows you to control the directories on O2 to which your agent has read access and write access. This even applies to jobs that your agent submits: all submitted jobs dispatch to Singularity containers with write access only within specified directories. 

Please do not give your agent write access to directories containing data owned by other lab members. In the event of accidental data deletion, O2 does maintain regular snapshots of the group directory, but these are deleted after a short period of time, so you should notify Luke immediately.

### Workflow skills

Specialized prompts are provided for recurring workflows:

1. `work-cycle`: prescribes an interactive, planning-centric workflow. This is recommended for all substantive research tasks, including software development and running analyses. It instructs the agent to ask questions and work with the user to create a plan. It has four special "modes", which are non-exclusive: worktree mode, which instructs the agent to implement its changes in a Git worktree; afk mode, which instructs it to ask any questions immediately and then implement autonomously; methods-first mode, which instructs it to use a Methods section or other document as a plan or specification; and grill-me mode, which instructs it to stress-test the plan through structured questioning.
2. `artifacts`: gives a few guidelines for writing, provides subskills for various file formats (DOCX, PDF, PPTX, TikZ/LaTeX diagrams), provides a subskill for manuscript-quality figures, and provides a subskill for manuscript submission checks.
3. `documentation`: create and update documentation so the human and agent maintain a shared understanding of a codebase.
4. `systematize`: customize the behavior of the agent. Use this for running a postmortem when agent does not behave as desired, for updating the agent's system prompt, and for creating custom skills. 

### Other skills

1. `defer`: capture deferred work as notebook-backed TODOs.
2. `notebook-entry`: create durable project notebook entries.
3. `use-o2`: operate the O2 bridge after first-time setup.
4. `dx-jobs`: check, monitor, diagnose, and resubmit DNAnexus jobs.
5. `run-graphld-o2`: install and run [GraphLD](https://github.com/oclb/graphld) graphREML on O2.
6. `init-project`: initialize your agent config within a project, set up the notebook if you opted into it, and optionally install project-scope skills such as the O2 and DNAnexus helpers.

## See Also

Some workflows are adapted from or inspired by external sources:

1. `grillme` mode is inspired by Matt Pocock's [`grill-me`](https://www.skills.sh/mattpocock/skills/grill-me) skill.
2. `polished-manuscript-figure` draws on the [Nature research figure guide](https://research-figure-guide.nature.com/figures/).
3. `docx`, `pptx`, and `pdf` are adapted from Anthropic's official document skill patterns for [Word document creation, PowerPoint presentation generation, and PDF creation and processing](https://support.claude.com/en/articles/12512180-use-skills-in-claude).


## Contributing

Please contribute back to this project to help improve it for others: ask your agent to create a GitHub Issue, make a pull request with a fix or improvement, or create a new workflow and contribute it to the repo using a pull request.
