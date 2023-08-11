use console::style;
use may_clack::{error::ClackSelectError, intro, multi_select, outro, select};

/// Activating "less" mode activates a pager.
///
/// With the value given to the `.less(val: i32)` function,
/// you can decide the amount of options per page.
fn main() -> Result<(), ClackSelectError> {
	println!();
	intro(style(" less ").reverse());

	let select_less = select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.option("val 4", "value 4")
		.option("val 5", "value 5")
		.less(3)
		.interact();

	let multi_less_noop = multi_select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.less(5)
		.interact();

	let multi_less = multi_select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.option("val 4", "value 4")
		.option("val 5", "value 5")
		.less(3)
		.interact();

	outro("");

	println!("select_less {:?}", select_less);
	println!("multi_less_noop {:?}", multi_less_noop);
	println!("multi_less {:?}", multi_less);

	Ok(())
}
