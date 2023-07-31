use std::io::{stdout, Write};

use console::style;
use crossterm::{cursor, QueueableCommand};
use rustyline::DefaultEditor;

use crate::style::chars;

type ValidateFn = dyn Fn(&str) -> bool;

pub struct MultiInput {
	message: String,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
	initial_value: Option<String>,
	min: u16,
	max: u16,
}

// todo rm derive
#[derive(Debug)]
enum InteractOnce {
	Cancel,
	Value(Option<String>),
}

impl MultiInput {
	#[must_use]
	pub fn new<S: Into<String>>(message: S) -> Self {
		MultiInput {
			message: message.into(),
			validate: None,
			initial_value: None,
			cancel: None,
			min: 1,
			max: u16::MAX,
		}
	}

	#[must_use]
	pub fn placeholder(self) -> Self {
		todo!();
	}

	#[must_use]
	pub fn initial_value<S: Into<String>>(mut self, initial_value: S) -> Self {
		self.initial_value = Some(initial_value.into());
		self
	}

	#[must_use]
	pub fn min(mut self, min: u16) -> Self {
		self.min = min;
		self
	}

	#[must_use]
	pub fn max(mut self, max: u16) -> Self {
		self.max = max.into();
		self
	}

	#[must_use]
	pub fn validate<F>(mut self, validate: F) -> Self
	where
		F: Fn(&str) -> bool + 'static,
	{
		let validate = Box::new(validate);
		self.validate = Some(validate);
		self
	}

	fn do_validate(&self, input: &str) -> bool {
		if let Some(validate) = self.validate.as_deref() {
			validate(input)
		} else {
			true
		}
	}

	#[must_use]
	pub fn cancel<F>(mut self, cancel: F) -> Self
	where
		F: Fn() + 'static,
	{
		let cancel = Box::new(cancel);
		self.cancel = Some(cancel);
		self
	}

	fn interact_once(&self, enforce_non_empty: bool) -> InteractOnce {
		let prompt = format!("{}  ", style(*chars::BAR).cyan());
		let mut editor = DefaultEditor::new().unwrap();

		loop {
			let line = if let Some(init) = &self.initial_value {
				editor.readline_with_initial(&prompt, (init, ""))
			} else {
				editor.readline(&prompt)
			};

			if let Ok(value) = line {
				if value.is_empty() {
					if enforce_non_empty {
						let mut stdout = stdout();
						let _ = stdout.queue(cursor::MoveToPreviousLine(1));
						let _ = stdout.flush();
					} else {
						break InteractOnce::Value(None);
					}
				} else {
					if self.do_validate(&value) {
						break InteractOnce::Value(Some(value));
					} else {
						let mut stdout = stdout();
						let _ = stdout.queue(cursor::MoveToPreviousLine(1));
						let _ = stdout.flush();
					}
				}
			} else {
				break InteractOnce::Cancel;
			}
		}
	}

	// todo max
	// todo validate
	pub fn interact(self) -> Option<Vec<String>> {
		self.w_init();

		let mut v = vec![];

		loop {
			let enforce_non_empty = (v.len() as u16) < self.min;
			let once = self.interact_once(enforce_non_empty);

			match once {
				InteractOnce::Cancel => {
					self.w_cancel(v.len());
					if let Some(cancel) = self.cancel {
						cancel();
					}

					return None;
				}
				InteractOnce::Value(Some(value)) => {
					self.w_line(&value);
					v.push(value);
				}
				InteractOnce::Value(None) => {
					self.w_out(v.len());
					break;
				}
			}
		}

		Some(v)
	}
}

impl MultiInput {
	fn w_init(&self) {
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

	fn w_line(&self, value: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::BAR).cyan(), style(value).dim());
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}  ", style(*chars::BAR).cyan());
		let _ = stdout.flush();
	}

	fn w_out(&self, amt: usize) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		for _ in 0..amt {
			println!("{}", *chars::BAR);
		}
	}

	fn w_cancel(&self, amt: usize) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!(
			"{}  {}",
			*chars::BAR,
			style("cancelled").strikethrough().dim()
		);

		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_CANCEL).red(), self.message);

		for _ in 0..amt {
			println!("{}", *chars::BAR);
		}

		let _ = stdout.queue(cursor::MoveToNextLine(1));
		let _ = stdout.flush();
	}
}

pub fn multi_input<S: Into<String>>(message: S) -> MultiInput {
	MultiInput::new(message)
}
