pub trait Executable {
    fn execute(&self) -> std::io::Result<()>;
}
