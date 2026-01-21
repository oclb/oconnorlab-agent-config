//! Handler unit tests
//!
//! Tests for RPC handlers using mock SSH connections.

use crate::commands::{MemorySpec, MemoryUnit, Partition, SandboxedSbatchRequest, TimeSpec};
use crate::rpc::handlers::RpcState;
use crate::ssh::mock::{MockExecutor, MockResponse};
use crate::testing::{self, responses};
use std::sync::Arc;

// ============================================================================
// Helper functions
// ============================================================================

fn create_state(mock: MockExecutor) -> RpcState {
    RpcState::new(Arc::new(mock), testing::test_config())
}

fn create_state_with_config(
    mock: MockExecutor,
    config: crate::config::PermissionConfig,
) -> RpcState {
    RpcState::new(Arc::new(mock), config)
}

// ============================================================================
// connection_status tests
// ============================================================================

#[tokio::test]
async fn test_connection_status_connected() {
    let mock = MockExecutor::new().with_connected(true);
    let state = create_state(mock);

    let status = state.connection_status().await;

    assert!(status.connected);
    assert_eq!(status.user, "testuser");
    assert_eq!(status.host, "testhost");
    assert!(status.instructions.is_none());
}

#[tokio::test]
async fn test_connection_status_disconnected() {
    let mock = MockExecutor::new().with_connected(false);
    let state = create_state(mock);

    let status = state.connection_status().await;

    assert!(!status.connected);
    assert!(status.instructions.is_some());
}

// ============================================================================
// ls tests
// ============================================================================

