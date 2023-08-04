use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClackError {
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
