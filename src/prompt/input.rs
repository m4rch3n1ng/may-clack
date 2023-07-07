use crate::style::chars;
use console::style;
use crossterm::{cursor, QueueableCommand};
use rustyline::DefaultEditor;
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
pub struct Input {
	message: String,
	default_value: Option<String>,
	initial_value: Option<String>,
}

impl Input {
	#[must_use]
	pub fn new<S: Into<String>>(message: S) -> Self {
		Input {
			message: message.into(),
			default_value: None,
			initial_value: None,
		}
	}

	#[must_use]
	pub fn default_value<S: Into<String>>(mut self, def: S) -> Self {
		self.default_value = Some(def.into());
		self
	}

	#[must_use]
	pub fn placeholder(self) -> Self {
		todo!()
	}

	#[must_use]
	pub fn initial_value<S: Into<String>>(mut self, init: S) -> Self {
		self.initial_value = Some(init.into());
		self
	}

	// todo: Result
	#[must_use]
	pub fn interact(self) -> Option<String> {
		self.init();

		let prompt = format!("{}  ", style(*chars::BAR).cyan());
		let mut editor = DefaultEditor::new().unwrap();
		let line = if let Some(init) = &self.initial_value {
			editor.readline_with_initial(&prompt, (init, ""))
		} else {
			editor.readline(&prompt)
		};

		if let Ok(value) = line {
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

impl Input {
	fn init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);
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

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);
		println!("{}  {}", *chars::BAR, style(value).dim());
	}
}

#[must_use]
pub fn prompt<S: Into<String>>(message: S) -> Input {
	Input::new(message)
}
