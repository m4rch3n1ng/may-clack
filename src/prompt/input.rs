use crate::prompt::prompt::Prompt;
use console::{style, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

pub struct Input {
	message: Option<String>,
}

impl Input {
	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	// todo: Result
	pub fn interact(self) -> Option<String> {
		self.init();

		let term = Term::stdout();
		let value = term.read_line();

		let value = value.ok()?;

		self.out(&value);
		if value.len() > 0 {
			Some(value)
		} else {
			None
		}
	}
}

impl Prompt<String> for Input {

	fn init(&self) {
		let mut stdout = stdout();
		let msg = self.message.as_ref().unwrap();

		println!("│");
		println!("{}  {}", style("◆").cyan(), msg);
		println!("{}", style("│").cyan());
		print!("{}", style("└").cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}  ", style("│").cyan());
		let _ = stdout.flush();
	}

	fn out(&self, value: &String) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let msg = self.message.as_ref().unwrap();

		println!("{}  {}", style("◇").green(), msg);
		println!("{}  {}", "│", style(value).dim());
	}
}

impl Input {
	pub fn new() -> Input {
		Input { message: None }
	}

	pub fn placeholder(&self) {
		todo!()
	}

	pub fn initial_value(&self) {
		todo!()
	}
}

pub fn main() -> Input {
	Input::new()
}
