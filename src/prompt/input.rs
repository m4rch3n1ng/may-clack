use super::traits::Prompt;
use crate::style::chars;
use console::{style, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

pub struct Input {
	message: Option<String>,
	default_value: Option<String>,
}

impl Default for Input {
	fn default() -> Self {
		Self::new()
	}
}

impl Input {
	#[must_use]
	pub fn new() -> Input {
		Input {
			message: None,
			default_value: None,
		}
	}

	#[must_use]
	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	#[must_use]
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
	#[must_use]
	pub fn interact(self) -> Option<String> {
		self.init();

		let term = Term::stdout();
		let read_line = term.read_line();

		if let Ok(value) = read_line {
			if !value.is_empty() {
				self.out(&value);
				Some(value)
			} else if let Some(default_value) = self.default_value.clone() {
				self.out(&default_value);
				Some(default_value)
			} else {
				self.out("");
				None
			}
		} else {
			// todo error
			self.out("");
			None
		}
	}
}

impl Prompt<&str> for Input {
	fn init(&self) {
		let mut stdout = stdout();
		let msg = self.message.as_ref().unwrap();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), msg);
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}  ", style(*chars::BAR).cyan());
		let _ = stdout.flush();
	}

	fn out(&self, value: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let msg = self.message.as_ref().unwrap();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), msg);
		println!("{}  {}", *chars::BAR, style(value).dim());
	}
}

#[must_use]
pub fn prompt() -> Input {
	Input::new()
}
