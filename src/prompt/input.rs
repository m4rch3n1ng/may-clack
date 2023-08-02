use crate::{error::ClackInputError, style::chars};
use console::style;
use crossterm::{cursor, QueueableCommand};
use rustyline::DefaultEditor;
use std::io::{stdout, Write};

type ValidateFn = dyn Fn(&str) -> bool;

pub struct Input {
	message: String,
	default_value: Option<String>,
	initial_value: Option<String>,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
}

impl Input {
	pub fn new<S: Into<String>>(message: S) -> Self {
		Input {
			message: message.into(),
			default_value: None,
			initial_value: None,
			validate: None,
			cancel: None,
		}
	}

	pub fn default_value<S: Into<String>>(&mut self, def: S) -> &mut Self {
		self.default_value = Some(def.into());
		self
	}

	pub fn placeholder(&mut self) -> &mut Self {
		todo!();
	}

	pub fn initial_value<S: Into<String>>(&mut self, init: S) -> &mut Self {
		self.initial_value = Some(init.into());
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
					if let Some(default_value) = self.default_value.clone() {
						break Ok(Some(default_value));
					} else if enforce_non_empty {
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

	pub fn required(&self) -> Result<String, ClackInputError> {
		self.w_init();

		let interact = self.interact_once(true);
		match interact {
			Ok(Some(value)) => {
				self.w_out(&value);
				Ok(value)
			}
			Ok(None) => unreachable!(),
			Err(ClackInputError::Cancelled) => {
				self.w_cancel();
				Err(ClackInputError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}

	pub fn interact(&self) -> Result<Option<String>, ClackInputError> {
		self.w_init();

		let interact = self.interact_once(false);
		match interact {
			Ok(val) => {
				let v = val.clone().unwrap_or(String::new());
				self.w_out(&v);
				Ok(val)
			}
			Err(ClackInputError::Cancelled) => {
				self.w_cancel();
				Err(ClackInputError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}
}

impl Input {
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

	fn w_out(&self, value: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);
		println!("{}  {}", *chars::BAR, style(value).dim());
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_cancel(&self) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_CANCEL).red(), self.message);
		println!(
			"{}  {}",
			*chars::BAR,
			style("cancelled").strikethrough().dim()
		);
	}
}

pub fn input<S: Into<String>>(message: S) -> Input {
	Input::new(message)
}
