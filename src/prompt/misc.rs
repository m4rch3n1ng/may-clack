use crate::style::chars;
use console::style;

pub fn intro(thing: &str) {
	println!("{}  {}", *chars::BAR_START, thing);
}

pub fn outro(thing: &str) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, thing);
	println!();
}

pub fn cancel(thing: &str) {
	println!("{}", *chars::BAR);
	println!("{}  {}", *chars::BAR_END, style(thing).red());
	println!();
}
