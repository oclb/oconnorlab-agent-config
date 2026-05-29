---
name: run-graphld-o2
description: Install and run GraphLD graphREML on the HMS O2 cluster, including SuiteSparse, scikit-sparse, MKL, polars-lts-cpu, data downloads, and SLURM job setup. Use when the user asks to install GraphLD on O2, set up GraphLD on O2, or get graphREML running on the O2 cluster.
recommended_scope: project
version: 1.0.0
---

# Install GraphLD on O2

## Role

GraphLD installation specialist for the HMS O2 cluster. Guide the user through a complete install of GraphLD and its dependencies, handling the O2-specific issues that prevent a naive install from working.

## Goal

Get `graphld reml` running successfully on O2 with correct dependencies, BLAS backend, and environment configuration.

This is intentionally an O2-specific skill. For generic GraphLD usage outside O2, prefer the GraphLD repository documentation or a project-local GraphLD skill.

## Before Starting

Determine:

1. Where GraphLD should be installed, for example `/n/data1/hms/dbmi/oconnor/lab/<username>/bin/graphld/`.
2. The user's O2 username, needed for paths and SLURM context.

Store the install path as `$GRAPHLD` and the username as `$O2_USER` throughout. Do not use `$USER` as a placeholder; it is normally already set by the shell.

## Step 1: Install Miniconda If Needed

Check if conda is available:

```bash
which conda
```

If not found, install Miniconda:

```bash
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O /tmp/miniconda.sh
bash /tmp/miniconda.sh -b -p "$HOME/miniconda3"
eval "$("$HOME/miniconda3/bin/conda" shell.bash hook)"
conda init bash
```

Then log out and back in, or source the updated shell startup file.

## Step 2: Install SuiteSparse Via Conda

GraphLD depends on scikit-sparse, which requires the CHOLMOD library from SuiteSparse. Install it into the conda base environment:

```bash
conda install -n base -y suitesparse
```

Record the paths; they are needed later:

```bash
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib
echo "Include: $SUITESPARSE_INCLUDE_DIR"
echo "Library: $SUITESPARSE_LIBRARY_DIR"
```

Verify CHOLMOD is present:

```bash
ls "$SUITESPARSE_LIBRARY_DIR"/libcholmod*
```

## Step 3: Switch BLAS to Intel MKL

SuiteSparse performance is extremely sensitive to the BLAS backend. OpenBLAS can be much slower than Intel MKL for the sparse Cholesky operations GraphLD uses.

```bash
conda install -n base -y 'libblas=*=*mkl' mkl
```

Verify:

```bash
ls -la "$(conda info --base)/lib/libblas.so.3"
```

The symlink should point to `libmkl_rt.so.2`, not `libopenblasp*.so`.

## Step 4: Clone and Install GraphLD

```bash
if [ ! -d "$GRAPHLD/.git" ]; then
    git clone https://github.com/oclb/graphld.git "$GRAPHLD"
fi
cd "$GRAPHLD"
uv venv
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include \
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib \
uv sync
```

## Step 5: Fix Polars AVX2 Incompatibility

O2 compute nodes may lack AVX2 CPU instructions. The default `polars` wheel can crash with `Illegal instruction`. Replace it with the CPU-compatible build:

```bash
uv pip uninstall --python "$GRAPHLD/.venv/bin/python" polars polars-runtime-32
```

Uninstalling `polars` can leave a partial `polars/` directory in site-packages that shadows the new install. Remove it only after confirming the target path resolves inside `$GRAPHLD/.venv/lib/`:

```bash
POLARS_DIR=$(find "$GRAPHLD/.venv/lib" -path '*/site-packages/polars' -type d -print -quit)
case "$POLARS_DIR" in
    "$GRAPHLD"/.venv/lib/*/site-packages/polars) rm -rf "$POLARS_DIR" ;;
    "") ;;
    *) echo "Refusing to remove unexpected path: $POLARS_DIR" >&2; exit 1 ;;
esac
uv pip install --python "$GRAPHLD/.venv/bin/python" 'polars-lts-cpu>=1.31.0'
```

Verify:

```bash
"$GRAPHLD/.venv/bin/python" -c "import polars; print(polars.__version__)"
```

## Step 6: Pin scikit-sparse to 0.4.16

