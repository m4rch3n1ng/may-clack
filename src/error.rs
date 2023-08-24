use rustyline::error::ReadlineError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ClackError {
	#[error("io error")]
	IoError(#[from] std::io::Error),
	#[error("operation cancelled")]
	Cancelled,
	#[error("readline error")]
	ReadlineError(#[from] ReadlineError),
	#[error("no options specified")]
	NoOptions,
}
