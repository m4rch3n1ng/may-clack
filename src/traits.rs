//! Traits

use crate::error::ClackError;

/// Returns true if the operation was cancelled
/// 
/// For use in impl for `Result<T, ClackError>`
pub trait IsCancel {
	fn is_cancel (&self) -> bool;
}

impl<T> IsCancel for Result<T, ClackError> {
	fn is_cancel (&self) -> bool {
		matches!(*self, Err(ClackError::Cancelled))
	}
}
