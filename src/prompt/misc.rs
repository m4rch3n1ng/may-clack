use crate::style::chars;
use console::style;
use std::fmt::Display;

pub fn intro(thing: impl Display) {
	println!("{}  {}", *chars::BAR_START, thing);
}

pub fn outro(thing: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, thing);
	println!();
}

pub fn cancel(thing: impl Display) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, style(thing).red());
	println!();
}
