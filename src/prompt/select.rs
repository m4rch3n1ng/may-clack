//! Select option
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
use unicode_truncate::UnicodeTruncateStr;

/// `Select` `Opt` struct
#[derive(Debug, Clone)]
pub struct Opt<T: Clone, O: Display + Clone> {
	value: T,
	label: O,
	hint: Option<String>,
}

impl<T: Clone, O: Display + Clone> Opt<T, O> {
	/// Creates a new `Opt` struct.
	///
	/// # Examples
	///
	/// ```
	/// use may_clack::select::Opt;
	///
	/// let option = Opt::new("value", "lavel", Some("hint"));
	/// ```
	pub fn new<S: Into<String>>(value: T, label: O, hint: Option<S>) -> Self {
		Opt {
			value,
			label,
			hint: hint.map(|hint| hint.into()),
		}
	}

	/// Creates a new `Opt` struct without a hint
	///
	/// # Examples
	///
	/// ```
	/// use may_clack::select::Opt;
	///
	/// let option = Opt::simple("value", "label");
	/// ```
	pub fn simple(value: T, label: O) -> Self {
		Opt::new(value, label, None::<String>)
	}

	/// Creates a new `Opt` struct with a hint
	///
	/// # Examples
	///
	/// ```
	/// use may_clack::select::Opt;
	///
	/// let option = Opt::hint("value", "label", "hint");
	/// ```
	pub fn hint<S: Into<String>>(value: T, label: O, hint: S) -> Self {
		Opt::new(value, label, Some(hint))
	}

	fn trunc(&self, hint: usize) -> String {
		let size = crossterm::terminal::size();
		let label = format!("{}", self.label);

		match size {
			Ok((width, _height)) => label
				.unicode_truncate(width as usize - 5 - hint)
				.0
				.to_owned(),
			Err(_) => label,
		}
	}

	fn focus(&self) -> String {
		let hint_len = self.hint.as_deref().map_or(0, |hint| hint.len() + 3);
		let label = self.trunc(hint_len);

		let fmt = format!("{} {}", (*chars::RADIO_ACTIVE).green(), label);

		if let Some(hint) = &self.hint {
			let hint = format!("({})", hint);
			format!("{} {}", fmt, hint.dimmed())
		} else {
			fmt
		}
	}

	fn unfocus(&self) -> String {
		let label = self.trunc(0);
		format!("{} {}", (*chars::RADIO_INACTIVE).dimmed(), label.dimmed())
	}
}

/// `Select` struct.
///
/// # Examples
///
/// ```no_run
/// use may_clack::select;
///
/// let answer = select("message")
///     .option("val 1", "value 1")
///     .option("val 2", "value 2")
///     .option_hint("val 3", "value 3", "hint")
///     .option("val 4", "value 4")
///     .option("val 5", "value 5")
///     .less_amt(3)
///     .interact();
/// println!("answer {:?}", answer);
/// ```
pub struct Select<M: Display, T: Clone, O: Display + Clone> {
	message: M,
	less: bool,
	less_amt: Option<u16>,
	less_max: Option<u16>,
	cancel: Option<Box<dyn Fn()>>,
	options: Vec<Opt<T, O>>,
}

impl<M: Display, T: Clone, O: Display + Clone> Select<M, T, O> {
	/// Creates a new `Select` struct.
	///
	/// Has a shorthand version in [`select()`]
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{select, select::Select};
	///
	/// // these two are equivalent
	/// let mut question = Select::new("message");
	/// question.option("value", "hint");
	///
	/// let mut question = select("message");
	/// question.option("value", "hint");
	/// ```
	pub fn new(message: M) -> Self {
		Select {
			message,
			less: false,
			less_amt: None,
			less_max: None,
			cancel: None,
			options: vec![],
		}
	}