#[tokio::test]
async fn test_ls_basic() {
    // ls uses execute_with_args which escapes arguments
    // Path validation normalizes trailing slashes
    let mock = MockExecutor::new().expect(
        "ls '--color=never' '/data/input'",
        MockResponse::ok(responses::LS_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::LsRequest {
        path: "/data/input/".to_string(),
        flags: vec![],
    };

    let result = state.ls(request).await;
    assert!(result.is_ok(), "ls failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(!response.entries.is_empty());
}

#[tokio::test]
async fn test_ls_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::LsRequest {
        path: "/unauthorized/path/".to_string(),
        flags: vec![],
    };

    let result = state.ls(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed"));
}

// ============================================================================
// cat tests
// ============================================================================

#[tokio::test]
async fn test_cat_basic() {
    let mock = MockExecutor::new().expect(
        "cat '/data/input/file.txt'",
        MockResponse::ok(responses::CAT_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::CatRequest {
        path: "/data/input/file.txt".to_string(),
        flags: vec![],
        head: None,
        tail: None,
        offset: None,
        limit: None,
    };

    let result = state.cat(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.content, responses::CAT_RESPONSE);
}

#[tokio::test]
async fn test_cat_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::CatRequest {
        path: "/unauthorized/file.txt".to_string(),
        flags: vec![],
        head: None,
        tail: None,
        offset: None,
        limit: None,
    };

    let result = state.cat(request).await;
    assert!(result.is_err());
}

// ============================================================================
// squeue tests
// ============================================================================

#[tokio::test]
async fn test_squeue_empty() {
    let mock = MockExecutor::new().expect(
        "squeue -u testuser -h -o '%i %P %j %u %t %M %D %R'",
        MockResponse::ok(""),
    );
    let state = create_state(mock);

    let request = crate::commands::SqueueRequest {
        user: Some("testuser".to_string()),
        job_ids: vec![],
        partition: None,
        state: None,
    };

    let result = state.squeue(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.jobs.is_empty());
}

// ============================================================================
// sacct tests
// ============================================================================

#[tokio::test]
async fn test_sacct_by_job_id() {
    let mock = MockExecutor::new().expect(
        "sacct --parsable2 --format=JobID,JobName,Partition,State,ExitCode,Elapsed,MaxRSS,MaxVMSize,NCPUs,NTasks -j 12345678",
        MockResponse::ok(responses::SACCT_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::SacctRequest {
        job_ids: vec!["12345678".to_string()],
        user: None,
        start_time: None,
        end_time: None,
        state: None,
    };

    let result = state.sacct(request).await;
    assert!(result.is_ok());
}

// ============================================================================
// git_pull tests
// ============================================================================

#[tokio::test]
async fn test_git_pull_up_to_date() {
    // git_pull quotes the remote and branch args
    let mock = MockExecutor::new().expect(
        "cd '/data/output/project' && git pull 'origin' 'main'",
        MockResponse::ok(responses::GIT_PULL_UP_TO_DATE),
    );
    let state = create_state(mock);

    let request = crate::commands::GitPullRequest {
        path: "/data/output/project".to_string(),
        remote: "origin".to_string(),
        branch: Some("main".to_string()),
    };

    let result = state.git_pull(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.already_up_to_date);
}

// ============================================================================
// sandboxed_sbatch tests
// ============================================================================

#[tokio::test]
async fn test_sandboxed_sbatch_success() {
    let mock = MockExecutor::new()
        .expect("mkdir -p '/scratch/scripts/'", MockResponse::ok(""))
        .with_default(MockResponse::ok("Submitted batch job 12345678"));

    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "python /data/input/script.py".to_string(),
        image: None, // Use default
        partition: Some(Partition::Short),
        cpus: 4,
        memory: Some(MemorySpec {
            amount: 16,
            unit: MemoryUnit::GB,
        }),
        time: Some(TimeSpec {
            days: 0,
            hours: 2,
            minutes: 0,
        }),
        gpu: None,
        array: None,
        working_dir: Some("/data/output/work".to_string()),
        output: None,
        error: None,
        input_paths: vec!["/data/input/".to_string()],
        output_paths: vec!["/data/output/".to_string()],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: true,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_ok(), "Expected success, got: {:?}", result.err());

    let response = result.unwrap();
    assert_eq!(response.job_id, "12345678");
    assert_eq!(response.image_used, "/containers/python.sif");
    assert!(response.script_content.is_some());

    // Verify script content
    let script = response.script_content.unwrap();
    assert!(script.contains("#SBATCH --job-name=test-job"));
    assert!(script.contains("#SBATCH --partition=short"));
    assert!(script.contains("#SBATCH --cpus-per-task=4"));
    assert!(script.contains("#SBATCH --mem=16G"));
    assert!(script.contains("singularity exec"));
    assert!(script.contains("/data/input/:/data/input/:ro"));
    assert!(script.contains("/data/output/:/data/output/:rw"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_no_default_image() {
    let mock = MockExecutor::new();
    let state = create_state_with_config(mock, testing::config_no_default_image());

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None, // No default, should fail
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("image"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_explicit_image() {
    let mock = MockExecutor::new()
        .expect("mkdir -p '/scratch/scripts/'", MockResponse::ok(""))
        .with_default(MockResponse::ok("Submitted batch job 99999999"));

    let state = create_state_with_config(mock, testing::config_no_default_image());

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: Some("/custom/image.sif".to_string()), // Explicit image
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.image_used, "/custom/image.sif");
}

#[tokio::test]
async fn test_sandboxed_sbatch_input_path_not_allowed() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec!["/unauthorized/path/".to_string()],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed") || err.message.contains("Input path"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_output_path_not_allowed() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec!["/unauthorized/output/".to_string()],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed") || err.message.contains("Output path"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_resource_limit_cpus() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 100, // Exceeds limit of 32
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("CPUs") || err.message.contains("exceeds"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_resource_limit_memory() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: Some(MemorySpec {
            amount: 500, // 500GB exceeds limit of 128GB
            unit: MemoryUnit::GB,
        }),
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Memory") || err.message.contains("exceeds"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_resource_limit_gpus() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: Some(crate::commands::GpuSpec {
            count: 10, // Exceeds limit of 2
            gpu_type: None,
        }),
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("GPU") || err.message.contains("exceeds"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_ssh_mkdir_failure() {
    let mock = MockExecutor::new().expect(
        "mkdir -p '/scratch/scripts/'",
        MockResponse::fail("Permission denied"),
    );

    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("directory") || err.message.contains("Permission"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_sbatch_failure() {
    let mock = MockExecutor::new()
        .expect("mkdir -p '/scratch/scripts/'", MockResponse::ok(""))
        .expect("cat > '/scratch/scripts/", MockResponse::ok(""))
        .expect("chmod +x '/scratch/scripts/", MockResponse::ok(""))
        .expect(
            "sbatch '/scratch/scripts/",
            MockResponse::fail("sbatch: error: Batch job submission failed"),
        );

    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("sbatch") || err.message.contains("failed"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_with_array() {
    let mock = MockExecutor::new()
        .expect("mkdir -p '/scratch/scripts/'", MockResponse::ok(""))
        .with_default(MockResponse::ok("Submitted batch job 12345678"));

    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "array-job".to_string(),
        command: "echo $SLURM_ARRAY_TASK_ID".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: Some("1-100%10".to_string()),
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: true,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    let script = response.script_content.unwrap();
    assert!(script.contains("#SBATCH --array=1-100%10"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_array_exceeds_limit() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "array-job".to_string(),
        command: "echo $SLURM_ARRAY_TASK_ID".to_string(),
        image: None,
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: Some("1-5000".to_string()), // Exceeds limit of 1000
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Array") || err.message.contains("exceeds"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_no_scripts_dir() {
    let mock = MockExecutor::new();
    let state = create_state_with_config(mock, testing::config_no_scripts_dir());

    let request = SandboxedSbatchRequest {
        job_name: "test-job".to_string(),
        command: "echo hello".to_string(),
        image: Some("/some/image.sif".to_string()),
        partition: None,
        cpus: 1,
        memory: None,
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec![],
        output_paths: vec![],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false,
        dry_run: false,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("scripts_dir") || err.message.contains("not configured"));
}

#[tokio::test]
async fn test_sandboxed_sbatch_dry_run() {
    // dry_run should still create and write the script, but not submit it
    let mock = MockExecutor::new()
        .expect("mkdir -p '/scratch/scripts/'", MockResponse::ok(""))
        .with_default(MockResponse::ok("")); // For write and chmod, but NOT sbatch

    let state = create_state(mock);

    let request = SandboxedSbatchRequest {
        job_name: "dry-run-test".to_string(),
        command: "python /data/input/script.py".to_string(),
        image: None,
        partition: Some(Partition::Short),
        cpus: 4,
        memory: Some(MemorySpec {
            amount: 16,
            unit: MemoryUnit::GB,
        }),
        time: None,
        gpu: None,
        array: None,
        working_dir: None,
        output: None,
        error: None,
        input_paths: vec!["/data/input/".to_string()],
        output_paths: vec!["/data/output/".to_string()],
        environment: Default::default(),
        extra_directives: Default::default(),
        return_script: false, // dry_run should implicitly return script
        dry_run: true,
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_ok(), "dry_run failed: {:?}", result.err());

    let response = result.unwrap();
    // dry_run returns special job_id
    assert_eq!(response.job_id, "DRY_RUN");
    // Script content should be returned even without return_script=true
    assert!(response.script_content.is_some());

    let script = response.script_content.unwrap();
    assert!(script.contains("#SBATCH --job-name=dry-run-test"));
    assert!(script.contains("#SBATCH --partition=short"));
    assert!(script.contains("singularity exec"));
}

// ============================================================================
// grep tests
// ============================================================================

#[tokio::test]
async fn test_grep_basic() {
    let mock = MockExecutor::new().expect(
        "grep -E -n 'def main' '/data/input/file.py'",
        MockResponse::ok(responses::GREP_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::GrepRequest {
        pattern: "def main".to_string(),
        paths: vec!["/data/input/file.py".to_string()],
        flags: vec![crate::commands::GrepFlag::LineNumbers],
    };

    let result = state.grep(request).await;
    assert!(result.is_ok(), "grep failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(!response.matches.is_empty());
    assert_eq!(response.files_searched, 1);
}

#[tokio::test]
async fn test_grep_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::GrepRequest {
        pattern: "test".to_string(),
        paths: vec!["/unauthorized/file.txt".to_string()],
        flags: vec![],
    };

    let result = state.grep(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed"));
}

#[tokio::test]
async fn test_grep_invalid_regex() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::GrepRequest {
        pattern: "[invalid".to_string(), // Unclosed bracket
        paths: vec!["/data/input/file.txt".to_string()],
        flags: vec![],
    };

    let result = state.grep(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("regex") || err.message.contains("Invalid"));
}

// ============================================================================
// head tests
// ============================================================================

#[tokio::test]
async fn test_head_basic() {
    let mock = MockExecutor::new().expect(
        "head -n 5 '/data/input/file.txt'",
        MockResponse::ok(responses::HEAD_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::HeadRequest {
        path: "/data/input/file.txt".to_string(),
        lines: 5,
    };

    let result = state.head(request).await;
    assert!(result.is_ok(), "head failed: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.lines_returned, 5);
    assert!(response.content.contains("line1"));
}

#[tokio::test]
async fn test_head_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::HeadRequest {
        path: "/unauthorized/file.txt".to_string(),
        lines: 10,
    };

    let result = state.head(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("not allowed"));
}

// ============================================================================
// wc tests
// ============================================================================

#[tokio::test]
async fn test_wc_basic() {
    let mock = MockExecutor::new().expect(
        "wc  '/data/input/file.txt'",
        MockResponse::ok(responses::WC_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::WcRequest {
        path: "/data/input/file.txt".to_string(),
        lines_only: false,
        words_only: false,
        bytes_only: false,
    };

    let result = state.wc(request).await;
    assert!(result.is_ok(), "wc failed: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.lines, Some(10));
    assert_eq!(response.words, Some(50));
    assert_eq!(response.bytes, Some(500));
}

#[tokio::test]
async fn test_wc_lines_only() {
    let mock = MockExecutor::new().expect(
        "wc -l '/data/input/file.txt'",
        MockResponse::ok("      10 /path/to/file.txt"),
    );
    let state = create_state(mock);

    let request = crate::commands::WcRequest {
        path: "/data/input/file.txt".to_string(),
        lines_only: true,
        words_only: false,
        bytes_only: false,
    };

    let result = state.wc(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.lines, Some(10));
    assert!(response.words.is_none());
    assert!(response.bytes.is_none());
}

#[tokio::test]
async fn test_wc_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::WcRequest {
        path: "/unauthorized/file.txt".to_string(),
        lines_only: false,
        words_only: false,
        bytes_only: false,
    };

    let result = state.wc(request).await;
    assert!(result.is_err());
}

// ============================================================================
// find tests
// ============================================================================

#[tokio::test]
async fn test_find_basic() {
    let mock = MockExecutor::new().expect(
        "find '/data/input' -name '*.py' 2>/dev/null | head -n 101",
        MockResponse::ok(responses::FIND_RESPONSE),
    );
    let state = create_state(mock);

    let request = crate::commands::FindRequest {
        path: "/data/input".to_string(),
        name: Some("*.py".to_string()),
        file_type: None,
        max_depth: None,
        limit: 100,
    };

    let result = state.find(request).await;
    assert!(result.is_ok(), "find failed: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.files.len(), 3);
    assert!(!response.truncated);
}

#[tokio::test]
async fn test_find_with_depth() {
    let mock = MockExecutor::new().expect(
        "find '/data/input' -maxdepth 2 -type f 2>/dev/null | head -n 51",
        MockResponse::ok("/data/input/file.txt"),
    );
    let state = create_state(mock);

    let request = crate::commands::FindRequest {
        path: "/data/input".to_string(),
        name: None,
        file_type: Some(crate::commands::FindType::File),
        max_depth: Some(2),
        limit: 50,
    };

    let result = state.find(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.files.len(), 1);
}

#[tokio::test]
async fn test_find_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::FindRequest {
        path: "/unauthorized/path".to_string(),
        name: None,
        file_type: None,
        max_depth: None,
        limit: 100,
    };

    let result = state.find(request).await;
    assert!(result.is_err());
}

// ============================================================================
// download tests
// ============================================================================

#[tokio::test]
async fn test_download_basic() {
    let content = "Hello, World!";
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, content);

    let mock = MockExecutor::new()
        .expect(
            "stat -c%s '/data/input/file.txt' 2>/dev/null || stat -f%z '/data/input/file.txt'",
            MockResponse::ok("13"), // Size of "Hello, World!"
        )
        .expect("base64 '/data/input/file.txt'", MockResponse::ok(&encoded));
    let state = create_state(mock);

    let request = crate::commands::DownloadRequest {
        path: "/data/input/file.txt".to_string(),
    };

    let result = state.download(request).await;
    assert!(result.is_ok(), "download failed: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.size_bytes, 13);
    // Verify content can be decoded
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &response.content,
    );
    assert!(decoded.is_ok());
}

#[tokio::test]
async fn test_download_file_too_large() {
    let mock = MockExecutor::new().expect(
        "stat -c%s '/data/input/large.bin' 2>/dev/null || stat -f%z '/data/input/large.bin'",
        MockResponse::ok("104857601"), // 100MB + 1 byte (exceeds limit)
    );
    let state = create_state(mock);

    let request = crate::commands::DownloadRequest {
        path: "/data/input/large.bin".to_string(),
    };

    let result = state.download(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("too large") || err.message.contains("File too large"));
}

#[tokio::test]
async fn test_download_permission_denied() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::DownloadRequest {
        path: "/unauthorized/file.txt".to_string(),
    };

    let result = state.download(request).await;
    assert!(result.is_err());
}

// ============================================================================
// scancel tests
// ============================================================================

#[tokio::test]
async fn test_scancel_basic() {
    let mock = MockExecutor::new().expect("scancel 12345678", MockResponse::ok(""));
    let state = create_state(mock);

    let request = crate::commands::ScancelRequest {
        job_ids: vec!["12345678".to_string()],
    };

    let result = state.scancel(request).await;
    assert!(result.is_ok(), "scancel failed: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.cancelled_jobs, vec!["12345678"]);
}

#[tokio::test]
async fn test_scancel_multiple_jobs() {
    let mock =
        MockExecutor::new().expect("scancel 12345678 12345679 12345680", MockResponse::ok(""));
    let state = create_state(mock);

    let request = crate::commands::ScancelRequest {
        job_ids: vec![
            "12345678".to_string(),
            "12345679".to_string(),
            "12345680".to_string(),
        ],
    };

    let result = state.scancel(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.cancelled_jobs.len(), 3);
}

#[tokio::test]
async fn test_scancel_empty_job_ids() {
    let mock = MockExecutor::new();
    let state = create_state(mock);

    let request = crate::commands::ScancelRequest { job_ids: vec![] };

    let result = state.scancel(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("No job IDs"));
}

// ============================================================================
// job_wait tests
// ============================================================================

#[tokio::test]
async fn test_job_wait_completed() {
    let mock = MockExecutor::new().expect(
        "sacct -j 12345678 --parsable2 --noheader --format=JobID,State,ExitCode,Elapsed",
        MockResponse::ok("12345678|COMPLETED|0:0|00:05:32"),
    );
    let state = create_state(mock);

    let request = crate::commands::JobWaitRequest {
        job_id: "12345678".to_string(),
        max_wait_secs: 60,
        array_mode: crate::commands::ArrayWaitMode::All,
    };

    let result = state.job_wait(request).await;
    assert!(result.is_ok(), "job_wait failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.all_completed);
    assert_eq!(response.completed_jobs.len(), 1);
    assert_eq!(response.completed_jobs[0].state, "COMPLETED");
}

#[tokio::test]
async fn test_job_wait_failed_job() {
    let mock = MockExecutor::new().expect(
        "sacct -j 12345678 --parsable2 --noheader --format=JobID,State,ExitCode,Elapsed",
        MockResponse::ok("12345678|FAILED|1:0|00:01:15"),
    );
    let state = create_state(mock);

    let request = crate::commands::JobWaitRequest {
        job_id: "12345678".to_string(),
        max_wait_secs: 60,
        array_mode: crate::commands::ArrayWaitMode::All,
    };

    let result = state.job_wait(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.all_completed);
    assert_eq!(response.completed_jobs[0].state, "FAILED");
}

#[tokio::test]
async fn test_job_wait_array_any() {
    // First call returns pending, second returns one completed
    let mock = MockExecutor::new().expect(
        "sacct -j 12345678 --parsable2 --noheader --format=JobID,State,ExitCode,Elapsed",
        MockResponse::ok("12345678_1|COMPLETED|0:0|00:02:00\n12345678_2|RUNNING||"),
    );
    let state = create_state(mock);

    let request = crate::commands::JobWaitRequest {
        job_id: "12345678".to_string(),
        max_wait_secs: 60,
        array_mode: crate::commands::ArrayWaitMode::Any,
    };

    let result = state.job_wait(request).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.completed_jobs.is_empty());
    // Should return as soon as any job completes
    assert!(!response.all_completed); // Not all are done yet
}
