use may_clack::{error::ClackError, intro, multi_select, outro, select};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!("{}", " single ".reversed());
	let do_single_select = select("single").option("one", "one").interact()?;
	let do_single_multi = multi_select("single").option("one", "one").interact()?;

	outro!();

	println!("select {do_single_select:?}");
	println!("multi {do_single_multi:?}");

	Ok(())
}
