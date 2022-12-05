pub trait CommandResult<T> {
    fn ok_response(data: T) -> Self;
    fn error_response(err_code: u16, s: &str) -> Self;
}
