use may_clack::{cancel, error::ClackError, intro, multi_select, outro, select};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!(" single ".reversed());
	let do_single_select = select("single")
		.option("one", "one")
		.cancel(do_cancel)
		.interact()?;
	let do_single_multi = multi_select("single")
		.option("one", "one")
		.cancel(do_cancel)
		.interact()?;

	outro!();

	println!("select {do_single_select:?}");
	println!("multi {do_single_multi:?}");

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
