use may_clack::{cancel, error::ClackError, intro, multi_select, outro, select};
use owo_colors::OwoColorize;

/// Activating "less" mode activates a pager.
///
/// With the value given to the `.less(val: i32)` function,
/// you can decide the amount of options per page.
fn main() -> Result<(), ClackError> {
	println!();
	intro!(" less ".reversed());

	let select_less = select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.option("val 4", "value 4")
		.option("val 5", "value 5")
		.less_amt(3)
		.cancel(do_cancel)
		.interact()?;

	let multi_less_noop = multi_select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.less_amt(5)
		.cancel(do_cancel)
		.interact()?;

	let multi_less = multi_select("less")
		.option("val 1", "value 1")
		.option("val 2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.option("val 4", "value 4")
		.option("val 5", "value 5")
		.less()
		.cancel(do_cancel)
		.interact()?;

	let mut page_up_down = select("page up / down");
	page_up_down.less_max(25);
	page_up_down.cancel(do_cancel);

	for i in 0..100 {
		page_up_down.option(i, i);
	}

	let page_up_down = page_up_down.interact()?;

	outro!();

	println!("page_up_down {page_up_down:?}");
	println!("select_less {select_less:?}");
	println!("multi_less_noop {multi_less_noop:?}");
	println!("multi_less {multi_less:?}");

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
