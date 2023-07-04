pub mod chars {
	pub static BAR: &str = "│"; // "|"
	pub static BAR_START: &str = "┌"; // "T"
	pub static BAR_END: &str = "└"; // "—"

	pub static STEP_ACTIVE: &str = "◆"; // "*"
	pub static STEP_CANCEL: &str = "■"; // "x"
	pub static STEP_ERROR: &str = "▲"; // "x"
	pub static STEP_SUBMIT: &str = "◇"; // "o"

	pub static RADIO_ACTIVE: &str = "●"; // ">"
	pub static RADIO_INACTIVE: &str = "○"; // " "

	pub static CHECKBOX_ACTIVE: &str = "◻"; // "[.]"
	pub static CHECKBOX_SELECTED: &str = "◼"; // "[+]"
	pub static CHECKBOX_INACTIVE: &str = "◻"; // "[ ]"
}
