/// Error produced during compilation.
#[derive(Debug, Clone)]
pub struct Error(pub String);

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
