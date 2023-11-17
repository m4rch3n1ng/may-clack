//! Text input
use crate::{
	error::ClackError,
	style::{ansi, chars},
};
use crossterm::{cursor, QueueableCommand};
use owo_colors::OwoColorize;
use rustyline::{highlight::Highlighter, Completer, Editor, Helper, Hinter, Validator};
use std::{
	borrow::Cow,
	fmt::Display,
	io::{stdout, Write},
	str::FromStr,
};

#[derive(Completer, Helper, Hinter, Validator)]
pub(super) struct PlaceholderHightlighter<'a> {
	placeholder: Option<&'a str>,
	pub is_val: bool,
}

impl<'a> PlaceholderHightlighter<'a> {
	pub fn new(placeholder: Option<&'a str>) -> Self {
		PlaceholderHightlighter {
			placeholder,
			is_val: false,
		}
	}
}

impl Highlighter for PlaceholderHightlighter<'_> {
	fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
		if let Some(placeholder) = self.placeholder {
			if line.is_empty() {
				Cow::Owned(placeholder.dimmed().to_string())
			} else {
				Cow::Borrowed(line)
			}
		} else {
			Cow::Borrowed(line)
		}
	}

	fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
		true
	}

	fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
		&'s self,
		prompt: &'p str,
		default: bool,
	) -> Cow<'b, str> {
		if !default {
			Cow::Owned(format!("aa {}", prompt))
		} else if self.is_val {
			Cow::Owned(prompt.yellow().to_string())
		} else {
			Cow::Owned(prompt.cyan().to_string())
		}
	}
}

type ValidateFn = dyn Fn(&str) -> Option<&'static str>;

/// `Input` struct
///
/// # Examples
///
/// ```no_run
/// use may_clack::{input, cancel};
///
/// let answer = input("message")
///     .initial_value("initial_value")
///     .validate(|x| x.parse::<u32>().err().map(|_| "invalid u32"))
///     .cancel(do_cancel)
///     .interact();
/// println!("answer {:?}", answer);
///
/// fn do_cancel() {
///     cancel!("operation cancelled");
///     std::process::exit(1);
/// }
pub struct Input<M: Display> {
	message: M,
	initial_value: Option<String>,
	placeholder: Option<String>,
	validate: Option<Box<ValidateFn>>,
	cancel: Option<Box<dyn Fn()>>,
}

impl<M: Display> Input<M> {
	/// Creates a new `Input` struct.
	///
	/// Has a shorthand version in [`input()`]
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{input, input::Input};
	///
	/// // these two are equivalent
	/// let question = Input::new("message");
	/// let question = input("message");
	/// ```
	pub fn new(message: M) -> Self {
		Input {
			message,
			initial_value: None,
			placeholder: None,
			validate: None,
			cancel: None,
		}
	}