scikit-sparse 0.5.0 introduced a breaking API change: `cholesky()` returns a `(L, perm)` tuple instead of a callable `Factor`. GraphLD expects the old API. With 0.5.0, jobs may crash immediately in serial mode or silently hang in multiprocessing mode because worker process errors can be swallowed.

```bash
SUITESPARSE_INCLUDE_DIR=$(conda info --base)/include/suitesparse \
SUITESPARSE_LIBRARY_DIR=$(conda info --base)/lib \
uv pip install --python "$GRAPHLD/.venv/bin/python" 'scikit-sparse==0.4.16'
```

Verify the version:

```bash
"$GRAPHLD/.venv/bin/python" -c "import sksparse; print(sksparse.__version__)"
```

It must show `0.4.16`.

## Step 7: Download Data

GraphLD needs LDGM precision matrices and baselineLD annotations at minimum. Summary statistics are optional.

```bash
cd "$GRAPHLD/data"
```

Minimum for running graphREML, approximately 2 GB:

```bash
make download_reml
```

This downloads:

- UK Biobank LDGM precision matrices, approximately 1.5 GB.
- baselineLD annotations, approximately 400 MB.
- surrogate marker files, approximately 60 MB.

Optional bundled UK Biobank summary statistics, approximately 7 GB:

```bash
make download_sumstats
```

Download everything, approximately 25 GB:

```bash
make download_all
```

## Step 8: Create activate.sh

Create a convenience script that sets the required environment:

```bash
CONDA_LIB=$(conda info --base)/lib
cat > "$GRAPHLD/activate.sh" << EOF
#!/bin/bash
export LD_LIBRARY_PATH=${CONDA_LIB}:\$LD_LIBRARY_PATH
export MKL_NUM_THREADS=1
export OMP_NUM_THREADS=1
source ${GRAPHLD}/.venv/bin/activate
EOF
chmod +x "$GRAPHLD/activate.sh"
```

Use resolved paths in the generated file because the runtime shell may not have `$GRAPHLD` or `conda` initialized.

## Step 9: Verify Installation

Run a quick test with chromosome 22:

```bash
source "$GRAPHLD/activate.sh"
graphld reml \
    "$GRAPHLD/data/sumstats/sumstats/body_HEIGHTz.sumstats" \
    /tmp/graphld_test \
    -a "$GRAPHLD/data/annot/" \
    --metadata "$GRAPHLD/data/ldgms/metadata.csv" \
    --run-in-serial \
    -c 22 \
    -v
```

Expected result: completes in roughly 60 seconds with convergence output. Check that `/tmp/graphld_test.tall.csv` exists and has reasonable values.

If it fails, check:

- `ImportError` mentioning `libcholmod`: `LD_LIBRARY_PATH` is not set.
- `TypeError: 'tuple' object is not callable`: scikit-sparse is 0.5.0 and needs downgrade.
- `Illegal instruction`: the default `polars` wheel is incompatible and should be replaced by `polars-lts-cpu`.

## SLURM Job Template

For production jobs:

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
"$GRAPHLD/.venv/bin/graphld" reml \
    "$GRAPHLD/data/sumstats/sumstats/TRAIT.sumstats" \
    /path/to/output_prefix \
    -a "$GRAPHLD/data/annot/" \
    --metadata "$GRAPHLD/data/ldgms/metadata.csv" \
    --run-in-serial \
    -v
```

Replace `CONDA_LIB_PATH`, `GRAPHLD_PATH`, `TRAIT`, and the output path.

Critical SLURM flags and environment:

- `--run-in-serial` is mandatory because multiprocessing mode can silently hang.
- `MKL_NUM_THREADS=1` prevents thread oversubscription.
- 16 GB memory is sufficient for full-genome runs with approximately 100 annotations.
- Full-genome runtime is about 80 minutes per trait with 6 annotations; more annotations take longer.

## Required Environment Variables

These must be set in every SLURM job or interactive session:

| Variable | Value | Why |
| --- | --- | --- |
| `LD_LIBRARY_PATH` | `<conda_base>/lib:$LD_LIBRARY_PATH` | scikit-sparse needs `libcholmod.so` at runtime. |
| `MKL_NUM_THREADS` | `1` | Prevents thread oversubscription. |
| `OMP_NUM_THREADS` | `1` | Prevents thread oversubscription. |
