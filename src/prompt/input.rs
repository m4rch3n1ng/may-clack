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
	/// Creates a new Input struct.
	/// 
	/// Has a shorthand version in [`input()`]
	/// 
	/// ```
	/// use may_clack::{input, input::Input};
	/// 
	/// // these two are equivalent
	/// let question = Input::new("message");
	/// let question = input("message");
	/// ```
	/// 
	pub fn new<S: Into<String>>(message: S) -> Self {
		Input {
			message: message.into(),
			default_value: None,
			initial_value: None,
			validate: None,
			cancel: None,
		}
	}

	/// Specify the default value when no input is given
	/// 
	/// Useful in combination with [`Input::required()`]
	/// 
	/// ```no_run
	/// use may_clack::input;
	/// 
	/// let answer = input("message").default_value("default_value").required();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn default_value<S: Into<String>>(&mut self, def: S) -> &mut Self {
		self.default_value = Some(def.into());
		self
	}

	/// Todo
	pub fn placeholder(&mut self) -> &mut Self {
		todo!();
	}

	/// Specify the initial value
	/// 
	/// ```no_run
	/// use may_clack::input;
	/// 
	/// let answer = input("message").initial_value("initial_value").interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn initial_value<S: Into<String>>(&mut self, init: S) -> &mut Self {
		self.initial_value = Some(init.into());
		self
	}

	/// Specify a validation function
	/// 
	/// ```no_run
	/// use may_clack::input;
	/// 
	/// let answer = input("message").validate(|x| x.is_ascii()).interact();
	/// println!("answer {:?}", answer);
	/// ```
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

	/// Specify function to call on cancel
	/// 
	/// ```no_run
	/// use may_clack::{input, cancel};
	/// 
	/// let answer = input("message").cancel(do_cancel).interact();
	/// 
	/// fn do_cancel() {
	///     cancel("operation cancelled");
	///     std::process::exit(1);
	/// }
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

	/// Like [`Input::interact()`], but does not return an empty line.
	/// 
	/// Useful when used with [`Input::default_value`], as that means that there can be no empty value.
	/// 
	/// ```no_run
	/// use may_clack::input;
	/// 
	/// let answer = input("message").default_value("default_value").required();
	/// println!("answer {:?}", answer);
	/// ```
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
				if let Some(cancel) = self.cancel.as_ref() {
					cancel()
				}

				Err(ClackInputError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}

	/// Waits for the user to submit a line of text.
	/// 
	/// Returns [`Option::None`] on an empty line and [`Option::Some(String)`] otherwise.
	/// 
	/// ```no_run
	/// use may_clack::{input, cancel};
	/// 
	/// let answer = input("message")
	///     .initial_value("initial_value")
	///     .validate(|x| x.is_ascii())
	///     .cancel(do_cancel)
	///     .interact();
	/// 
	/// fn do_cancel() {
	///     cancel("operation cancelled");
	///     std::process::exit(1);
	/// }
	/// ```
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

/// Shorthand for [`Input::new()`]
pub fn input<S: Into<String>>(message: S) -> Input {
	Input::new(message)
}
