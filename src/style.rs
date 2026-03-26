//! Style utility

use std::sync::LazyLock;
use supports_unicode::supports_unicode;

pub(crate) static IS_UNICODE: LazyLock<bool> = LazyLock::new(supports_unicode);

fn is_unicode(unicode: &'static str, non_unicode: &'static str) -> &'static str {
	if *IS_UNICODE { unicode } else { non_unicode }
}

/// Clack prompt chars.
///
/// Changes if the terminal supports unicode.
pub mod chars {
	use super::is_unicode;
	use std::sync::LazyLock;

	/// Straight left bar
	pub static BAR: LazyLock<&str> = LazyLock::new(|| is_unicode("│", "|"));
	/// Start bar
	pub static BAR_START: LazyLock<&str> = LazyLock::new(|| is_unicode("┌", "T"));
	/// End bar
	pub static BAR_END: LazyLock<&str> = LazyLock::new(|| is_unicode("└", "—"));
	/// Active step
	pub static STEP_ACTIVE: LazyLock<&str> = LazyLock::new(|| is_unicode("◆", "*"));
	/// Cancelled step
	pub static STEP_CANCEL: LazyLock<&str> = LazyLock::new(|| is_unicode("■", "x"));
	/// Error step
	pub static STEP_ERROR: LazyLock<&str> = LazyLock::new(|| is_unicode("▲", "x"));
	/// Submitted step
	pub static STEP_SUBMIT: LazyLock<&str> = LazyLock::new(|| is_unicode("◇", "o"));
	/// Active radio
	pub static RADIO_ACTIVE: LazyLock<&str> = LazyLock::new(|| is_unicode("●", ">"));
	/// Inactive radio
	pub static RADIO_INACTIVE: LazyLock<&str> = LazyLock::new(|| is_unicode("○", " "));
	/// Active checkbox
	pub static CHECKBOX_ACTIVE: LazyLock<&str> = LazyLock::new(|| is_unicode("◻", "[.]"));
	/// Selected checkbox
	pub static CHECKBOX_SELECTED: LazyLock<&str> = LazyLock::new(|| is_unicode("◼", "[+]"));
	/// Inactive checkbox
	pub static CHECKBOX_INACTIVE: LazyLock<&str> = LazyLock::new(|| is_unicode("◻", "[ ]"));
}

/// ANSI escape codes
pub mod ansi {
	/// ANSI escape code to clear the line
	pub const CLEAR_LINE: &str = "\x1b[2K";
}
