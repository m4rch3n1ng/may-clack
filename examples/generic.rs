use console::style;
use may_clack::{confirm, intro, multi_input, outro, error::ClackError};

fn main() -> Result<(), ClackError> {
	intro("generic messages");

	let number = confirm(20).prompts("yes", "no").interact()?;
	let styled = multi_input(style("style").on_cyan()).interact()?;

	outro("");

	println!("number {:?}", number);
	println!("styled {:?}", styled);

	Ok(())
}
