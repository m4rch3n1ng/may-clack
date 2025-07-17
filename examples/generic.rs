use may_clack::{cancel, confirm, error::ClackError, intro, multi_input, outro};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!(" generic messages ".reversed());

	let number = confirm(20).cancel(do_cancel).interact()?;
	let styled = multi_input("style".on_cyan())
		.cancel(do_cancel)
		.interact()?;

	outro!();

	println!("number {number:?}");
	println!("styled {styled:?}");

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
