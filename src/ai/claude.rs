//! Claude CLI detection and invocation.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Maximum time to wait for a single Claude CLI invocation.
const TIMEOUT_SECS: u64 = 180;

/// Detect whether the Claude CLI is available.
///
/// Returns the path to the `claude` binary, or `None` if not found.
pub(crate) fn detect_cli() -> Option<PathBuf> {
    let output = Command::new("which").arg("claude").output().ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            None
        } else {
            Some(PathBuf::from(path))
        }
    } else {
        None
    }
}

/// Invoke Claude CLI with a prompt in the given directory.
///
/// Uses `claude -p "<prompt>" --output-format json` to get structured output.
/// Returns the raw stdout string on success.
pub(crate) fn invoke(
    cli_path: &Path,
    target_dir: &Path,
    prompt: &str,
) -> Result<String, InvokeError> {
    let child = Command::new(cli_path)
        .args(["-p", prompt, "--output-format", "json", "--model", "haiku"])
        .current_dir(target_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| InvokeError::SpawnFailed(e.to_string()))?;

    let output = wait_with_timeout(child, Duration::from_secs(TIMEOUT_SECS))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(InvokeError::NonZeroExit {
            code: output.status.code(),
            stderr,
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.trim().is_empty() {
        return Err(InvokeError::EmptyOutput);
    }

    // Claude CLI --output-format json wraps the response in an envelope:
    // {"type":"result","result":"...actual content..."}
    // Extract the inner "result" field if present.
    Ok(extract_result_field(&stdout))
}

/// Extract the "result" field from Claude CLI's JSON envelope.
///
/// If the output is a JSON object with a "result" string field, returns that string.
/// Otherwise returns the raw output unchanged.
fn extract_result_field(raw: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw.trim()) {
        if let Some(result) = value.get("result").and_then(|v| v.as_str()) {
            return result.to_string();
        }
    }
    raw.to_string()
}

/// Wait for a child process with a timeout.
fn wait_with_timeout(
    child: std::process::Child,
    timeout: Duration,
) -> Result<std::process::Output, InvokeError> {
    // Use a thread to wait with timeout
    let (tx, rx) = std::sync::mpsc::channel();

    let child_id = child.id();
    let handle = std::thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(output)) => {
            let _ = handle.join();
            Ok(output)
        }
        Ok(Err(e)) => {
            let _ = handle.join();
            Err(InvokeError::WaitFailed(e.to_string()))
        }
        Err(_) => {
            // Timeout — kill the child process to avoid zombies
            let _ = Command::new("kill")
                .args(["-9", &child_id.to_string()])
                .output();
            let _ = handle.join();
            Err(InvokeError::Timeout)
        }
    }
}

/// Errors that can occur during Claude CLI invocation.
#[derive(Debug, thiserror::Error)]
pub(crate) enum InvokeError {
    /// Failed to spawn the process.
    #[error("failed to spawn claude: {0}")]
    SpawnFailed(String),

    /// Process exited with non-zero status.
    #[error("claude exited with code {code:?}: {stderr}")]
    NonZeroExit {
        /// Exit code, if available.
        code: Option<i32>,
        /// stderr output.
        stderr: String,
    },

    /// Process produced no output.
    #[error("claude produced no output")]
    EmptyOutput,

    /// Process timed out.
    #[error("claude timed out after 180 seconds")]
    Timeout,

    /// Failed to wait for process.
    #[error("failed to wait for claude: {0}")]
    WaitFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_cli_returns_some_or_none() {
        // This test just verifies the function doesn't panic.
        // Result depends on whether claude is installed.
        let _result = detect_cli();
    }

    #[test]
    fn invoke_error_display() {
        let err = InvokeError::Timeout;
        assert!(err.to_string().contains("timed out"));

        let err = InvokeError::EmptyOutput;
        assert!(err.to_string().contains("no output"));

        let err = InvokeError::NonZeroExit {
            code: Some(1),
            stderr: "auth failed".to_string(),
        };
        assert!(err.to_string().contains("auth failed"));
    }
}
