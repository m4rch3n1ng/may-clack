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
/// ```
#[macro_export]
macro_rules! intro {
	() => {
		println!("{}", *$crate::style::chars::BAR_START);
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
/// ```
#[macro_export]
macro_rules! outro {
	() => {{
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", *$crate::style::chars::BAR_END);
		println!();
	}};
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
		$crate::outro!("{}", $crate::owo_colors::OwoColorize::red(&$arg));
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
/// ```
#[macro_export]
macro_rules! info {
	() => {{
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", $crate::owo_colors::OwoColorize::cyan(&*$crate::style::chars::STEP_SUBMIT));
	}};
	($($arg:tt)*) => {{
		println!("{}", *$crate::style::chars::BAR);
		print!("{}  ", $crate::owo_colors::OwoColorize::cyan(&*$crate::style::chars::STEP_SUBMIT));
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
/// ```
#[macro_export]
macro_rules! warn {
	() => {{
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", $crate::owo_colors::OwoColorize::yellow(&*$crate::style::chars::STEP_ERROR));
	}};
	($($arg:tt)*) => {{
		println!("{}", *$crate::style::chars::BAR);
		print!("{}  ", $crate::owo_colors::OwoColorize::yellow(&*$crate::style::chars::STEP_ERROR));
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
/// use may_clack::{err, intro, outro};
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
/// ```
#[macro_export]
macro_rules! err {
	() => {{
		println!("{}", *$crate::style::chars::BAR);
		println!("{}", $crate::owo_colors::OwoColorize::red(&*$crate::style::chars::STEP_CANCEL));
	}};
	($($arg:tt)*) => {{
		println!("{}", *$crate::style::chars::BAR);
		print!("{}  ", $crate::owo_colors::OwoColorize::red(&*$crate::style::chars::STEP_CANCEL));
		println!($($arg)*);
	}};
}
