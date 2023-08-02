use std::io::{stdout, Write};

use console::style;
use crossterm::{cursor, QueueableCommand};
use rustyline::DefaultEditor;

use crate::{error::ClackInputError, style::chars};

type ValidateFn = dyn Fn(&str) -> bool;

pub struct MultiInput {
	message: String,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
	initial_value: Option<String>,
	min: u16,
	max: u16,
}

impl MultiInput {
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

	pub fn placeholder(&mut self) -> &mut Self {
		todo!();
	}

	pub fn initial_value<S: Into<String>>(&mut self, initial_value: S) -> &mut Self {
		self.initial_value = Some(initial_value.into());
		self
	}

	pub fn min(&mut self, min: u16) -> &mut Self {
		self.min = min;
		self
	}

	pub fn max(&mut self, max: u16) -> &mut Self {
		self.max = max;
		self
	}

	pub fn validate<F>(&mut self, validate: F) -> &mut Self
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

	pub fn cancel<F>(&mut self, cancel: F) -> &mut Self
	where
		F: Fn() + 'static,
	{
		let cancel = Box::new(cancel);
		self.cancel = Some(cancel);
		self
	}

	fn interact_once(&self, enforce_non_empty: bool) -> Result<Option<String>, ClackInputError> {
		let prompt = format!("{}  ", style(*chars::BAR).cyan());
		let mut editor = DefaultEditor::new()?;

		let mut initial_value = self.initial_value.clone();
		loop {
			let line = if let Some(ref init) = initial_value {
				editor.readline_with_initial(&prompt, (init, ""))
			} else {
				editor.readline(&prompt)
			};

			// todo this looks refactor-able
			if let Ok(value) = line {
				if value.is_empty() {
					if enforce_non_empty {
						initial_value = None;
						let mut stdout = stdout();
						let _ = stdout.queue(cursor::MoveToPreviousLine(1));
						let _ = stdout.flush();
					} else {
						break Ok(None);
					}
				} else if self.do_validate(&value) {
					break Ok(Some(value));
				} else {
					initial_value = Some(value);
					let mut stdout = stdout();
					let _ = stdout.queue(cursor::MoveToPreviousLine(1));
					let _ = stdout.flush();
				}
			} else {
				break Err(ClackInputError::Cancelled);
			}
		}
	}

	// todo max
	pub fn interact(&self) -> Result<Vec<String>, ClackInputError> {
		self.w_init();

		let mut v = vec![];
		loop {
			let enforce_non_empty = (v.len() as u16) < self.min;
			let once = self.interact_once(enforce_non_empty);

			match once {
				Ok(Some(value)) => {
					self.w_line(&value);
					v.push(value);
				}
				Ok(None) => {
					self.w_out(v.len());
					break;
				}
				Err(ClackInputError::Cancelled) => {
					self.w_cancel(v.len());
					if let Some(cancel) = self.cancel.as_ref() {
						cancel();
					}

					return Err(ClackInputError::Cancelled);
				}
				Err(err) => return Err(err),
			}
		}

		Ok(v)
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
