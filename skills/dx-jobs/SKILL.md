---
name: dx-jobs
description: Check and manage DNAnexus jobs. Use when the user asks to "check jobs", "check status", "check dx jobs", "monitor jobs", "what failed", or references DNAnexus job status. Also use when resubmitting failed jobs or inspecting job logs.
---

# DNAnexus Job Management

## Prerequisites

Ensure `dx` is available in the shell. If not, activate the appropriate conda environment or check the user's PATH.

## Check Job Status

Get a status breakdown for a batch of jobs by name pattern:

```bash
for state in done failed running runnable; do
    count=$(dx find jobs --name "<NAME_PATTERN>" --state $state --num-results 10000 --brief 2>&1 | wc -l)
    echo "$state: $count"
done
```

**Important:** `dx find jobs` defaults to 1000 results. Always pass `--num-results 10000` for large batches.

## Inspect Failed Jobs

Get failure reasons for all failed jobs:

```bash
dx find jobs --name "<NAME_PATTERN>" --state failed --num-results 10000 --brief 2>&1 | while read -r jobid; do
    echo "=== $jobid ==="
    dx describe "$jobid" --json 2>&1 | python3 -c "
import sys, json
d = json.load(sys.stdin)
print(f\"Name: {d.get('name', '')}\")
print(f\"Reason: {d.get('failureReason', '')}\")
print(f\"Message: {d.get('failureMessage', '')[:200]}\")
"
done
```

Common failure reasons:
- `AppInsufficientResourceError` with "Out of memory" — resubmit with a larger instance type
- `AppError` — check the job log with `dx watch <job-id>` or `dx describe <job-id> --json`

## Check a Single Job

```bash
dx describe <JOB_ID> --json 2>&1 | python3 -c "
import sys, json
d = json.load(sys.stdin)
print(f\"State: {d['state']}\")
print(f\"Name: {d.get('name', '')}\")
"
```

## Resubmit Failed Jobs

Extract the job index from the failed job's name and resubmit with a larger instance type:

```bash
dx run app-swiss-army-knife \
    -icmd="<COMMAND>" \
    --destination "<OUTPUT_PATH>" \
    --instance-type <LARGER_INSTANCE> \
    --priority high \
    --name "<JOB_NAME>" \
    --brief --ignore-reuse -y
```

Determine the appropriate command and parameters from the original submission script.

## Instance Type Reference

| Instance | vCPUs | RAM |
|----------|-------|-----|
| `mem1_ssd1_v2_x2` | 2 | 4 GB |
| `mem1_ssd1_v2_x4` | 4 | 8 GB |
| `mem3_ssd1_v2_x4` | 4 | 32 GB |
| `mem3_ssd1_v2_x8` | 8 | 64 GB |
| `mem3_ssd1_v2_x16` | 16 | 128 GB |
