pub trait Executable {
    fn execute(&self) -> anyhow::Result<()>;
}
