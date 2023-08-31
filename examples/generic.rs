use console::style;
use may_clack::{confirm, error::ClackError, intro, multi_input, outro};

fn main() -> Result<(), ClackError> {
	intro!("generic messages");

	let number = confirm(20).interact()?;
	let styled = multi_input(style("style").on_cyan()).interact()?;

	outro!();

	println!("number {:?}", number);
	println!("styled {:?}", styled);

	Ok(())
}
