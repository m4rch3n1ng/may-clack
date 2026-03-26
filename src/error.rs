//! Error

use rustyline::error::ReadlineError;
use std::{error::Error, fmt::Display};

/// The error type for clack errors
#[non_exhaustive]
#[derive(Debug)]
pub enum ClackError {
	/// I/O error
	IoError(std::io::Error),
	/// Clack input cancelled
	Cancelled,
	/// Rustyline readline error
	ReadlineError(ReadlineError),
	/// No options specified
	NoOptions,
}

impl Error for ClackError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ClackError::IoError(source) => Some(source),
			ClackError::Cancelled => None,
			ClackError::ReadlineError(source) => Some(source),
			ClackError::NoOptions => None,
		}
	}
}

impl Display for ClackError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ClackError::IoError(_) => f.write_str("io error"),
			ClackError::Cancelled => f.write_str("operation cancelled"),
			ClackError::ReadlineError(_) => f.write_str("readline error"),
			ClackError::NoOptions => f.write_str("no options specified"),
		}
	}
}

impl From<std::io::Error> for ClackError {
	fn from(source: std::io::Error) -> Self {
		ClackError::IoError(source)
	}
}

impl From<ReadlineError> for ClackError {
	fn from(source: ReadlineError) -> Self {
		ClackError::ReadlineError(source)
	}
}
