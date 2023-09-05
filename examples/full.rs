use console::style;
use may_clack::{
	cancel, confirm, error::ClackError, info, input, intro, multi_input, multi_select, outro,
	select,
};

fn main() -> Result<(), ClackError> {
	println!();
	intro!(style(" full ").reverse());

	info!("visit the documentation at https://docs.rs/may-clack");

	let do_input = input("input")
		.default_value("default")
		.cancel(do_cancel)
		.interact()?;
	let do_multi_input = multi_input("multi input")
		.max(4)
		.cancel(do_cancel)
		.interact()?;
	let do_confirm = confirm("confirm")
		.prompts("true", "false")
		.cancel(do_cancel)
		.interact()?;
	let do_multi_select = multi_select("multi select")
		.option("opt1", "option 1")
		.option("opt2", "option 2")
		.option_hint("opt3", "option 3", "hint")
		.cancel(do_cancel)
		.interact()?;
	let do_select = select("select")
		.option("val1", "value 1")
		.option("val2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.cancel(do_cancel)
		.interact()?;

	outro!();

	println!("input {:?}", do_input);
	println!("confirm {:?}", do_confirm);
	println!("multi_input {:?}", do_multi_input);
	println!("multi_select {:?}", do_multi_select);
	println!("select {:?}", do_select);

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