	/// Add an option without a hint.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("message")
	///     .option("val1", "label 1")
	///     .option("val2", "label 2")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn option(&mut self, value: T, label: O) -> &mut Self {
		let opt = Opt::new(value, label, None::<String>);
		self.options.push(opt);
		self
	}

	/// Add an option with a hint.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("message")
	///     .option("val1", "label 1")
	///     .option_hint("val2", "label 2", "hint")
	///     .option("val3", "label 3")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn option_hint<S: Into<String>>(&mut self, value: T, label: O, hint: S) -> &mut Self {
		let opt = Opt::new(value, label, Some(hint));
		self.options.push(opt);
		self
	}

	/// Add multiple options.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{select, select::Opt};
	///
	/// let opts = vec![
	///     Opt::simple("val1", "label 1"),
	///     Opt::hint("val2", "label 2", "hint"),
	///     Opt::simple("val3", "label 3")
	/// ];
	///
	/// let answer = select("message").options(opts).interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn options(&mut self, options: Vec<Opt<T, O>>) -> &mut Self {
		self.options = options;
		self
	}

	/// Enable paging with the amount of terminal rows.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("message")
	///     .option("val 1", "value 1")
	///     .option("val 2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .option("val 4", "value 4")
	///     .option("val 5", "value 5")
	///     .less()
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn less(&mut self) -> &mut Self {
		self.less = true;
		self
	}

	/// Enable paging with the amount of terminal rows, additionally setting a maximum amount.
	///
	/// # Panics
	///
	/// Panics when the given value is 0.  
	/// Panics when called after [`Select::less_amt`] has already been called.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("message")
	///     .option("val 1", "value 1")
	///     .option("val 2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .option("val 4", "value 4")
	///     .option("val 5", "value 5")
	///     .less_max(3)
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn less_max(&mut self, max: u16) -> &mut Self {
		assert!(max > 0, "less max value has to be greater than zero");
		assert!(
			self.less_amt.is_none(),
			"cannot set both less_amt and less_max"
		);
		self.less = true;
		self.less_max = Some(max);
		self
	}

	/// Enable paging with the specified amount of lines.
	///
	/// # Panics
	///
	/// Panics when the given value is 0.  
	/// Panics when called after [`Select::less_max`] has already been called.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("message")
	///     .option("val 1", "value 1")
	///     .option("val 2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .option("val 4", "value 4")
	///     .option("val 5", "value 5")
	///     .less_amt(3)
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn less_amt(&mut self, less: u16) -> &mut Self {
		assert!(less > 0, "less value has to be greater than zero");
		assert!(
			self.less_max.is_none(),
			"cannot set both less_amt and less_max"
		);
		self.less = true;
		self.less_amt = Some(less);
		self
	}

	/// Specify function to call on cancel.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{select, cancel};
	///
	/// let answer = select("select")
	///     .option("val1", "value 1")
	///     .option("val2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .cancel(do_cancel)
	///     .interact();
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

	fn mk_less(&self) -> Option<u16> {
		if !self.less {
			return None;
		}

		if let Some(less) = self.less_amt {
			let is_less = self.options.len() > less as usize;
			is_less.then_some(less)
		} else if let Ok((_, rows)) = crossterm::terminal::size() {
			let len = self.options.len();
			let rows = rows.saturating_sub(4);
			let rows = self.less_max.map_or(rows, |max| u16::min(rows, max));

			let is_less = rows > 0 && len > rows as usize;
			is_less.then_some(rows)
		} else {
			None
		}
	}

	/// Wait for the user to submit an option.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::select;
	///
	/// let answer = select("select")
	///     .option("val1", "value 1")
	///     .option("val2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn interact(&self) -> Result<T, ClackError> {
		if self.options.is_empty() {
			return Err(ClackError::NoOptions);
		}

		let max = self.options.len();
		let is_less = self.mk_less();

		let mut idx = 0;
		let mut less_idx: u16 = 0;

		if let Some(less) = is_less {
			self.w_init_less(less);
		} else {
			self.w_init();
		}

		terminal::enable_raw_mode()?;

		loop {
			if let Event::Key(key) = event::read()? {
				match (key.code, key.modifiers) {
					(KeyCode::Up | KeyCode::Left, _) => {
						if let Some(less) = is_less {
							let prev_less = less_idx;

							if idx > 0 {
								idx -= 1;
								less_idx = less_idx.saturating_sub(1);
							} else {
								idx = max - 1;
								less_idx = less - 1;
							}

							self.draw_less(less, idx, less_idx, prev_less);
						} else {
							self.draw_unfocus(idx);
							let mut stdout = stdout();

							if idx > 0 {
								idx -= 1;
								let _ = execute!(stdout, cursor::MoveUp(1));
							} else {
								idx = max - 1;
								let _ = execute!(stdout, cursor::MoveDown(max as u16 - 1));
							}

							self.draw_focus(idx);
						}
					}
					(KeyCode::Down | KeyCode::Right, _) => {
						if let Some(less) = is_less {
							let prev_less = less_idx;

							if idx < max - 1 {
								idx += 1;
								if less_idx < less - 1 {
									less_idx += 1;
								}
							} else {
								idx = 0;
								less_idx = 0;
							}

							self.draw_less(less, idx, less_idx, prev_less);
						} else {
							self.draw_unfocus(idx);
							let mut stdout = stdout();

							if idx < max - 1 {
								idx += 1;
								let _ = execute!(stdout, cursor::MoveDown(1));
							} else {
								idx = 0;
								let _ = execute!(stdout, cursor::MoveUp(max as u16 - 1));
							}

							self.draw_focus(idx);
						}
					}
					(KeyCode::PageDown, _) => {
						if let Some(less) = is_less {
							let prev_less = less_idx;

							if idx + less as usize >= max - 1 {
								less_idx = less - 1;
								idx = max - 1;
							} else {
								idx += less as usize;

								if max - idx < (less - less_idx) as usize {
									less_idx = less - (max - idx) as u16;
								}
							}

							self.draw_less(less, idx, less_idx, prev_less);
						}
					}
					(KeyCode::PageUp, _) => {
						if let Some(less) = is_less {
							let prev_less = less_idx;

							if idx <= less as usize {
								less_idx = 0;
								idx = 0;
							} else {
								idx -= less as usize;
								less_idx = prev_less.min(idx as u16);
							}

							self.draw_less(less, idx, less_idx, prev_less);
						}
					}
					(KeyCode::Enter, _) => {
						terminal::disable_raw_mode()?;

						if let Some(less) = is_less {
							self.w_out_less(less, idx, less_idx);
						} else {
							self.w_out(idx);
						}

						let opt = self
							.options
							.get(idx)
							.cloned()
							.expect("idx should always be in bound");
						return Ok(opt.value);
					}
					(KeyCode::Char('c'), KeyModifiers::CONTROL) => {
						terminal::disable_raw_mode()?;

						if let Some(less) = is_less {
							self.w_cancel_less(less, idx, less_idx);
						} else {
							self.w_cancel(idx);
						}

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

impl<M: Display, T: Clone, O: Display + Clone> Select<M, T, O> {
	fn draw_focus(&self, idx: usize) {
		let opt = self
			.options
			.get(idx)
			.expect("idx should always be in bound");
		let line = opt.focus();
		self.draw(&line);
	}

	fn draw_unfocus(&self, idx: usize) {
		let opt = self
			.options
			.get(idx)
			.expect("idx should always be in bound");
		let line = opt.unfocus();
		self.draw(&line);
	}

	fn draw(&self, line: &str) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToColumn(0));

		print!("{}", ansi::CLEAR_LINE);
		print!("{}  {}", (*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}

	fn draw_less(&self, less: u16, idx: usize, less_idx: u16, prev_less: u16) {
		let mut stdout = stdout();
		if prev_less > 0 {
			let _ = execute!(stdout, cursor::MoveToPreviousLine(prev_less));
		} else {
			let _ = execute!(stdout, cursor::MoveToColumn(0));
		}

		for i in 0..less.into() {
			let i_idx = idx + i - less_idx as usize;
			let opt = self.options.get(i_idx).unwrap();
			let line = opt.unfocus();

			print!("{}", ansi::CLEAR_LINE);
			println!("{}  {}\r", (*chars::BAR).cyan(), line);

			let _ = execute!(stdout, cursor::MoveToColumn(0));
		}

		let max = self.options.len();
		let amt = max.to_string().len();
		print!("{}", ansi::CLEAR_LINE);
		println!(
			"{}  ......... ({:#0amt$}/{})",
			(*chars::BAR).cyan(),
			idx + 1,
			max,
			amt = amt
		);

		let _ = execute!(stdout, cursor::MoveToPreviousLine(less + 1));
		if less_idx > 0 {
			let _ = execute!(stdout, cursor::MoveToNextLine(less_idx));
		}

		self.draw_focus(idx);
	}
}

impl<M: Display, T: Clone, O: Display + Clone> Select<M, T, O> {
	fn w_init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", (*chars::STEP_ACTIVE).cyan(), self.message);

		for opt in &self.options {
			let line = opt.unfocus();
			println!("{}  {}", (*chars::BAR).cyan(), line);
		}

		print!("{}", (*chars::BAR_END).cyan());

		let len = self.options.len() as u16;
		let _ = execute!(stdout, cursor::MoveToPreviousLine(len));

		self.draw_focus(0);
	}

	fn w_init_less(&self, less: u16) {
		println!("{}", *chars::BAR);
		println!("{}  {}", (*chars::STEP_ACTIVE).cyan(), self.message);

		self.draw_less(less, 0, 0, 0);

		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToNextLine(less));

		println!();
		print!("{}", (*chars::BAR_END).cyan());

		let _ = execute!(stdout, cursor::MoveToPreviousLine(less + 1));

		self.draw_focus(0);
	}

	fn w_cancel(&self, idx: usize) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToPreviousLine(idx as u16 + 1));

		println!("{}  {}", (*chars::STEP_CANCEL).red(), self.message);

		for _ in &self.options {
			println!("{}", ansi::CLEAR_LINE);
		}
		print!("{}", ansi::CLEAR_LINE);

		let len = self.options.len() as u16;
		let _ = execute!(stdout, cursor::MoveToPreviousLine(len));

		let label = &self
			.options
			.get(idx)
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, label.strikethrough().dimmed());
	}

	fn w_cancel_less(&self, less: u16, idx: usize, less_idx: u16) {
		let mut stdout = stdout();
		if less_idx > 0 {
			let _ = execute!(stdout, cursor::MoveToPreviousLine(less_idx + 1));
		} else {
			let _ = execute!(stdout, cursor::MoveToPreviousLine(1));
		}

		println!("{}  {}", (*chars::STEP_CANCEL).red(), self.message);

		for _ in 0..less.into() {
			println!("{}", ansi::CLEAR_LINE);
		}

		println!("{}", ansi::CLEAR_LINE);
		println!("{}", ansi::CLEAR_LINE);

		let mv = less + 2;
		let _ = execute!(stdout, cursor::MoveToPreviousLine(mv));

		let label = &self
			.options
			.get(idx)
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, label.strikethrough().dimmed());
	}

	fn w_out(&self, idx: usize) {
		let mut stdout = stdout();
		let _ = execute!(stdout, cursor::MoveToPreviousLine(idx as u16 + 1));

		println!("{}  {}", (*chars::STEP_SUBMIT).green(), self.message);

		for _ in &self.options {
			println!("{}", ansi::CLEAR_LINE);
		}
		print!("{}", ansi::CLEAR_LINE);

		let len = self.options.len() as u16;
		let _ = execute!(stdout, cursor::MoveToPreviousLine(len));

		let label = &self
			.options
			.get(idx)
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, label.dimmed());
	}

	fn w_out_less(&self, less: u16, idx: usize, less_idx: u16) {
		let mut stdout = stdout();
		if less_idx > 0 {
			let _ = execute!(stdout, cursor::MoveToPreviousLine(less_idx + 1));
		} else {
			let _ = execute!(stdout, cursor::MoveToPreviousLine(1));
		}

		println!("{}  {}", (*chars::STEP_SUBMIT).green(), self.message);

		for _ in 0..less.into() {
			println!("{}", ansi::CLEAR_LINE);
		}

		println!("{}", ansi::CLEAR_LINE);
		println!("{}", ansi::CLEAR_LINE);

		let mv = less + 2;
		let _ = execute!(stdout, cursor::MoveToPreviousLine(mv));

		let label = &self
			.options
			.get(idx)
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, label.dimmed());
	}
}

/// Shorthand for [`Select::new()`]
pub fn select<M: Display, T: Clone, O: Display + Clone>(message: M) -> Select<M, T, O> {
	Select::new(message)
}
