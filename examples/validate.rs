use may_clack::{cancel, error::ClackError, input, intro, multi_input, outro};
use owo_colors::OwoColorize;
use std::{borrow::Cow, net::Ipv4Addr};

fn main() -> Result<(), ClackError> {
	println!();
	intro!(" validate ".reversed());

	let do_validate_input = input("validate single (only use ascii)")
		.validate(|x| {
			if x.is_ascii() {
				Ok(())
			} else {
				Err(Cow::Borrowed("only use ascii characters"))
			}
		})
		.cancel(do_cancel)
		.required()?;
	let do_validate_multi_input = multi_input("validate multi (only use lowercase)")
		.validate(|x| {
			x.find(char::is_uppercase)
				.map(|_| "only use lowercase characters")
		})
		.cancel(do_cancel)
		.interact()?;
	let do_parse_input = input("parse to u8").cancel(do_cancel).parse::<u8>()?;
	let do_maybe_parse = input("maybe parse to ipv4 addr").maybe_parse::<Ipv4Addr>()?;
	let do_parse_multi = multi_input("parse multiple to u8")
		.cancel(do_cancel)
		.parse::<u8>()?;

	outro!();

	println!("single {:?}", do_validate_input);
	println!("multi {:?}", do_validate_multi_input);
	println!("parse single {:?}", do_parse_input);
	println!("maybe parse single {:?}", do_maybe_parse);
	println!("parse multi {:?}", do_parse_multi);

	Ok(())
}

fn do_cancel() {
	cancel!("demo cancelled");
	panic!("demo cancelled");
}
