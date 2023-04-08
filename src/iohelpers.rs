#[macro_export]
macro_rules! ioerror {
    ( $tmpl:expr, $( $arg:expr ),* ) => {
        std::io::Error::new(std::io::ErrorKind::Other, format!($tmpl, $( $arg ),* ))
    }
}

pub fn invalid_input<T, E>(reason: &str, input: E) -> std::io::Result<T>
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
