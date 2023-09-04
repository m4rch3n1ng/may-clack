//! Confirm
use crate::{
	error::ClackError,
	style::{ansi, chars},
};
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
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
#[derive(Debug, Clone)]
pub struct Confirm<M: Display> {
	message: M,
	initial_value: bool,
	prompts: (String, String),
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

		let term = Term::stdout();
		// let _ = term.hide_cursor(); // todo

		let mut val = self.initial_value;
		loop {
			match term.read_key()? {
				Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight => {
					val = !val;
					self.draw(val);
				}
				Key::Char('y' | 'Y') => {
					let _ = term.show_cursor();
					self.w_out(true);
					return Ok(true);
				}
				Key::Char('n' | 'N') => {
					let _ = term.show_cursor();
					self.w_out(false);
					return Ok(false);
				}
				Key::Enter => {
					let _ = term.show_cursor();
					self.w_out(val);
					return Ok(val);
				}
				_ => {}
			}
		}
	}
}

impl<M: Display> Confirm<M> {
	/// Format a radio point.
	fn radio_pnt(&self, is_active: bool, prompt: &str) -> String {
		if is_active {
			format!("{} {}", style(*chars::RADIO_ACTIVE).green(), prompt)
		} else {
			style(format!("{} {}", *chars::RADIO_INACTIVE, prompt))
				.dim()
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
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let r = self.radio(value);
		print!("{}  {}", style("│").cyan(), r);
		let _ = stdout.flush();
	}
}

impl<M: Display> Confirm<M> {
	/// Write initial prompt.
	fn w_init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		self.draw(self.initial_value);

		let _ = stdout.flush();
	}

	/// Write outro prompt.
	fn w_out(&self, value: bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		let answer = if value {
			&self.prompts.0
		} else {
			&self.prompts.1
		};

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);
		print!("{}", ansi::CLEAR_LINE);
		println!("{}  {}", *chars::BAR, style(answer).dim());
	}
}

/// Shorthand for [`Confirm::new()`]
pub fn confirm<M: Display>(message: M) -> Confirm<M> {
	Confirm::new(message)
}
