use may_clack::{error::ClackError, input, intro, outro};
use owo_colors::OwoColorize;

fn main() -> Result<(), ClackError> {
	println!();
	intro!("{}", " maybe_initial ".reversed());

	let opt = Some("test");

	#[allow(clippy::needless_borrows_for_generic_args)]
	let ref_opt = input("message").maybe_initial(&opt).required()?;
	let opt = input("message").maybe_initial(opt).required()?;
	let none = input("message").maybe_initial(None::<&str>).required()?;

	outro!();

	println!("ref_opt {ref_opt:?}");
	println!("opt {opt:?}");
	println!("none {none:?}");

	Ok(())
}
