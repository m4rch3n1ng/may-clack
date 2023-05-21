use crate::prompt::prompt::Prompt;
use console::{style, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

pub struct Input {
	message: Option<String>,
	default_value: Option<String>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			message: None,
			default_value: None,
		}
	}

	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	pub fn default_value<S: Into<String>>(mut self, def: S) -> Self {
		self.default_value = Some(def.into());
		self
	}

	pub fn placeholder(&self) {
		todo!()
	}

	pub fn initial_value(&self) {
		todo!()
	}

	// todo: Result
	pub fn interact(self) -> Option<String> {
		self.init();

		let term = Term::stdout();
		let read_line = term.read_line();

		if let Ok(value) = read_line {
			if value.len() > 0 {
				self.out(&value);
				Some(value)
			} else {
				if let Some(default_value) = self.default_value.clone() {
					self.out(&default_value);
					Some(default_value)
				} else {
					self.out(&"".into());
					None
				}
			}
		} else {
			// todo error
			self.out(&"".into());
			return None;
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

pub fn main() -> Input {
	Input::new()
}
