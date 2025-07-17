use may_clack::{cancel, error::ClackError, input, intro, outro};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!(" maybe_initial ".reversed());

	let opt = Some("test");

	#[allow(clippy::needless_borrows_for_generic_args)]
	let ref_opt = input("message")
		.maybe_initial(&opt)
		.cancel(do_cancel)
		.required()?;
	let opt = input("message")
		.maybe_initial(opt)
		.cancel(do_cancel)
		.required()?;
	let none = input("message")
		.maybe_initial(None::<&str>)
		.cancel(do_cancel)
		.required()?;

	outro!();

	println!("ref_opt {ref_opt:?}");
	println!("opt {opt:?}");
	println!("none {none:?}");

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
