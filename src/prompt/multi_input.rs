use crate::{
	error::ClackError,
	style::{ansi, chars},
};
use console::style;
use crossterm::{cursor, QueueableCommand};
use rustyline::DefaultEditor;
use std::{
	fmt::Display,
	io::{stdout, Write},
};

type ValidateFn = dyn Fn(&str) -> Option<&'static str>;

pub struct MultiInput<M: Display> {
	message: M,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
	initial_value: Option<String>,
	min: u16,
	max: u16,
}

impl<M: Display> MultiInput<M> {
	/// Creates a new `MultiInput` struct.
	///
	/// Has a shorthand version in [`multi_input()`]
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{multi_input, multi_input::MultiInput};
	///
	/// // these two are equivalent
	/// let question = MultiInput::new("message");
	/// let question = multi_input("message");
	/// ```
	pub fn new(message: M) -> Self {
		MultiInput {
			message,
			validate: None,
			initial_value: None,
			cancel: None,
			min: 1,
			max: u16::MAX,
		}
	}

	/// Todo
	pub fn placeholder(&mut self) -> &mut Self {
		todo!();
	}

	/// Specify the initial value.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// let answers = multi_input("message").initial_value("initial_value").interact();
	/// println!("answers {:?}", answers);
	/// ```
	pub fn initial_value<S: Into<String>>(&mut self, initial_value: S) -> &mut Self {
		self.initial_value = Some(initial_value.into());
		self
	}

	/// Specify the minimum amount of answers.
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// let answers = multi_input("message").min(2).interact();
	/// println!("answers {:?}", answers)
	/// ```
	pub fn min(&mut self, min: u16) -> &mut Self {
		self.min = min;
		self
	}

	/// Specify the maximum amount of answers.
	/// Will automatically submit when that amount is reached.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// let answers = multi_input("message").max(4).interact();
	/// println!("answers {:?}", answers);
	/// ```
	pub fn max(&mut self, max: u16) -> &mut Self {
		self.max = max;
		self
	}

	/// Specify a validation function.
	///
	/// On a successful validation, return a `None` from the closure,
	/// and on an unsuccessful validation return a `Some<&'static str>` with the error message.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// let answers = multi_input("message")
	///     .validate(|x| (!x.is_ascii()).then_some("only use ascii characters"))
	///     .interact();
	/// println!("answers {:?}", answers);
	/// ```
	pub fn validate<F>(&mut self, validate: F) -> &mut Self
	where
		F: Fn(&str) -> Option<&'static str> + 'static,
	{
		let validate = Box::new(validate);
		self.validate = Some(validate);
		self
	}

