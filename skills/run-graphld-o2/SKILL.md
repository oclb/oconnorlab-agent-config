---
name: run-graphld-o2
description: Install GraphLD on the HMS O2 cluster from scratch, including all dependencies (SuiteSparse, scikit-sparse, MKL, polars-lts-cpu) and data downloads. Use when the user asks to "install graphld", "set up graphld on O2", or needs to get graphREML working on the cluster.
user_invocable: true
version: 1.0.0
---

# Install GraphLD on O2

## Role

GraphLD installation specialist for the HMS O2 cluster. Guides the user through a complete install of GraphLD and its dependencies, handling the O2-specific gotchas that prevent a naive install from working.

## Goal

Get `graphld reml` running successfully on O2 with correct dependencies, BLAS backend, and environment configuration.

## Before Starting

Ask the user:
1. **Where should GraphLD be installed?** (e.g., `/n/data1/hms/dbmi/oconnor/lab/<username>/bin/graphld/`)
2. **What is your O2 username?** (needed for conda paths and SLURM)

Store the install path as `$GRAPHLD` and the username as `$USER` throughout.

## Step 1: Install Miniconda (if not present)

Check if conda is available:

```bash
which conda
```

If not found, install Miniconda:

```bash
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O /tmp/miniconda.sh
bash /tmp/miniconda.sh -b -p $HOME/miniconda3
eval "$($HOME/miniconda3/bin/conda shell.bash hook)"
conda init bash
```

Then log out and back in (or `source ~/.bashrc`).

## Step 2: Install SuiteSparse via Conda

GraphLD depends on scikit-sparse, which requires the CHOLMOD library from SuiteSparse. Install it into the conda base environment:

```bash
conda install -n base -y suitesparse
```

Record the paths — they'll be needed later:

```bash
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib
echo "Include: $SUITESPARSE_INCLUDE_DIR"
echo "Library: $SUITESPARSE_LIBRARY_DIR"
```

Verify CHOLMOD is present:

```bash
ls $SUITESPARSE_LIBRARY_DIR/libcholmod*
```

## Step 3: Switch BLAS to Intel MKL

SuiteSparse performance is extremely sensitive to the BLAS backend. OpenBLAS (the default) can be **100x slower** than Intel MKL for the sparse Cholesky operations GraphLD uses.

```bash
conda install -n base -y 'libblas=*=*mkl' mkl
```

Verify:

```bash
ls -la $(conda info --base)/lib/libblas.so.3
```

The symlink should point to `libmkl_rt.so.2`, not `libopenblasp*.so`.

## Step 4: Clone and Install GraphLD

```bash
git clone https://github.com/oclb/graphld.git $GRAPHLD
cd $GRAPHLD
uv venv
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include \
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib \
uv sync
```

## Step 5: Fix Polars (AVX2 Incompatibility)

O2 compute nodes lack AVX2 CPU instructions. The default `polars` wheel will crash with `Illegal instruction`. Replace it with the CPU-compatible build:

```bash
uv pip uninstall --python $GRAPHLD/.venv/bin/python polars polars-runtime-32
```

**Important:** Uninstalling polars always leaves a partial `polars/` directory in site-packages that will shadow the new install. Remove it before installing the replacement:

```bash
rm -rf $GRAPHLD/.venv/lib/python*/site-packages/polars/
uv pip install --python $GRAPHLD/.venv/bin/python 'polars-lts-cpu>=1.31.0'
```

Verify:

```bash
$GRAPHLD/.venv/bin/python -c "import polars; print(polars.__version__)"
```

## Step 6: Pin scikit-sparse to 0.4.16

scikit-sparse 0.5.0 introduced a breaking API change: `cholesky()` returns a `(L, perm)` tuple instead of a callable `Factor`. GraphLD expects the old API. With 0.5.0, jobs will either crash immediately (serial mode) or **silently hang forever** (multiprocessing mode — errors are swallowed by worker processes).

```bash
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include/suitesparse \
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib \
uv pip install --python $GRAPHLD/.venv/bin/python 'scikit-sparse==0.4.16'
```

Verify the version:

```bash
$GRAPHLD/.venv/bin/python -c "import sksparse; print(sksparse.__version__)"
```

Must show `0.4.16`.

## Step 7: Download Data

GraphLD needs LDGM precision matrices and baselineLD annotations at minimum. Summary statistics are optional.

```bash
cd $GRAPHLD/data
```

**Minimum for running graphREML (~2 GB):**

