use crate::IOResult;

pub trait Executable {
    fn execute(&self) -> IOResult<()>;
}
