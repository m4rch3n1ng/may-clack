use crossterm::style::{style, Stylize};
use may_prompt::{confirm, input, intro, outro};

// todo testing please ignore

fn main() {
	println!();
	intro(&style(" test ").reverse().to_string());

	let a = input().message("input").default_value("default").interact();
	let b = confirm().message("confirm").interact();

	outro("");

	println!();
	println!("a {:?}", a);
	println!("b {:?}", b);
}
