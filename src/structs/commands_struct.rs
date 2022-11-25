use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponseStruct<T> {
    pub message: &'static str,
    pub data: Option<T>,
}
