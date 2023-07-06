use crossterm::style::{style, Stylize};
use may_prompt::{confirm, input, intro, multi, outro};

// todo testing please ignore

fn main() {
	println!();
	intro(&style(" test ").reverse().to_string());

	let a = input().message("input").default_value("default").interact();
	let b = confirm()
		.message("confirm")
		.prompts("true", "false")
		.interact();
	let c = multi()
		.message("multi")
		.option("opt1", "option 1")
		.option("opt2", "option 2")
		.option_hint("opt3", "option 3", "hint")
		.interact();

	outro("");

	println!("a {:?}", a);
	println!("b {:?}", b);
	println!("c {:?}", c);
}
