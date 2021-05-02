#[derive(Debug, PartialEq)]
/// rusty-matrix specific Error enum.
pub enum Error {
    /// NotEq: Two Matrices were expected to be equal in one way or another,
    /// custom message specified within the String type.
    NotEq(String),
    /// OutOfRange: Returned when attempting to index the Matrix with an
    /// out-of-range x and y coordinates.
    OutOfRange,
    /// IncorrectLength: Returned when passing a Vec<T> as the data to construct
    /// a Matrix which is too long or short for the specified Matrix type.
    IncorrectLength,
}

/// rusty-matrix built-in Result type for use with the rusty-matrix Matrix
/// functions.
pub type Result<T> = std::result::Result<T, Error>;