	/// Specify a placeholder.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// let answer = input("message").placeholder("placeholder").required();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn placeholder<S: Into<String>>(&mut self, placeholder: S) -> &mut Self {
		self.placeholder = Some(placeholder.into());
		self
	}

	/// Specify the initial value.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// let answer = input("message").initial_value("initial_value").interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn initial_value<S: Into<String>>(&mut self, initial_value: S) -> &mut Self {
		self.initial_value = Some(initial_value.into());
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
	/// use may_clack::input;
	///
	/// let answer = input("message")
	///     .validate(|x| (!x.is_ascii()).then_some("only use ascii characters"))
	///     .interact();
	/// println!("answer {:?}", answer);
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
	/// use may_clack::{input, cancel};
	///
	/// let answer = input("message").cancel(do_cancel).interact();
	/// println!("answer {:?}", answer);
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

	fn interact_once<T: FromStr>(&self, enforce_non_empty: bool) -> Result<Option<T>, ClackError>
	where
		T::Err: Display,
	{
		let prompt = format!("{}  ", *chars::BAR);

		let mut editor = Editor::new()?;
		let helper = PlaceholderHightlighter::new(self.placeholder.as_deref());
		editor.set_helper(Some(helper));

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

						if let Some(helper) = editor.helper_mut() {
							helper.is_val = true;
						}

						self.w_val("value is required");
					} else {
						break Ok(None);
					}
				} else if let Some(text) = self.do_validate(&value) {
					initial_value = Some(value);

					if let Some(helper) = editor.helper_mut() {
						helper.is_val = true;
					}

					self.w_val(text);
				} else {
					match value.parse::<T>() {
						Ok(val) => break Ok(Some(val)),
						Err(err) => {
							initial_value = Some(value);

							if let Some(helper) = editor.helper_mut() {
								helper.is_val = true;
							}

							self.w_val(&err.to_string());
						}
					}
				}
			} else {
				break Err(ClackError::Cancelled);
			}
		}
	}

	/// Like [`Input::required()`], but parses the value before returning.
	///
	/// Useful for getting number inputs.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// # fn main() -> Result<(), may_clack::error::ClackError> {
	/// let answer: i32 = input("message").parse::<i32>()?;
	/// println!("answer {:?}", answer);
	/// # Ok(())
	/// # }
	/// ```
	pub fn parse<T: FromStr + Display>(&self) -> Result<T, ClackError>
	where
		T::Err: Display,
	{
		self.w_init();

		let interact = self.interact_once::<T>(true);
		match interact {
			Ok(Some(value)) => {
				self.w_out(&value.to_string());
				Ok(value)
			}
			Ok(None) => unreachable!(),
			Err(ClackError::Cancelled) => {
				self.w_cancel();
				if let Some(cancel) = self.cancel.as_deref() {
					cancel();
				}

				Err(ClackError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}

	/// Like [`Input::parse()`]
	pub fn maybe_parse<T: FromStr + Display>(&self) -> Result<Option<T>, ClackError>
	where
		T::Err: Display,
	{
		self.w_init();

		let interact = self.interact_once::<T>(false);
		match interact {
			Ok(val) => {
				let v = val.as_ref().map_or(String::new(), ToString::to_string);
				self.w_out(&v);
				Ok(val)
			}
			Err(ClackError::Cancelled) => {
				self.w_cancel();
				if let Some(cancel) = self.cancel.as_deref() {
					cancel();
				}

				Err(ClackError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}

	/// Like [`Input::interact()`], but does not return an empty line.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// let answer = input("message").required();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn required(&self) -> Result<String, ClackError> {
		self.w_init();

		let interact = self.interact_once::<String>(true);
		match interact {
			Ok(Some(value)) => {
				self.w_out(&value);
				Ok(value)
			}
			Ok(None) => unreachable!(),
			Err(ClackError::Cancelled) => {
				self.w_cancel();
				if let Some(cancel) = self.cancel.as_deref() {
					cancel();
				}

				Err(ClackError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}

	/// Waits for the user to submit a line of text.
	///
	/// Returns [`None`] on an empty line and [`Some::<String>`] otherwise.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{input, cancel};
	///
	/// let answer = input("message")
	///     .initial_value("initial_value")
	///     .validate(|x| x.parse::<u32>().err().map(|_| "invalid u32"))
	///     .cancel(do_cancel)
	///     .interact();
	/// println!("answer {:?}", answer);
	///
	/// fn do_cancel() {
	///     cancel!("operation cancelled");
	///     std::process::exit(1);
	/// }
	/// ```
	pub fn interact(&self) -> Result<Option<String>, ClackError> {
		self.w_init();

		let interact = self.interact_once(false);
		match interact {
			Ok(val) => {
				let v = val.as_deref().unwrap_or("");
				self.w_out(v);
				Ok(val)
			}
			Err(ClackError::Cancelled) => {
				self.w_cancel();
				if let Some(cancel) = self.cancel.as_deref() {
					cancel();
				}

				Err(ClackError::Cancelled)
			}
			Err(err) => Err(err),
		}
	}
}

impl<M: Display> Input<M> {
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

	fn w_val(&self, text: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_ERROR).yellow(), self.message);
		println!("{}", (*chars::BAR).yellow());

		print!("{}", ansi::CLEAR_LINE);
		print!("{}  {}", (*chars::BAR_END).yellow(), text.yellow());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();
	}

	fn w_out(&self, value: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_SUBMIT).green(), self.message);
		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, value.dimmed());

		print!("{}", ansi::CLEAR_LINE);
	}

	fn w_cancel(&self) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.flush();

		println!("{}  {}", (*chars::STEP_CANCEL).red(), self.message);

		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, "cancelled".strikethrough().dimmed());

		print!("{}", ansi::CLEAR_LINE);
	}
}

/// Shorthand for [`Input::new()`]
pub fn input<M: Display>(message: M) -> Input<M> {
	Input::new(message)
}
