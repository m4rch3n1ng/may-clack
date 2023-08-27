use crate::style::chars;
use console::style;
use std::fmt::Display;

/// Setup intro
pub fn intro(display: impl Display) {
	println!("{}  {}", *chars::BAR_START, display);
}

/// Setup outro
pub fn outro(display: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, display);
	println!();
}

/// Cancel message
pub fn cancel(display: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, style(display).red());
	println!();
}
