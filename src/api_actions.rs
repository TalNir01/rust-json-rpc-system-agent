use crate::api_models::{ApiResponse, CommandExecutionResult, CommandRequest, InternalApiError};

// Api has a 'process' function which return a serializable object
pub trait ProcessApiRequest {
    // type Response: Serialize; //

    async fn process(&self) -> ApiResponse;
}

impl ProcessApiRequest for CommandRequest {
    // type Response: ApiResponse::OkResponse(EchoResponse);

    async fn process(&self) -> ApiResponse {
        let execution_result: std::io::Result<CommandExecutionResult> = match self.timeout {
            0 => self.execute_without_timeout().await,
            _ => self.execute_with_timeout().await,
        };
        match execution_result {
            Ok(CommandExecutionResult::Response(command_response)) => {
                ApiResponse::CommandExecutionOk(command_response)
            }
            Ok(CommandExecutionResult::Timeout(command_has_timeout)) => {
                ApiResponse::CommandExecutionTimeOut(command_has_timeout)
            }
            Err(e) => ApiResponse::CommandSystemError(InternalApiError {
                error_message: e.to_string(),
                error_code: e.raw_os_error().unwrap_or(-1) as i64,
            }),
        }
    }
}
