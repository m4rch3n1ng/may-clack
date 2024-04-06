/// Intro message.
///
/// Write a message to start a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`std::fmt::Display`], or nothing.
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
		println!("{}", *$crate::style::chars::BAR_START);
	};
	($arg:expr) => {
		$crate::intro!("{}", $arg);
	};
	($($arg:tt)*) => {{
		print!("{}  ", *$crate::style::chars::BAR_START);
		println!($($arg)*);
	}}
}

/// Setup outro
///
/// Write a message to start a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`std::fmt::Display`], or nothing.
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
	() => {{
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", *$crate::style::chars::BAR_END);
		println!();
	}};
	($arg:expr) => {
		$crate::outro!("{}", $arg);
	};
	($($arg:tt)*) => {{
		println!("{}", *$crate::style::chars::BAR);
		print!("{}  ", *$crate::style::chars::BAR_END);
		println!($($arg)*);
		println!();
	}};
}

/// Cancel message.
///
/// Write a message when cancelled.
///
/// Is the same as calling the [`outro!`] macro with `outro!("{}", message.red())`
///
/// # Examples
///
/// ```
/// use may_clack::cancel;
///
/// cancel!("cancel");
/// ```
#[macro_export]
macro_rules! cancel {
	($arg:expr) => {{
		use owo_colors::OwoColorize;
		$crate::outro!("{}", ($arg).red());
	}};
}

/// Info message.
///
/// Write an info message while in a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`std::fmt::Display`], or nothing.
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
	() => {{
		use owo_colors::OwoColorize;
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", (*$crate::style::chars::STEP_SUBMIT).cyan());
	}};
	($arg:expr) => {
		$crate::info!("{}", $arg);
	};
	($($arg:tt)*) => {{
		{
			use owo_colors::OwoColorize;
			println!("{}", *$crate::style::chars::BAR);
			print!("{}  ", (*$crate::style::chars::STEP_SUBMIT).cyan());
		}
		println!($($arg)*);
	}}
}

/// Warn message.
///
/// Write a warning while in a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`std::fmt::Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::{intro, outro, warn};
///
/// intro!("intro");
/// // do stuff
/// warn!("warn");
/// // do stuff
/// outro!();
/// ```
///
/// ```
/// use may_clack::warn;
///
/// // empty
/// warn!();
/// // fmt string
/// warn!("fmt {:?}", "string");
/// // impl Display
/// warn!("text");
/// warn!(4);
/// ```
#[macro_export]
macro_rules! warn {
	() => {{
		use owo_colors::OwoColorize;
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", (*$crate::style::chars::STEP_ERROR).yellow());
	}};
	($arg:expr) => {
		$crate::warn!("{}", $arg);
	};
	($($arg:tt)*) => {{
		{
			use owo_colors::OwoColorize;
			println!("{}", *$crate::style::chars::BAR);
			print!("{}  ", (*$crate::style::chars::STEP_ERROR).yellow());
		}
		println!($($arg)*);
	}};
}

/// Error message.
///
/// Write an error while in a prompt session.
///
/// Can take either a [fmt](std::fmt) string like [`format!`], a type that implements [`std::fmt::Display`], or nothing.
///
/// # Examples
///
/// ```
/// use may_clack::{intro, err, outro};
///
/// intro!("intro");
/// // do stuff
/// err!("err");
/// // do stuff
/// outro!();
/// ```
///
/// ```
/// use may_clack::err;
///
/// // empty
/// err!();
/// // fmt string
/// err!("fmt {:?}", "string");
/// // impl Display
/// err!("text");
/// err!(4);
/// ```
#[macro_export]
macro_rules! err {
	() => {{
		use owo_colors::OwoColorize;
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", (*$crate::style::chars::STEP_CANCEL).red());
	}};
	($arg:expr) => {
		$crate::err!("{}", $arg);
	};
	($($arg:tt)*) => {{
		{
			use owo_colors::OwoColorize;
			println!("{}", *$crate::style::chars::BAR);
			print!("{}  ", (*$crate::style::chars::STEP_CANCEL).red());
		}
		println!($($arg)*);
	}};
}
