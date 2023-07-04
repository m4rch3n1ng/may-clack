use crate::style::chars;

pub fn intro(thing: &str) {
	println!("{}  {}", chars::BAR_START, thing);
}

pub fn outro(thing: &str) {
	println!("{}  {}", chars::BAR_END, thing);
	println!();
}
