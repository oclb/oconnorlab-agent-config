# O2 Cluster Reference

Reference material about the Harvard O2 cluster and SLURM workload manager. This skill provides knowledge that `/remote-o2` uses when executing cluster commands.

## Overview

**This is a reference skill, not directly invoked.** Use `/remote-o2` to access O2 from your local machine. This skill provides the SLURM knowledge that remote-o2 needs.

**What this skill knows:**
- Partition selection (short, medium, long, gpu, highmem)
- Resource estimation (memory, time, cores)
- SLURM script templates
- Job submission and monitoring
- Troubleshooting common issues

## O2 Cluster Basics

**O2** is Harvard Medical School's shared HPC cluster using **SLURM** (Simple Linux Utility for Resource Management).

**Main Partitions:**

| Partition | Time Limit | Use Case |
|-----------|------------|----------|
| `priority` | 5 days | Single urgent job (dispatched first) |
| `short` | 12 hours | Default for most jobs |
| `medium` | 5 days | Longer running jobs |
| `long` | 30 days | Very long (requires RC access) |
| `interactive` | 12 hours | Interactive sessions |
| `highmem` | Varies | Memory >200GB |
| `gpu` | Varies | GPU computation |

## Quick Reference

### Submit a Job
```bash
# With script file
sbatch my_job.sh

# Inline
sbatch -p short -t 0-02:00 -c 4 --mem=16G --wrap="python analysis.py"
```

### Basic SLURM Script
```bash
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-04:00
#SBATCH -c 8
#SBATCH --mem=32G
#SBATCH -o %j.out
#SBATCH -e %j.err
#SBATCH -J my_analysis

module load python/3.9.14
python analysis.py
```

### Monitor Jobs
```bash
squeue -u $USER        # Check status
seff 12345678          # Check efficiency
scancel 12345678       # Cancel job
tail -f 12345678.out   # Follow output
```

## Resource Estimation

### Memory Guidelines
| Task Type | Typical Memory |
|-----------|----------------|
| Small datasets | 8-16GB |
| Medium datasets | 32-64GB |
| Large datasets | 128GB+ |
| Very large | highmem partition |

**Rule of thumb:** 2-3x the size of data loaded into memory

### Time Strategy
- For `short` partition: request `0-12:00` (max)
- For `medium` partition: request `5-00:00` (max)
- GPU jobs: estimate carefully (higher competition)

### Cores
Only request what you'll use. Check if code supports parallelism:
- Python: `multiprocessing`, `joblib`
- R: `parallel`, `foreach`
- Tools: `-j`, `--threads` flags

## Common Issues

### Job Pending
- Cluster busy → wait
- Request too large → reduce resources
- Time too long → use different partition

### Job Fails
- Check error file: `cat 12345678.err`
- Common causes: module not loaded, file not found, out of memory

### Low Efficiency
- `seff` shows <50% CPU → code not parallelized, reduce cores
- Memory <30% used → reduce `--mem` request

## Best Practices

1. **Test first** - small data, interactive session
2. **Estimate conservatively** - add 20-30% buffer
3. **Check efficiency** - always run `seff` after completion
4. **Use job arrays** - for multiple similar jobs
5. **Document** - record actual resource usage for future reference

## Notifications

Jobs can send notifications via ntfy.sh:

```bash
# In SLURM script
notify_job_complete $?

# Or inline
sbatch --wrap="python analysis.py && notify 'Done!'"
```

See the main README for notification setup.

## Resources

- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [SLURM Cheat Sheet](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1594261585/O2+Command+CheatSheet)
- [Partition Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793641)
