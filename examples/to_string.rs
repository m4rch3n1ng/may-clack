use may_clack::{error::ClackError, input, intro, outro};
use owo_colors::OwoColorize;
use std::fmt::Display;

struct Name;

impl Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "display")
	}
}

fn main() -> Result<(), ClackError> {
	println!();
	intro!(" to_string ".reversed());

	let int = input("int").initial_value(23).parse::<i32>()?;
	let unit = input("struct").placeholder(Name).required()?;

	outro!();

	println!("int {int:?}");
	println!("unit {unit:?}");

	Ok(())
}