	fn do_validate(&self, input: &str) -> Option<&'static str> {
		if let Some(validate) = self.validate.as_deref() {
			validate(input)
		} else {
			None
		}
	}

	/// Specify function to call on cancel.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{multi_input, cancel};
	///
	/// let answers = multi_input("message").cancel(do_cancel).interact();
	/// println!("answers {:?}", answers);
	///
	/// fn do_cancel() {
	///     cancel("operation cancelled");
	///     panic!("operation cancelled");
	/// }
	pub fn cancel<F>(&mut self, cancel: F) -> &mut Self
	where
		F: Fn() + 'static,
	{
		let cancel = Box::new(cancel);
		self.cancel = Some(cancel);
		self
	}

	fn interact_once(
		&self,
		enforce_non_empty: bool,
		amt: u16,
	) -> Result<Option<String>, ClackError> {
		let default_prompt = format!("{}  ", style(*chars::BAR).cyan());
		let val_prompt = format!("{}  ", style(*chars::BAR).yellow());
		let mut editor = DefaultEditor::new()?;

		let mut initial_value = self.initial_value.clone();
		let mut is_val = false;

		loop {
			let prompt = if is_val { &val_prompt } else { &default_prompt };

			let line = if let Some(ref init) = initial_value {
				editor.readline_with_initial(prompt, (init, ""))
			} else {
				editor.readline(prompt)
			};

			// todo this looks refactor-able
			if let Ok(value) = line {
				if value.is_empty() {
					if enforce_non_empty {
						initial_value = None;

						is_val = true;

						let text = format!("minimum {}", self.min);
						self.w_val(&text, amt);
					} else {
						break Ok(None);
					}
				} else if let Some(text) = self.do_validate(&value) {
					initial_value = Some(value);

					is_val = true;
					self.w_val(text, amt);
				} else {
					break Ok(Some(value));
				}
			} else {
				break Err(ClackError::Cancelled);
			}
		}
	}

	/// Waits for the user to submit a line of text.
	///
	/// Returns [`None`] on an empty line and [`Some::<String>`] otherwise.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{multi_input, cancel};
	///
	/// let answers = multi_input("message")
	///     .validate(|x| x.parse::<u32>().err().map(|_| "invalid u32"))
	///     .cancel(do_cancel)
	///     .interact();
	/// println!("answers {:?}", answers);
	///
	/// fn do_cancel() {
	///     cancel("operation cancelled");
	///     std::process::exit(1);
	/// }
	/// ```
	pub fn interact(&self) -> Result<Vec<String>, ClackError> {
		self.w_init();

		let mut v = vec![];
		loop {
			let amt = v.len() as u16;

			let enforce_non_empty = amt < self.min;
			let once = self.interact_once(enforce_non_empty, amt);

			match once {
				Ok(Some(value)) => {
					self.w_line(&value, amt);
					v.push(value);

					if v.len() as u16 == self.max {
						println!();
						self.w_out(&v);
						break;
					}
				}
				Ok(None) => {
					self.w_out(&v);
					break;
				}
				Err(ClackError::Cancelled) => {
					self.w_cancel(v.len());
					if let Some(cancel) = self.cancel.as_deref() {
						cancel();
					}

					return Err(ClackError::Cancelled);
				}
				Err(err) => return Err(err),
			}
		}

		Ok(v)
	}
}

impl<M: Display> MultiInput<M> {
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

	fn w_line(&self, value: &str, amt: u16) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt + 2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		for _ in 0..amt {
			println!("{}", style(*chars::BAR).cyan());
		}

		println!("{}  {}", style(*chars::BAR).cyan(), style(value).dim());
		println!("{}", style(*chars::BAR).cyan());

		print!("{}", ansi::CLEAR_LINE);
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_val(&self, text: &str, amt: u16) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt + 2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_ERROR).yellow(), self.message);

		for _ in 0..=amt {
			println!("{}", style(*chars::BAR).yellow());
		}

		print!("{}", ansi::CLEAR_LINE);
		print!(
			"{}  {}",
			style(*chars::BAR_END).yellow(),
			style(text).yellow()
		);

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_out(&self, values: &[String]) {
		let amt = values.len();

		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		if amt == 0 {
			println!("{}", *chars::BAR);
		}

		for val in values {
			println!("{}  {}", *chars::BAR, style(val).dim());
		}

		println!("{}", style(ansi::CLEAR_LINE));
		println!("{}", style(ansi::CLEAR_LINE));

		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();
	}

	fn w_cancel(&self, amt: usize) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}", style(ansi::CLEAR_LINE));
		println!(
			"{}  {}",
			*chars::BAR,
			style("cancelled").strikethrough().dim()
		);

		print!("{}", style(ansi::CLEAR_LINE));

		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 2));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_CANCEL).red(), self.message);

		for _ in 0..amt {
			println!("{}", *chars::BAR);
		}

		let _ = stdout.queue(cursor::MoveToNextLine(1));
		let _ = stdout.flush();
	}
}

/// Shorthand for [`MultiInput::new()`]
pub fn multi_input<M: Display>(message: M) -> MultiInput<M> {
	MultiInput::new(message)
}
