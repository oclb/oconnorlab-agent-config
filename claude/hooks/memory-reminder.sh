#!/bin/bash

# Only emit memory reminder if a notebook exists in the project.
# Walk up from CWD looking for notebook/.git (the notebook is a separate git repo).
DIR="$(pwd)"
FOUND=false
while [ "$DIR" != "/" ]; do
  if [ -d "$DIR/notebook/.git" ]; then
    FOUND=true
    break
  fi
  DIR="$(dirname "$DIR")"
done

if [ "$FOUND" = true ]; then
  echo '<reminder>DECIDE NOW: Will this turn produce SUBSTANTIAL work (multi-step analysis, significant implementation, non-obvious discovery, tool setup with gotchas)? If yes, spawn a memory agent before your turn ends. Most turns do NOT need entries - skip for discussions, minor fixes, routine operations, and planning without action.</reminder>'
fi
