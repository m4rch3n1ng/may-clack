use may_clack::{confirm, error::ClackError, intro, multi_input, outro};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!("{}", " generic messages ".reversed());

	let number = confirm(20).interact()?;
	let styled = multi_input("style".on_cyan()).interact()?;

	outro!();

	println!("number {number:?}");
	println!("styled {styled:?}");

	Ok(())
}
