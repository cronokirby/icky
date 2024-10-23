/// Errors produced during compilation.
#[derive(Debug, Clone)]
pub struct Errors {}

pub type Result<T> = std::result::Result<T, Errors>;
