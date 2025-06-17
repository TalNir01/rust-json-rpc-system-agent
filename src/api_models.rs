use serde::{Deserialize, Serialize};
use std::io::{self};
use std::process::Stdio;
use tokio::process::Command;

// ############## Response ##############

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_status: i32, // Support negative numbers as well
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandTimedOut {
    pub time: i64, // TODO: Fix if needed
    pub error_message: String,
    pub command_pid: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandExecutionResult {
    Response(CommandResponse),
    Timeout(CommandTimedOut),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalApiError {
    pub error_message: String,
    pub error_code: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum ApiResponse {
    CommandExecutionOk(CommandResponse),
    CommandExecutionTimeOut(CommandTimedOut),
    CommandSystemError(InternalApiError),
    GenericError(InternalApiError),
}

// ############## Request ##############

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub cmd: String,
    pub timeout: u32, // When putting 0 command will not wait, just return empty response.
}

impl CommandRequest {
    pub async fn execute_without_timeout(&self) -> std::io::Result<CommandExecutionResult> {
        // Using spawn to execute
        let cmd = self.cmd.clone();
        tokio::spawn(async move {
            let _ = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .kill_on_drop(false)
                .spawn();
        });
        Ok(CommandExecutionResult::Response(CommandResponse {
            stdout: "".to_string(),
            stderr: "".to_string(),
            exit_status: 0,
        }))
    }

    pub async fn execute_with_timeout(&self) -> std::io::Result<CommandExecutionResult> {
        let timeout_duration = std::time::Duration::from_secs(self.timeout as u64); // Generate duration object

        let child = Command::new("sh")
            .arg("-c")
            .arg(self.cmd.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let child_pid = child.id().unwrap();
        let out_fut = child.wait_with_output();
        match tokio::time::timeout(timeout_duration, out_fut).await {
            Ok(Ok(output)) => Ok(CommandExecutionResult::Response(CommandResponse {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_status: output.status.code().unwrap_or(-1),
            })),
            Ok(Err(e)) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Process error: {e}"),
            )),
            Err(_) => {
                // We exits and it automatically killed.
                Ok(CommandExecutionResult::Timeout(CommandTimedOut {
                    time: self.timeout as i64,
                    error_message: "Command didn\'t finish before timeout. It was killed"
                        .to_string(),
                    command_pid: child_pid, // The pid we kill
                }))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum ApiRequest {
    ExecCmd(CommandRequest), // TODO: Add more features in the future
}
