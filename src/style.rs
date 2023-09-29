//! Style utility
use is_unicode_supported::is_unicode_supported;
use once_cell::sync::Lazy;

pub static IS_UNICODE: Lazy<bool> = Lazy::new(is_unicode_supported);

fn is_unicode(unicode: &'static str, non_unicode: &'static str) -> &'static str {
	if *IS_UNICODE {
		unicode
	} else {
		non_unicode
	}
}

/// Clack prompt chars.
///
/// Changes if the terminal supports unicode.
pub mod chars {
	use super::is_unicode;
	use once_cell::sync::Lazy;

	/// Straight left bar
	pub static BAR: Lazy<&str> = Lazy::new(|| is_unicode("│", "|"));
	/// Start bar
	pub static BAR_START: Lazy<&str> = Lazy::new(|| is_unicode("┌", "T"));
	/// End bar
	pub static BAR_END: Lazy<&str> = Lazy::new(|| is_unicode("└", "—"));
	/// Active step
	pub static STEP_ACTIVE: Lazy<&str> = Lazy::new(|| is_unicode("◆", "*"));
	/// Cancelled step
	pub static STEP_CANCEL: Lazy<&str> = Lazy::new(|| is_unicode("■", "x"));
	/// Error step
	pub static STEP_ERROR: Lazy<&str> = Lazy::new(|| is_unicode("▲", "x"));
	/// Submitted step
	pub static STEP_SUBMIT: Lazy<&str> = Lazy::new(|| is_unicode("◇", "o"));
	/// Active radio
	pub static RADIO_ACTIVE: Lazy<&str> = Lazy::new(|| is_unicode("●", ">"));
	/// Inactive radio
	pub static RADIO_INACTIVE: Lazy<&str> = Lazy::new(|| is_unicode("○", " "));
	/// Active checkbox
	pub static CHECKBOX_ACTIVE: Lazy<&str> = Lazy::new(|| is_unicode("◻", "[.]"));
	/// Selected checkbox
	pub static CHECKBOX_SELECTED: Lazy<&str> = Lazy::new(|| is_unicode("◼", "[+]"));
	/// Inactive checkbox
	pub static CHECKBOX_INACTIVE: Lazy<&str> = Lazy::new(|| is_unicode("◻", "[ ]"));
}

/// ANSI escape codes
pub mod ansi {
	/// ANSI escape code to clear the line
	pub const CLEAR_LINE: &str = "\x1b[2K";
}
