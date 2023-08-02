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
	#[error("readline error")]
	ReadlineError(#[from] ReadlineError),
}

#[derive(Error, Debug)]
pub enum ClackSelectError {
	#[error("io error")]
	IoError(#[from] std::io::Error),
	#[error("operation cancelled")]
	Cancelled,
	#[error("no options specified")]
	NoOptions,
}
