pub trait CommandResult<T> {
    fn OkResponse(data: T) -> Self;
    fn ErrorResponse(err_code: u16, s: &str) -> Self;
}