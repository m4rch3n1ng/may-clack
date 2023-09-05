use console::style;
use may_clack::{cancel, confirm, error::ClackError, intro, multi_input, outro};

fn main() -> Result<(), ClackError> {
	println!();
	intro!(style(" generic messages ").reverse());

	let number = confirm(20).cancel(do_cancel).interact()?;
	let styled = multi_input(style("style").on_cyan())
		.cancel(do_cancel)
		.interact()?;

	outro!();

	println!("number {:?}", number);
	println!("styled {:?}", styled);

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
