/// Error produced during compilation.
#[derive(Debug, Clone)]
pub struct Error(pub String);

pub type Result<T> = std::result::Result<T, Error>;
