#[derive(Debug, PartialEq)]
pub enum Error {
    None,
    NotEq(String),
}

pub type Result<T> = std::result::Result<T, Error>;
