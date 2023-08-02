use crossterm::style::{style, Stylize};
use may_clack::{
	cancel, confirm::confirm, input::input, intro, multi_input::multi_input,
	multi_select::multi_select, outro, select::select,
};

// todo testing please ignore

fn main() {
	println!();
	intro(style(" test ").reverse());

	let do_input = input("input")
		.default_value("default")
		.cancel(do_cancel)
		.interact();
	let do_input_validate = input("validate")
		.validate(|x| x.is_ascii())
		.cancel(do_cancel)
		.required();
	let do_multi_input = multi_input("multi input").cancel(do_cancel).interact();
	let do_confirm = confirm("confirm").prompts("true", "false").interact();
	let do_multi_select = multi_select("multi select")
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
	println!("multi_input {:?}", do_multi_input);
	println!("confirm {:?}", do_confirm);
	println!("multi_select {:?}", do_multi_select);
	println!("select {:?}", do_select);
}

fn do_cancel() {
	cancel("demo cancelled");
	std::process::exit(1);
}
