# run-graphld-o2

Claude Code skill for installing [GraphLD](https://github.com/oclb/graphld) on the HMS O2 cluster from scratch.

## What this skill does

Walks through a complete GraphLD installation on O2, from cloning the repo to a verified test run. It covers every dependency and O2-specific workaround required to get `graphld reml` working.

## Why a naive install doesn't work on O2

A straightforward `git clone && uv sync` will produce a broken installation on O2 due to five independent issues:

1. **No system SuiteSparse.** O2 doesn't provide SuiteSparse as a module. It must be installed via conda, and `LD_LIBRARY_PATH` must be set at runtime so scikit-sparse can find `libcholmod.so`.

2. **OpenBLAS is 100x slower than MKL.** Conda's default BLAS is OpenBLAS. SuiteSparse's sparse Cholesky factorization (the core operation in graphREML) is specifically optimized for Intel MKL. Without switching, jobs that should take 80 minutes take days.

3. **polars crashes on O2 CPUs.** The default polars wheel requires AVX2/FMA instructions that O2 nodes lack. Importing polars produces `Illegal instruction (core dumped)`. The fix is to install `polars-lts-cpu` instead.

4. **scikit-sparse 0.5.0 breaks GraphLD.** Version 0.5.0 changed the `cholesky()` API from returning a callable `Factor` to returning a `(L, perm)` tuple. GraphLD expects the old API. In multiprocessing mode, this error is silently swallowed — all worker processes crash immediately but the parent process hangs forever, appearing to run at 0% CPU until the SLURM time limit. The fix is to pin scikit-sparse to 0.4.16.

5. **Multiprocessing mode is broken.** Even after fixing scikit-sparse, GraphLD's `--num-processes` flag causes silent worker crashes. The `--run-in-serial` flag must be used for all runs. Performance is still good (~80 minutes for full genome with 6 annotations).

## Steps covered

1. Install Miniconda (if needed)
2. Install SuiteSparse via conda
3. Switch BLAS backend from OpenBLAS to Intel MKL
4. Clone GraphLD and run `uv sync` with SuiteSparse paths
5. Replace polars with polars-lts-cpu
6. Pin scikit-sparse to 0.4.16
7. Download data (precision matrices, annotations, optionally summary statistics)
8. Create an `activate.sh` convenience script
9. Verify with a chromosome 22 test run

## Invocation

```
/run-graphld-o2
```
