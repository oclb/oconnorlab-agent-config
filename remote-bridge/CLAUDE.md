# Remote Bridge

Secure bridge for Claude Code to access remote hosts (O2 cluster) via SSH. Provides a JSON-RPC interface over Unix sockets with permission-based access control.

## Architecture

```
Claude Code  <-->  Unix Socket  <-->  remote-bridge  <-->  SSH  <-->  O2 Cluster
                   (JSON-RPC)         (Rust binary)        (PTY)
```

### Key Components

| Module | Purpose |
|--------|---------|
| `src/rpc/` | JSON-RPC server and method dispatch |
| `src/ssh/` | SSH connection via PTY for Duo auth support |
| `src/commands/` | Request/response types and validation |
| `src/config/` | Permission config with integrity checking |
| `src/sbatch/` | Sandboxed SLURM job script generation |

### Permission Model

All operations are validated against `permissions.toml`:
- **Read paths**: Directories Claude can read from
- **Write paths**: Directories Claude can write to
- **Resource limits**: Max CPUs, memory, time, GPUs, array size
- **Singularity config**: Container sandboxing for job submission

## RPC Methods

| Method | Purpose |
|--------|---------|
| `connection_status` | Check SSH connection state |
| `ls`, `cat`, `grep`, `head`, `wc`, `find` | Filesystem operations |
| `download` | Base64-encoded file download |
| `git_pull` | Pull changes in a git repo |
| `squeue`, `sacct` | Query SLURM job status |
| `sbatch`, `sandboxed_sbatch` | Submit SLURM jobs |
| `scancel` | Cancel SLURM jobs |
| `job_wait` | Poll until job completes |

### Sandboxed Sbatch

The `sandboxed_sbatch` method provides secure job submission using Singularity containers:
- Input paths mounted read-only
- Output paths mounted read-write
- Resource limits enforced from config
- Script generated and submitted automatically

## Testing

### Test Architecture

The codebase uses trait-based mocking for testability:

```
RemoteExecutor (trait)
├── SshConnection (production)
└── MockExecutor (testing)
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_sandboxed_sbatch_success

# Run integration tests (requires running server)
cargo test --test rpc_integration -- --ignored
```

### Test Organization

| Location | Coverage |
|----------|----------|
| `src/rpc/handlers_test.rs` | RPC handler unit tests (26 tests) |
| `src/ssh/mock.rs` | MockExecutor tests (6 tests) |
| `src/sbatch/mod.rs` | Script generation tests (4 tests) |
| `src/commands/*.rs` | Type parsing tests |
| `src/config/*.rs` | Config loading/validation tests |
| `tests/rpc_integration.rs` | JSON-RPC protocol tests |

### MockExecutor Usage

```rust
use crate::ssh::mock::{MockExecutor, MockResponse};
use crate::testing::test_config;

#[tokio::test]
async fn test_example() {
    let mock = MockExecutor::new()
        .expect("ls '--color=never' '/data/input'", MockResponse::ok("file.txt"))
        .with_default(MockResponse::fail("unexpected command"));

    let state = RpcState::new(Arc::new(mock), test_config());
    // ... test the handler
}
```

### Test Fixtures

`src/testing/mod.rs` provides:
- `test_config()` - Standard permission config
- `config_no_default_image()` - Config without singularity default
- `config_no_scripts_dir()` - Config without scripts directory
- `responses::*` - Common mock SSH responses

## Development

### Building

```bash
cargo build --release
```

### Adding a New RPC Method

1. Define request/response types in `src/commands/`
2. Add handler method to `RpcState` in `src/rpc/handlers.rs`
3. Add dispatch case in `src/rpc/server.rs`
4. Add tests in `src/rpc/handlers_test.rs`
5. Update method list in server startup log

### Key Patterns

**Path validation**: All paths are validated against config before use
```rust
let validated = self.validator.validate_read_path(&request.path)?;
```

**Command execution**: Use `execute_with_args` for proper escaping
```rust
self.ssh.execute_with_args("ls", &["--color=never", path], 30).await
```

**Error handling**: Return `RpcError` with appropriate code
```rust
Err(RpcError {
    code: ERR_PERMISSION_DENIED,
    message: "Path not allowed".to_string(),
    data: None,
})
```

## Configuration

### permissions.toml

```toml
[paths]
read = ["/n/data1/lab/", "/n/scratch/user/"]
write = ["/n/scratch/user/"]

[resources]
max_cpus = 32
max_memory_gb = 128
max_time_hours = 120
max_gpus = 2
max_array_size = 1000

[singularity]
default_image = "/n/app/containers/python.sif"
scripts_dir = "/n/scratch/user/scripts/"
cache_dir = "/n/scratch/user/.singularity"
extra_binds = ["/n/app:ro"]
```

### Config Integrity

The bridge verifies config hasn't been tampered with using SHA-256 checksums:
```bash
remote-bridge verify-config
remote-bridge update-checksum  # After intentional changes
```
