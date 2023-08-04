use crate::style::chars;
use console::style;
use std::fmt::Display;

// Setup intro
pub fn intro(thing: impl Display) {
	println!("{}  {}", *chars::BAR_START, thing);
}

// Setup outro
pub fn outro(thing: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, thing);
	println!();
}

// Cancel message
pub fn cancel(thing: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, style(thing).red());
	println!();
}
