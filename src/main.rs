use crossterm::style::{style, Stylize};
use may_prompt::prompt::{input, intro, outro, confirm};

// todo testing please ignore

fn main() {
	println!();
	intro(&style(" test ").reverse().to_string());

	let a = input().message("input").interact();
	let b = confirm().message("confirm").interact();
	let c = confirm().message("confirm 2").initial_value(true).interact();

	outro("");

	println!();
	println!("a {:?}", a);
	println!("b {:?}", b);
	println!("c {:?}", c);
}
