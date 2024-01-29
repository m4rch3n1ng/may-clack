//! Multiple text inputs

use super::input::PlaceholderHightlighter;
use crate::{
	error::ClackError,
	style::{ansi, chars},
};
use crossterm::{cursor, QueueableCommand};
use owo_colors::OwoColorize;
use rustyline::Editor;
use std::{
	borrow::Cow,
	error::Error,
	fmt::Display,
	io::{stdout, Write},
	str::FromStr,
};

type ValidateFn = dyn Fn(&str) -> Option<&'static str>;

/// `MultiInput` struct
///
/// # Examples
///
/// ```no_run
/// use may_clack::{multi_input, cancel};
///
/// # fn main() -> Result<(), may_clack::error::ClackError> {
/// let answers = multi_input("message")
///     .validate(|x| (!x.is_ascii()).then_some("only use ascii characters"))
///     .cancel(do_cancel)
///     .interact()?;
/// println!("answers {:?}", answers);
/// # Ok(())
/// # }
///
/// fn do_cancel() {
///     cancel!("operation cancelled");
///     std::process::exit(1);
/// }
/// ```
pub struct MultiInput<M: Display> {
	message: M,
	initial_value: Option<String>,
	placeholder: Option<String>,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
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
			placeholder: None,
			cancel: None,
			min: 1,
			max: u16::MAX,
		}
	}

	/// Specify the initial value.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message").initial_value("initial_value").interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
	/// ```
	pub fn initial_value<S: ToString>(&mut self, initial_value: S) -> &mut Self {
		self.initial_value = Some(initial_value.to_string());
		self
	}

	/// Specify a placeholder.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message").placeholder("placeholder").interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
	/// ```
	pub fn placeholder<S: ToString>(&mut self, placeholder: S) -> &mut Self {
		self.placeholder = Some(placeholder.to_string());
		self
	}

	/// Specify the minimum amount of answers.
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message").min(2).interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
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
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message").max(4).interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
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
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message")
	///     .validate(|x| x.find(char::is_whitespace).map(|_| "whitespace is disallowed"))
	///     .interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
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
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message").cancel(do_cancel).interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
	///
	/// fn do_cancel() {
	///     cancel!("operation cancelled");
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

	fn interact_once<T: FromStr>(
		&self,
		enforce_non_empty: bool,
		amt: u16,
	) -> Result<Option<T>, ClackError>
	where
		T::Err: Error,
	{
		let prompt = format!("{}  ", *chars::BAR);
		let mut editor = Editor::new()?;

		let highlighter = PlaceholderHightlighter::new(self.placeholder.as_deref());
		editor.set_helper(Some(highlighter));

		let mut initial_value = self.initial_value.as_deref().map(Cow::Borrowed);
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

						if let Some(helper) = editor.helper_mut() {
							helper.is_val = true;
						}

						let text = format!("minimum {}", self.min);
						self.w_val(&text, amt);
					} else {
						break Ok(None);
					}
				} else if let Some(text) = self.do_validate(&value) {
					initial_value = Some(Cow::Owned(value));

					if let Some(helper) = editor.helper_mut() {
						helper.is_val = true;
					}

					self.w_val(text, amt);
				} else {
					match value.parse::<T>() {
						Ok(value) => break Ok(Some(value)),
						Err(err) => {
							initial_value = Some(Cow::Owned(value));

							if let Some(helper) = editor.helper_mut() {
								helper.is_val = true;
							}

							self.w_val(&err.to_string(), amt);
						}
					}
					// break Ok(Some(value));
				}
			} else {
				break Err(ClackError::Cancelled);
			}
		}
	}

	/// Like [`MultiInput::interact()`], but parses the value before returning.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_input;
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers: Vec<i32> = multi_input("message")
	///     .min(2)
	///     .parse::<i32>()?;
	/// println!("answers {:?}", answers);
	///
	/// # Ok(())
	/// # }
	/// ```
	pub fn parse<T: FromStr + Display>(&self) -> Result<Vec<T>, ClackError>
	where
		T::Err: Error,
	{
		self.w_init();

		let mut v = vec![];
		loop {
			let amt = v.len() as u16;

			let enforce_non_empty = amt < self.min;
			let once = self.interact_once::<T>(enforce_non_empty, amt);

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

	/// Waits for the user to submit a line of text.
	///
	/// Returns [`None`] on an empty line and [`Some::<String>`] otherwise.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{multi_input, cancel};
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answers = multi_input("message")
	///     .validate(|x| x.contains('\0').then_some("contains nul byte"))
	///     .cancel(do_cancel)
	///     .interact()?;
	/// println!("answers {:?}", answers);
	/// # Ok(())
	/// # }
	///
	/// fn do_cancel() {
	///     cancel!("operation cancelled");
	///     std::process::exit(1);
	/// }
	/// ```
	pub fn interact(&self) -> Result<Vec<String>, ClackError> {
		self.w_init();

		let mut v = vec![];
		loop {
			let amt = v.len() as u16;

			let enforce_non_empty = amt < self.min;
			let once = self.interact_once::<String>(enforce_non_empty, amt);

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
		println!("{}  {}", (*chars::STEP_ACTIVE).cyan(), self.message);
		println!("{}", (*chars::BAR).cyan());
		print!("{}", (*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}  ", (*chars::BAR).cyan());
		let _ = stdout.flush();
	}

	fn w_line<V: Display>(&self, value: V, amt: u16) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt + 2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_ACTIVE).cyan(), self.message);

		for _ in 0..amt {
			println!("{}", (*chars::BAR).cyan());
		}

		println!("{}  {}", (*chars::BAR).cyan(), value.dimmed());
		println!("{}", (*chars::BAR).cyan());

		print!("{}", ansi::CLEAR_LINE);
		print!("{}", (*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_val(&self, text: &str, amt: u16) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt + 2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_ERROR).yellow(), self.message);

		for _ in 0..=amt {
			println!("{}", (*chars::BAR).yellow());
		}

		print!("{}", ansi::CLEAR_LINE);
		print!("{}  {}", (*chars::BAR_END).yellow(), text.yellow());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_out<V: Display>(&self, values: &[V]) {
		let amt = values.len();

		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_SUBMIT).green(), self.message);

		if amt == 0 {
			println!("{}", *chars::BAR);
		}

		for val in values {
			println!("{}  {}", *chars::BAR, val.dimmed());
		}

		println!("{}", ansi::CLEAR_LINE);
		println!("{}", ansi::CLEAR_LINE);

		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();
	}

	fn w_cancel(&self, amt: usize) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, "cancelled".strikethrough().dimmed());

		print!("{}", ansi::CLEAR_LINE);

		let _ = stdout.queue(cursor::MoveToPreviousLine(amt as u16 + 2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_CANCEL).red(), self.message);

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
