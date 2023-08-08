use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClackError {
	#[error("{0}")]
	ClackSimpleError(#[from] ClackSimpleError),
	#[error("{0}")]
	ClackInputError(#[from] ClackInputError),
	#[error("{0}")]
	ClackSelectError(#[from] ClackSelectError),
}

#[derive(Error, Debug)]
pub enum ClackSimpleError {
	#[error("io error")]
	IoError(#[from] std::io::Error),
	#[error("operation cancelled")]
	Cancelled,
}

#[derive(Error, Debug)]
pub enum ClackInputError {
	#[error("operation cancelled")]
	Cancelled,
	#[error("readline error {0}")]
	ReadlineError(#[from] ReadlineError),
}

#[derive(Error, Debug)]
pub enum ClackSelectError {
	#[error("io error {0}")]
	IoError(#[from] std::io::Error),
	#[error("operation cancelled")]
	Cancelled,
	#[error("no options specified")]
	NoOptions,
}
