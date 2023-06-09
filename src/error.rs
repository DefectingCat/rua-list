use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code, unused)]
pub enum CandyError {
    #[error("can not parse target")]
    Parse(String),

    #[error("page not found")]
    // NotFound(#[from] io::Error),
    NotFound,

    #[error("unknown file type, found file {file:?}")]
    UnknownFileType { file: String },

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}