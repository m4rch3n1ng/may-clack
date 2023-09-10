//! Confirm
use crate::{
	error::ClackError,
	style::{ansi, chars},
};
use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyModifiers},
	execute, terminal,
};
use owo_colors::OwoColorize;
use std::{
	fmt::Display,
	io::{stdout, Write},
};

/// `Confirm` struct.
///
/// # Examples
///
/// ```no_run
/// use may_clack::confirm;
///
/// let answer = confirm("message")
///     .initial_value(true)
///     .prompts("true", "false")
///     .interact();
/// println!("answer {:?}", answer);
/// ```
pub struct Confirm<M: Display> {
	message: M,
	initial_value: bool,
	prompts: (String, String),
	cancel: Option<Box<dyn Fn()>>,
}

impl<M: Display> Confirm<M> {
	/// Creates a new `Confirm` struct.
	///
	/// Has a shorthand in [`confirm()`].
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{confirm, confirm::Confirm};
	///
	/// // these two are equivalent
	/// let question = Confirm::new("message");
	/// let question = confirm("message");
	/// ```
	pub fn new(message: M) -> Confirm<M> {
		Confirm {
			message,
			initial_value: false,
			prompts: ("yes".into(), "no".into()),
			cancel: None,
		}
	}

	/// Specify the initial value.
	///
	/// Default: [`false`]
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::confirm;
	///
	/// let answer = confirm("message").initial_value(true).interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn initial_value(&mut self, b: bool) -> &mut Self {
		self.initial_value = b;
		self
	}

	/// Specify the prompts to display for [`true`] and [`false`].
	///
	/// Default: `"yes"`, `"no"`.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::confirm;
	///
	/// let answer = confirm("message").prompts("true", "false").interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn prompts<S: Into<String>>(&mut self, yes: S, no: S) -> &mut Self {
		self.prompts = (yes.into(), no.into());
		self
	}

	/// Specify function to call on cancel.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{confirm, cancel};
	///
	/// let answer = confirm("message").cancel(do_cancel).interact();
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

	/// Wait for the user to submit an answer.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::confirm;
	///
	/// let answer = confirm("message")
	///     .initial_value(true)
	///     .prompts("true", "false")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn interact(&self) -> Result<bool, ClackError> {
		self.w_init();

		let mut stdout = stdout();
		let _ = execute!(stdout, crossterm::cursor::Hide);
		terminal::enable_raw_mode()?;

		let mut val = self.initial_value;
		loop {
			if let Event::Key(key) = event::read()? {
				match (key.code, key.modifiers) {
					(KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right, _) => {
						val = !val;
						self.draw(val);
					}
					(KeyCode::Char('y' | 'Y'), _) => {
						let _ = execute!(stdout, crossterm::cursor::Show);
						terminal::disable_raw_mode()?;
						self.w_out(true);
						return Ok(true);
					}
					(KeyCode::Char('n' | 'N'), _) => {
						let _ = execute!(stdout, crossterm::cursor::Show);
						terminal::disable_raw_mode()?;
						self.w_out(false);
						return Ok(false);
					}
					(KeyCode::Enter, _) => {
						let _ = execute!(stdout, crossterm::cursor::Show);
						terminal::disable_raw_mode()?;
						self.w_out(val);
						return Ok(val);
					}
					(KeyCode::Char('c' | 'd'), KeyModifiers::CONTROL) => {
						let _ = execute!(stdout, crossterm::cursor::Show);
						terminal::disable_raw_mode()?;
						self.w_cancel(val);
						if let Some(cancel) = self.cancel.as_deref() {
							cancel();
						}

						return Err(ClackError::Cancelled);
					}
					_ => {}
				}
			}
		}
	}
}

impl<M: Display> Confirm<M> {
	/// Format a radio point.
	fn radio_pnt(&self, is_active: bool, prompt: &str) -> String {
		if is_active {
			format!("{} {}", (*chars::RADIO_ACTIVE).green(), prompt)
		} else {
			format!("{} {}", *chars::RADIO_INACTIVE, prompt)
				.dimmed()
				.to_string()
		}
	}

	/// Format the actual prompt.
	fn radio(&self, value: bool) -> String {
		let yes = self.radio_pnt(value, &self.prompts.0);
		let no = self.radio_pnt(!value, &self.prompts.1);

		format!("{} / {}", yes, no)
	}

	/// Draw the prompt.
	fn draw(&self, value: bool) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToColumn(0));

		let r = self.radio(value);
		print!("{}  {}", (*chars::BAR).cyan(), r);
		let _ = stdout.flush();
	}
}

impl<M: Display> Confirm<M> {
	/// Write initial prompt.
	fn w_init(&self) {
		println!("{}", *chars::BAR);
		println!("{}  {}", (*chars::STEP_ACTIVE).cyan(), self.message);
		println!("{}", (*chars::BAR).cyan());
		print!("{}", (*chars::BAR_END).cyan());

		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToPreviousLine(1));

		self.draw(self.initial_value);
	}

	/// Write outro prompt.
	fn w_out(&self, value: bool) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToPreviousLine(1));

		let answer = if value {
			&self.prompts.0
		} else {
			&self.prompts.1
		};

		println!("{}  {}", (*chars::STEP_SUBMIT).green(), self.message);
		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, answer.dimmed());
	}

	fn w_cancel(&self, value: bool) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToPreviousLine(1));

		let answer = if value {
			&self.prompts.0
		} else {
			&self.prompts.1
		};

		println!("{}  {}", (*chars::STEP_CANCEL).red(), self.message);
		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, answer.strikethrough().dimmed());
	}
}

/// Shorthand for [`Confirm::new()`]
pub fn confirm<M: Display>(message: M) -> Confirm<M> {
	Confirm::new(message)
}
