use console::style;
use may_clack::{error::ClackSelectError, intro, multi_select, outro, select};

#[derive(Debug, Clone)]
enum SelectEnum {
	One,
	Two,
	Three,
}

fn main() -> Result<(), ClackSelectError> {
	println!();
	intro(style(" generic select ").reverse());

	let select_string = select("string")
		.option("val1", "value 1")
		.option("val2", "value 2")
		.option("val3", "value 3")
		.interact()?;

	let select_enum = select("enum")
		.option(SelectEnum::One, "one")
		.option(SelectEnum::Two, "two")
		.option(SelectEnum::Three, "three")
		.interact()?;

	let multi_enum = multi_select("multi_enum")
		.option(SelectEnum::One, "one")
		.option(SelectEnum::Two, "two")
		.option(SelectEnum::Three, "three")
		.interact()?;

	outro("");

	println!("string {:?}", select_string);
	println!("enum {:?}", select_enum);
	println!("multi_enum {:?}", multi_enum);

	Ok(())
}
