use console::style;
use may_clack::{error::ClackSelectError, intro, multi_select, outro, select};
use std::fmt::Display;

#[derive(Debug, Clone)]
enum SelectEnum {
	One,
	Two,
	Three,
}

impl Display for SelectEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			SelectEnum::One => write!(f, "one (1)"),
			SelectEnum::Two => write!(f, "two (2)"),
			SelectEnum::Three => write!(f, "thr (3)"),
		}
	}
}

fn main() -> Result<(), ClackSelectError> {
	println!();
	intro(style(" generic select ").reverse());

	let select_string = select("select string")
		.option("val1", SelectEnum::One)
		.option("val2", SelectEnum::One)
		.option("val3", SelectEnum::One)
		.interact()?;

	let select_enum = select("select enum")
		.option(SelectEnum::One, SelectEnum::One)
		.option(SelectEnum::Two, SelectEnum::Two)
		.option(SelectEnum::Three, SelectEnum::Three)
		.interact()?;

	let multi_enum = multi_select("multi_select enum")
		.option(SelectEnum::One, "one")
		.option(SelectEnum::Two, "two")
		.option(SelectEnum::Three, "three")
		.interact()?;

	outro("");

	println!("select string, label enum {:?}", select_string);
	println!("select enum, label enum {:?}", select_enum);
	println!("multi select enum, label string {:?}", multi_enum);

	Ok(())
}
