//! Traits

use crate::error::ClackError;

mod private {
	pub trait IsCancelSeal {}
}

/// Returns true if the operation was cancelled
///
/// For use in impl for `Result<T, ClackError>`
pub trait IsCancel: private::IsCancelSeal {
	/// Returns true if the operation was cancelled
	fn is_cancel(&self) -> bool;
}

impl<T> private::IsCancelSeal for Result<T, ClackError> {}

impl<T> IsCancel for Result<T, ClackError> {
	fn is_cancel(&self) -> bool {
		matches!(*self, Err(ClackError::Cancelled))
	}
}
