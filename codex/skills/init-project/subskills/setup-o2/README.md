# Setup O2

Codex playbook for first-time Harvard O2 access setup through the `remote-bridge` CLI.

This is an internal subskill of `init-project`, used when a project setup or onboarding task includes O2/SLURM access.

## What it covers
- Bridge installation and startup checks
- Path-permission configuration
- SSH key setup for O2
- First connection validation

## Notes
- This directory contains prompts only.
- `remote-bridge` code/binaries are intentionally not bundled with this configuration.
- For bridge commands, SLURM submission, monitoring, containers, and resource heuristics, use the top-level `use-o2` skill.
