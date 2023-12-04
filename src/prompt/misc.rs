use crate::style::chars;
use owo_colors::OwoColorize;
use std::fmt::Display;

#[doc(hidden)]
pub fn wr_intro<D: Display>(display: D) {
	println!("{}  {}", *chars::BAR_START, display);
}

#[doc(hidden)]
pub fn wr_outro<D: Display>(display: D) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, display);
	println!();
}

#[doc(hidden)]
pub fn wr_cancel<D: Display>(display: D) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, display.red());
	println!();
}

#[doc(hidden)]
pub fn wr_info<D: Display>(display: D) {
	println!("{}", *chars::BAR);
	println!("{}  {}", (*chars::STEP_SUBMIT).cyan(), display);
}

/// Intro message.
///
/// Write a message to start a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::intro;
///
/// // empty
/// intro!();
/// // fmt string
/// intro!("fmt {:?}", "string");
/// // impl Display
/// intro!("text");
/// intro!(4);
/// ```
#[macro_export]
macro_rules! intro {
	() => {
		$crate::misc::wr_intro("");
	};
	($arg:expr) => {
		$crate::misc::wr_intro($arg);
	};
	($($arg:tt)*) => {
		let text = format!($($arg)*);
		$crate::misc::wr_intro(text);
	}
}

/// Setup outro
///
/// Write a message to start a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::outro;
///
/// // empty
/// outro!();
/// // fmt string
/// outro!("fmt {:?}", "string");
/// // impl Display
/// outro!("text");
/// outro!(4);
/// ```
#[macro_export]
macro_rules! outro {
	() => {
		$crate::misc::wr_outro("");
	};
	($arg:expr) => {
		$crate::misc::wr_outro($arg);
	};
	($($arg:tt)*) => {
		let text = format!($($arg)*);
		$crate::misc::wr_outro(text);
	};
}

/// Cancel message.
///
/// Write a message when cancelled.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::cancel;
///
/// // empty
/// cancel!();
/// // fmt string
/// cancel!("fmt {:?}", "string");
/// // impl Display
/// cancel!("text");
/// cancel!(4);
/// ```
#[macro_export]
macro_rules! cancel {
	() => {
		$crate::misc::wr_cancel("");
	};
	($arg:expr) => {
		$crate::misc::wr_cancel($arg);
	};
	($($arg:tt)*) => {
		let text = format!($($arg)*);
		$crate::misc::wr_cancel(text);
	}
}

/// Info message.
///
/// Write an info message while in a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::{info, intro, outro};
///
/// intro!("intro");
/// // do stuff
/// info!("info");
/// // do stuff
/// outro!();
/// ```
///
/// ```
/// use may_clack::info;
///
/// // empty
/// info!();
/// // fmt string
/// info!("fmt {:?}", "string");
/// // impl Display
/// info!("text");
/// info!(4);
/// ```
#[macro_export]
macro_rules! info {
	() => {
		$crate::misc::wr_info("");
	};
	($arg:expr) => {
		$crate::misc::wr_info($arg);
	};
	($($arg:tt)*) => {
		let text = format!($($arg)*);
		$crate::misc::wr_info(text);
	}
}
