//! Error
use rustyline::error::ReadlineError;
use thiserror::Error;

/// The error type for clack errors
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ClackError {
	/// I/O error
	#[error("io error")]
	IoError(#[from] std::io::Error),
	/// Clack input cancelled
	#[error("operation cancelled")]
	Cancelled,
	/// Rustyline readline error
	#[error("readline error")]
	ReadlineError(#[from] ReadlineError),
	/// No options specified
	#[error("no options specified")]
	NoOptions,
}
