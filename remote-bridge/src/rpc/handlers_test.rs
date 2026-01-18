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
    let mock = MockExecutor::new().expect("cat '/data/input/file.txt'", MockResponse::ok(responses::CAT_RESPONSE));
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
    let mock = MockExecutor::new()
        .expect(
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
        .expect("sbatch '/scratch/scripts/", MockResponse::fail("sbatch: error: Batch job submission failed"));

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
    };

    let result = state.sandboxed_sbatch(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("scripts_dir") || err.message.contains("not configured"));
}
