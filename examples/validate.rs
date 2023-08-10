use console::style;
use may_clack::{cancel, input, intro, multi_input, outro};

fn main() {
	println!();
	intro(style(" test ").reverse());

	let do_validate_input = input("validate single")
		.validate(|x| x.is_ascii())
		.cancel(do_cancel)
		.required();
	let do_validate_multi_input = multi_input("validate multi")
		.validate(|x| x.is_ascii())
		.cancel(do_cancel)
		.interact();

	outro("");

	println!("single {:?}", do_validate_input);
	println!("multi {:?}", do_validate_multi_input);
}

fn do_cancel() {
	cancel("demo cancelled");
	std::process::exit(1);
}
