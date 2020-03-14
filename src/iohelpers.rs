pub use std::io::Result as IOResult;

pub fn invalid_input<T, E>(reason: &str, input: E) -> IOResult<T>
where
    E: std::fmt::Debug,
{
    Err(invalid_input_error(reason, input))
}

pub fn invalid_input_error<E>(reason: &str, input: E) -> std::io::Error
where
    E: std::fmt::Debug,
{
    use std::io::{Error, ErrorKind};

    Error::new(ErrorKind::InvalidInput, format!("{}: {:?}", reason, input))
}
