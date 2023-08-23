use is_unicode_supported::is_unicode_supported;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref IS_UNICODE: bool = is_unicode_supported();
}

fn is_unicode(unicode: &'static str, non_unicode: &'static str) -> &'static str {
	if *IS_UNICODE {
		unicode
	} else {
		non_unicode
	}
}

pub mod chars {
	use super::is_unicode;
	use lazy_static::lazy_static;

	lazy_static! {
		pub static ref BAR: &'static str = is_unicode("│", "|");
		pub static ref BAR_START: &'static str = is_unicode("┌", "T");
		pub static ref BAR_END: &'static str = is_unicode("└", "—");
		pub static ref STEP_ACTIVE: &'static str = is_unicode("◆", "*");
		pub static ref STEP_CANCEL: &'static str = is_unicode("■", "x");
		pub static ref STEP_ERROR: &'static str = is_unicode("▲", "x");
		pub static ref STEP_SUBMIT: &'static str = is_unicode("◇", "o");
		pub static ref RADIO_ACTIVE: &'static str = is_unicode("●", ">");
		pub static ref RADIO_INACTIVE: &'static str = is_unicode("○", " ");
		pub static ref CHECKBOX_ACTIVE: &'static str = is_unicode("◻", "[.]");
		pub static ref CHECKBOX_SELECTED: &'static str = is_unicode("◼", "[+]");
		pub static ref CHECKBOX_INACTIVE: &'static str = is_unicode("◻", "[ ]");
	}
}

pub mod ansi {
	pub const CLEAR_LINE: &str = "\x1b[2K";
}