```bash
make download_reml
```

This downloads:
- UK Biobank LDGM precision matrices (~1.5 GB)
- baselineLD annotations (~400 MB)
- Surrogate marker files (~60 MB)

**Optional: Download 20 UK Biobank summary statistics (~7 GB):**

Ask the user: *"Would you like to download the bundled UK Biobank summary statistics (20 traits, ~7 GB)?"*

If yes:

```bash
make download_sumstats
```

This provides sumstats for: AD, body_BMIz, body_HEIGHTz, BowelCancer, BreastCancer, CAD, COPD, cov_EDU_YEARS, Depression, HDL, HTN, LDL, LungCancer, mental_NEUROTICISM, PD, ProstateCancer, RBC, Stroke, T2D, WBC.

**Download everything (~25 GB):**

```bash
make download_all
```

## Step 8: Create activate.sh

Create a convenience script that sets the required environment:

```bash
cat > $GRAPHLD/activate.sh << 'ACTIVATE'
#!/bin/bash
export LD_LIBRARY_PATH=$(conda info --base)/lib:$LD_LIBRARY_PATH
export MKL_NUM_THREADS=1
export OMP_NUM_THREADS=1
source $GRAPHLD/.venv/bin/activate
ACTIVATE
```

**Important:** Replace `$(conda info --base)` and `$GRAPHLD` with the actual resolved paths in the file, since it won't have access to those variables at source-time:

```bash
CONDA_LIB=$(conda info --base)/lib
cat > $GRAPHLD/activate.sh << EOF
#!/bin/bash
export LD_LIBRARY_PATH=${CONDA_LIB}:\$LD_LIBRARY_PATH
export MKL_NUM_THREADS=1
export OMP_NUM_THREADS=1
source ${GRAPHLD}/.venv/bin/activate
EOF
```

## Step 9: Verify Installation

Run a quick test with chromosome 22 (smallest, fastest):

```bash
source $GRAPHLD/activate.sh
graphld reml \
    $GRAPHLD/data/sumstats/sumstats/body_HEIGHTz.sumstats \
    /tmp/graphld_test \
    -a $GRAPHLD/data/annot/ \
    --metadata $GRAPHLD/data/ldgms/metadata.csv \
    --run-in-serial \
    -c 22 \
    -v
```

**Expected:** Completes in ~60 seconds with convergence output. Check that `/tmp/graphld_test.tall.csv` exists and has reasonable values.

If it fails, check:
- `ImportError` mentioning `libcholmod` → `LD_LIBRARY_PATH` not set
- `TypeError: 'tuple' object is not callable` → scikit-sparse is 0.5.0, needs downgrade
- `Illegal instruction` → polars AVX2 issue, need polars-lts-cpu

## SLURM Job Template

For submitting production jobs:

```bash
#!/bin/bash
#SBATCH -p medium
#SBATCH -t 24:00:00
#SBATCH --mem=16G
#SBATCH -c 1
#SBATCH -o %x.%j.out
#SBATCH -e %x.%j.err

export LD_LIBRARY_PATH=CONDA_LIB_PATH:$LD_LIBRARY_PATH
export MKL_NUM_THREADS=1
export OMP_NUM_THREADS=1

GRAPHLD=GRAPHLD_PATH
$GRAPHLD/.venv/bin/graphld reml \
    $GRAPHLD/data/sumstats/sumstats/TRAIT.sumstats \
    /path/to/output_prefix \
    -a $GRAPHLD/data/annot/ \
    --metadata $GRAPHLD/data/ldgms/metadata.csv \
    --run-in-serial \
    -v
```

Replace `CONDA_LIB_PATH`, `GRAPHLD_PATH`, `TRAIT`, and the output path.

**Critical SLURM flags:**
- `--run-in-serial` is **mandatory** — multiprocessing mode silently hangs
- `MKL_NUM_THREADS=1` prevents thread oversubscription
- 16G memory is sufficient for full-genome runs with ~100 annotations
- ~80 minutes per trait for full genome (6 annotations); longer for more annotations

## Summary of Required Environment Variables

These must be set in every SLURM job or interactive session:

| Variable | Value | Why |
|----------|-------|-----|
| `LD_LIBRARY_PATH` | `<conda_base>/lib:$LD_LIBRARY_PATH` | scikit-sparse needs libcholmod.so at runtime |
| `MKL_NUM_THREADS` | `1` | Prevents thread oversubscription with multiprocessing |
| `OMP_NUM_THREADS` | `1` | Same as above |
