use console::style;
use may_clack::{intro, multi_select, outro};

/// Activating "less" mode activates a pager.
///
/// With the value given to the `.less(val: i32)` function,
/// you can decide the amount of options per page.
fn main() {
	println!();
	intro(style(" less ").reverse());

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

	println!("multi_less_noop {:?}", multi_less_noop);
	println!("multi_less {:?}", multi_less);
}
