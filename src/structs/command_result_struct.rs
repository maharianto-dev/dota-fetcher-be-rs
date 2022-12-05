use crate::traits::command_result_traits::CommandResult;
use rocket::http::Status;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandResultStruct<T> {
    pub response_status: u16,
    pub response_message: Option<String>,
    pub response_data: Option<T>,
}

impl<T> CommandResult<T> for CommandResultStruct<T> {
    fn ok_response(data: T) -> Self {
        return Self {
            response_status: Status::Ok.code,
            response_message: None,
            response_data: Some(data),
        };
    }

    fn error_response(err_code: u16, s: &str) -> Self {
        return Self {
            response_status: err_code,
            response_message: Some(s.to_string()),
            response_data: None,
        };
    }
}
