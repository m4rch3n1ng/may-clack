use crossterm::style::{style, Stylize};
use may_clack::{confirm, input, intro, multi, outro, select};

// todo testing please ignore

fn main() {
	println!();
	intro(&style(" test ").reverse().to_string());

	let do_input = input("input").default_value("default").interact();
	let do_input_validate = input("validate").validate(|x| !x.is_empty()).interact();
	let do_confirm = confirm("confirm").prompts("true", "false").interact();
	let do_multi = multi("multi")
		.option("opt1", "option 1")
		.option("opt2", "option 2")
		.option_hint("opt3", "option 3", "hint")
		.interact();
	let do_select = select("select")
		.option("val1", "value 1")
		.option("val2", "value 2")
		.option_hint("val 3", "value 3", "hint")
		.interact();

	outro("");

	println!("input {:?}", do_input);
	println!("validate {:?}", do_input_validate);
	println!("confirm {:?}", do_confirm);
	println!("multi {:?}", do_multi);
	println!("select {:?}", do_select);
}
