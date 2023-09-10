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
};

#[derive(Completer, Helper, Hinter, Validator)]
pub(super) struct PlaceholderHightlighter(pub String);

impl Highlighter for PlaceholderHightlighter {
	fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
		if line.is_empty() {
			Cow::Owned(self.0.dimmed().to_string())
		} else {
			Cow::Borrowed(line)
		}
	}

	fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
		true
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
	default_value: Option<String>,
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
			default_value: None,
			initial_value: None,
			placeholder: None,
			validate: None,
			cancel: None,
		}
	}

	/// Specify the default value to use, when no input is given.
	///
	/// Useful in combination with [`Input::required()`]
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// let answer = input("message").default_value("default_value").required();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn default_value<S: Into<String>>(&mut self, default_value: S) -> &mut Self {
		self.default_value = Some(default_value.into());
		self
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

	fn interact_once(&self, enforce_non_empty: bool) -> Result<Option<String>, ClackError> {
		let default_prompt = format!("{}  ", (*chars::BAR).cyan());
		let val_prompt = format!("{}  ", (*chars::BAR).yellow());

		let mut editor = Editor::new()?;

		if let Some(placeholder) = self.placeholder.clone() {
			let highlighter = PlaceholderHightlighter(placeholder);
			editor.set_helper(Some(highlighter));
		}

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
					if let Some(default_value) = self.default_value.clone() {
						break Ok(Some(default_value));
					} else if enforce_non_empty {
						initial_value = None;

						is_val = true;
						self.w_val("value is required");
					} else {
						break Ok(None);
					}
				} else if let Some(text) = self.do_validate(&value) {
					initial_value = Some(value.clone());

					is_val = true;
					self.w_val(text);
				} else {
					break Ok(Some(value));
				}
			} else {
				break Err(ClackError::Cancelled);
			}
		}
	}

	/// Like [`Input::interact()`], but does not return an empty line.
	///
	/// Useful when used with [`Input::default_value()`], as that means that there can be no empty value.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::input;
	///
	/// let answer = input("message").default_value("default_value").required();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn required(&self) -> Result<String, ClackError> {
		self.w_init();

		let interact = self.interact_once(true);
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
				let v = val.clone().unwrap_or(String::new());
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
