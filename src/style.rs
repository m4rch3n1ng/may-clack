//! Style utility
use is_unicode_supported::is_unicode_supported;
use lazy_static::lazy_static;

lazy_static! {
	/// Does the terminal support unicode
	pub static ref IS_UNICODE: bool = is_unicode_supported();
}

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
	use lazy_static::lazy_static;

	lazy_static! {
		/// Straight left bar
		pub static ref BAR: &'static str = is_unicode("│", "|");
		/// Start bar
		pub static ref BAR_START: &'static str = is_unicode("┌", "T");
		/// End bar
		pub static ref BAR_END: &'static str = is_unicode("└", "—");
		/// Active step
		pub static ref STEP_ACTIVE: &'static str = is_unicode("◆", "*");
		/// Cancelled step
		pub static ref STEP_CANCEL: &'static str = is_unicode("■", "x");
		/// Error step
		pub static ref STEP_ERROR: &'static str = is_unicode("▲", "x");
		/// Submitted step
		pub static ref STEP_SUBMIT: &'static str = is_unicode("◇", "o");
		/// Active radio
		pub static ref RADIO_ACTIVE: &'static str = is_unicode("●", ">");
		/// Inactive radio
		pub static ref RADIO_INACTIVE: &'static str = is_unicode("○", " ");
		/// Active checkbox
		pub static ref CHECKBOX_ACTIVE: &'static str = is_unicode("◻", "[.]");
		/// Selected checkbox
		pub static ref CHECKBOX_SELECTED: &'static str = is_unicode("◼", "[+]");
		/// Inactive checkbox
		pub static ref CHECKBOX_INACTIVE: &'static str = is_unicode("◻", "[ ]");
	}
}

/// ANSI escape codes
pub mod ansi {
	/// ANSI escape code to clear the line
	pub const CLEAR_LINE: &str = "\x1b[2K";
}
