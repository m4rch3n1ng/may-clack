use crate::{
	error::ClackError,
	style::{ansi, chars, IS_UNICODE},
};
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::{
	fmt::Display,
	io::{stdout, Write},
};
use unicode_truncate::UnicodeTruncateStr;

#[derive(Debug, Clone)]
pub struct Opt<T: Clone, O: Display + Clone> {
	value: T,
	label: O,
	hint: Option<String>,
	active: bool,
}

impl<T: Clone, O: Display + Clone> Opt<T, O> {
	/// Creates a new `Opt` struct.
	///
	/// # Examples
	/// 
	/// ```
	/// use may_clack::multi_select::Opt;
	///
	/// let option = Opt::new("value", "lavel", Some("hint"));
	/// ```
	pub fn new<S: Into<String>>(value: T, label: O, hint: Option<S>) -> Self {
		Opt {
			value,
			label,
			hint: hint.map(|st| st.into()),
			active: false,
		}
	}

	/// Creates a new `Opt` struct without a hint
	///
	/// # Examples
	/// 
	/// ```
	/// use may_clack::multi_select::Opt;
	///
	/// let option = Opt::simple("value", "label");
	/// ```
	pub fn simple(value: T, label: O) -> Self {
		Opt::new(value, label, None::<String>)
	}

	/// Creates a new `Opt` struct with a hint.
	/// 
	/// # Examples
	///
	/// ```
	/// use may_clack::multi_select::Opt;
	///
	/// let option = Opt::hint("value", "label", "hint");
	/// ```
	pub fn hint<S: Into<String>>(value: T, label: O, hint:S ) -> Self {
		Opt::new(value, label, Some(hint))
	}

	fn toggle(&mut self) {
		self.active = !self.active;
	}

	fn trunc(&self, hint: usize) -> String {
		let size = crossterm::terminal::size();
		let label = format!("{}", self.label);

		let one_three = if *IS_UNICODE { 1 } else { 3 };

		match size {
			Ok((width, _height)) => label
				.unicode_truncate(width as usize - 4 - one_three - hint)
				.0
				.to_owned(),
			Err(_) => label,
		}
	}

	fn focus(&self) -> String {
		let hint_len = self.hint.as_deref().map_or(0, |hint| hint.len() + 3);
		let label = self.trunc(hint_len);

		let fmt = if self.active {
			format!("{} {}", style(*chars::CHECKBOX_SELECTED).green(), label)
		} else {
			format!("{} {}", style(*chars::CHECKBOX_ACTIVE).cyan(), label)
		};

		if let Some(hint) = &self.hint {
			let hint = format!("({})", hint);
			format!("{} {}", fmt, style(hint).dim())
		} else {
			fmt
		}
	}

	fn unfocus(&self) -> String {
		let label = self.trunc(0);

		if self.active {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_SELECTED).green(),
				style(label).dim()
			)
		} else {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_INACTIVE).dim(),
				style(label).dim()
			)
		}
	}
}

#[derive(Debug, Clone)]
pub struct MultiSelect<M: Display, T: Clone, O: Display + Clone> {
	message: M,
	options: Vec<Opt<T, O>>,
	less: Option<u16>,
}

impl<M: Display, T: Clone, O: Display + Clone> MultiSelect<M, T, O> {
	/// Creates a new `MultiSelect` struct.
	///
	/// Has a shorthand version in [`multi_select()`]
	///
	/// # Examples
	/// 
	/// ```no_run
	/// use may_clack::{multi_select, multi_select::MultiSelect};
	///
	/// // these two are equivalent
	/// let mut question = MultiSelect::new("message");
	/// question.option("value", "hint");
	///
	/// let mut question = multi_select("message");
	/// question.option("value", "hint");
	/// ```
	pub fn new(message: M) -> Self {
		MultiSelect {
			message,
			options: vec![],
			less: None,
		}
	}

	/// Add an option without a hint.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_select;
	///
	/// let answer = multi_select("message")
	///     .option("val1", "label 1")
	///     .option("val2", "label 2")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn option(&mut self, val: T, label: O) -> &mut Self {
		// todo duplicate
		let opt = Opt::new(val, label, None::<String>);
		self.options.push(opt);
		self
	}

	/// Add an option with a hint.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_select;
	///
	/// let answer = multi_select("message")
	///     .option("val1", "label 1")
	///     .option_hint("val2", "label 2", "hint")
	///     .option("val3", "label 3")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn option_hint<S: Into<String>>(&mut self, val: T, label: O, hint: S) -> &mut Self {
		let opt = Opt::new(val, label, Some(hint));
		self.options.push(opt);
		self
	}

	/// Add multiple options.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::{multi_select, multi_select::Opt};
	///
	/// let opts = vec![
	///     Opt::simple("val1", "label 1"),
	///     Opt::hint("val2", "label 2", "hint"),
	///     Opt::simple("val3", "label 3")
	/// ];
	///
	/// let answer = multi_select("message").options(opts).interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn options(&mut self, options: Vec<Opt<T, O>>) -> &mut Self {
		self.options = options;
		self
	}

	/// Enable paging with the specified amount of lines.
	///
	/// # Panics
	///
	/// Panics when the given value is 0.
	///
	/// # Examples
	///
	/// ```no_run
	/// use may_clack::multi_select;
	/// 
	/// let answer = multi_select("message")
	///     .option("val 1", "value 1")
	///     .option("val 2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .option("val 4", "value 4")
	///     .option("val 5", "value 5")
	///     .less(3)
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn less(&mut self, less: u16) -> &mut Self {
		assert!(less > 0, "less value has to be greater than zero");
		self.less = Some(less);
		self
	}

	/// Wait for the user to submit the selected options.
	/// 
	/// # Examples
	/// 
	/// ```no_run
	/// use may_clack::multi_select;
	/// 
	/// let answer = multi_select("select")
	///     .option("val1", "value 1")
	///     .option("val2", "value 2")
	///     .option_hint("val 3", "value 3", "hint")
	///     .interact();
	/// println!("answer {:?}", answer);
	/// ```
	pub fn interact(&self) -> Result<Vec<T>, ClackError> {
		if self.options.is_empty() {
			return Err(ClackError::NoOptions);
		}

		let mut options = self.options.clone();

		let max = self.options.len();
		let is_less = self.less.is_some() && self.options.len() as u16 > self.less.unwrap();

		let mut idx = 0;
		let mut less_idx: u16 = 0;

		if !is_less {
			self.w_init();
		} else {
			self.w_init_less();
		}

		let term = Term::stdout();
		loop {
			match term.read_key()? {
				Key::ArrowUp | Key::ArrowLeft => {
					if !is_less {
						self.draw_unfocus(&options, idx);
						let mut stdout = stdout();

						if idx > 0 {
							idx -= 1;
							let _ = stdout.queue(cursor::MoveUp(1));
						} else {
							idx = max - 1;
							let _ = stdout.queue(cursor::MoveDown(max as u16 - 1));
						}

						let _ = stdout.flush();
						self.draw_focus(&options, idx);
					} else {
						let prev_less = less_idx;

						if idx > 0 {
							idx -= 1;
							less_idx = less_idx.saturating_sub(1);
						} else {
							let less = self.less.expect("less should unwrap if is_less");
							idx = max - 1;
							less_idx = less - 1;
						}

						self.draw_less(&options, idx, less_idx, prev_less);
					}
				}
				Key::ArrowDown | Key::ArrowRight => {
					if !is_less {
						self.draw_unfocus(&options, idx);
						let mut stdout = stdout();

						if idx < max - 1 {
							idx += 1;
							let _ = stdout.queue(cursor::MoveDown(1));
						} else {
							idx = 0;
							let _ = stdout.queue(cursor::MoveUp(max as u16 - 1));
						}

						let _ = stdout.flush();
						self.draw_focus(&options, idx);
					} else {
						let prev_less = less_idx;

						if idx < max - 1 {
							let less = self.less.expect("less should unwrap if is_less");
							idx += 1;
							if less_idx < less - 1 {
								less_idx += 1;
							}
						} else {
							idx = 0;
							less_idx = 0;
						}

						self.draw_less(&options, idx, less_idx, prev_less);
					}
				}
				Key::PageDown => {
					if is_less {
						let prev_less = less_idx;
						let less = self.less.expect("less should unwrap if is_less");

						if idx + less as usize >= max - 1 {
							less_idx = less - 1;
							idx = max - 1;
						} else {
							idx += less as usize;

							if max - idx < (less - less_idx) as usize {
								less_idx = less - (max - idx) as u16;
							}
						}

						self.draw_less(&options, idx, less_idx, prev_less);
					}
				}
				Key::PageUp => {
					if is_less {
						let prev_less = less_idx;
						let less = self.less.expect("less should unwrap if is_less");

						if idx <= less as usize {
							less_idx = 0;
							idx = 0;
						} else {
							idx -= less as usize;
							less_idx = prev_less.min(idx as u16);
						}

						self.draw_less(&options, idx, less_idx, prev_less);
					}
				}
				Key::Char(' ') => {
					let opt = options.get_mut(idx).expect("idx should always be in bound");
					opt.toggle();
					self.draw_focus(&options, idx);
				}
				Key::Enter => {
					let selected_opts = options.iter().filter(|opt| opt.active).collect::<Vec<_>>();

					if !is_less {
						self.w_out(idx, &selected_opts);
					} else {
						self.w_out_less(less_idx, &selected_opts);
					}

					let all = options
						.iter()
						.filter(|opt| opt.active)
						.cloned()
						.map(|opt| opt.value)
						.collect();

					return Ok(all);
				}
				_ => {}
			}
		}
	}
}

impl<M: Display, T: Clone, O: Display + Clone> MultiSelect<M, T, O> {
	fn draw_focus(&self, options: &[Opt<T, O>], idx: usize) {
		let opt = options.get(idx).expect("idx should always be in bound");
		let line = opt.focus();
		self.draw(&line);
	}

	fn draw_unfocus(&self, options: &[Opt<T, O>], idx: usize) {
		let opt = options.get(idx).expect("idx should always be in bound");
		let line = opt.unfocus();
		self.draw(&line);
	}

	fn draw(&self, line: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		print!("{}", ansi::CLEAR_LINE);
		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}

	fn draw_less(&self, opts: &[Opt<T, O>], idx: usize, less_idx: u16, prev_less: u16) {
		let mut stdout = stdout();
		if prev_less > 0 {
			let _ = stdout.queue(cursor::MoveToPreviousLine(prev_less));
		}

		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let less = self.less.expect("less should unwrap if is_less");
		for i in 0..less.into() {
			let _ = stdout.queue(cursor::MoveToColumn(0));
			let _ = stdout.flush();

			let i_idx = idx + i - less_idx as usize;
			let opt = opts.get(i_idx).unwrap();
			let line = opt.unfocus();

			print!("{}", ansi::CLEAR_LINE);
			println!("{}  {}", style(*chars::BAR).cyan(), line);
		}

		let max = self.options.len();
		let amt = max.to_string().len();
		print!("{}", ansi::CLEAR_LINE);
		println!(
			"{}  ......... ({:#0amt$}/{})",
			style(*chars::BAR).cyan(),
			idx + 1,
			max,
			amt = amt
		);

		let _ = stdout.queue(cursor::MoveToPreviousLine(less + 1));
		let _ = stdout.flush();

		if less_idx > 0 {
			let _ = stdout.queue(cursor::MoveToNextLine(less_idx));
			let _ = stdout.flush();
		}

		self.draw_focus(opts, idx);
	}
}

impl<M: Display, T: Clone, O: Display + Clone> MultiSelect<M, T, O> {
	fn w_init_less(&self) {
		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		self.draw_less(&self.options, 0, 0, 0);

		let less = self.less.expect("less should unwrap if is_less");
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToNextLine(less));
		let _ = stdout.flush();

		println!("{}  .........", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(less + 1));
		let _ = stdout.flush();
	}

	fn w_init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		for opt in &self.options {
			let line = opt.unfocus();
			println!("{}  {}", style(*chars::BAR).cyan(), line);
		}

		print!("{}", style(*chars::BAR_END).cyan());

		let len = self.options.len() as u16;
		let _ = stdout.queue(cursor::MoveToPreviousLine(len));
		let _ = stdout.flush();

		self.draw_focus(&self.options, 0);
	}

	fn w_out(&self, idx: usize, selected: &[&Opt<T, O>]) {
		let mut stdout = stdout();

		let _ = stdout.queue(cursor::MoveToPreviousLine(idx as u16 + 1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		for _ in &self.options {
			println!("{}", ansi::CLEAR_LINE);
		}
		println!("{}", ansi::CLEAR_LINE);

		let mv = self.options.len() as u16 + 1;
		let _ = stdout.queue(cursor::MoveToPreviousLine(mv));

		let vals = selected
			.iter()
			.map(|&opt| opt.label.clone())
			.collect::<Vec<_>>();

		let val_string = if vals.is_empty() {
			"none".into()
		} else {
			self.join(&vals)
		};
		println!("{}  {}", *chars::BAR, style(val_string).dim());
	}

	fn w_out_less(&self, less_idx: u16, selected: &[&Opt<T, O>]) {
		let mut stdout = stdout();
		if less_idx > 0 {
			let _ = stdout.queue(cursor::MoveToPreviousLine(less_idx));
		}

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		let less = self.less.expect("less should unwrap if is_less");
		for _ in 0..less.into() {
			println!("{}", ansi::CLEAR_LINE);
		}
		println!("{}", ansi::CLEAR_LINE);
		println!("{}", ansi::CLEAR_LINE);

		let mv = less + 2;
		let _ = stdout.queue(cursor::MoveToPreviousLine(mv));

		let vals = selected
			.iter()
			.map(|&opt| opt.label.clone())
			.collect::<Vec<_>>();

		let val_string = if vals.is_empty() {
			"none".into()
		} else {
			self.join(&vals)
		};
		println!("{}  {}", *chars::BAR, style(val_string).dim());
	}

	fn join(&self, v: &[O]) -> String {
		v.iter()
			.map(|val| format!("{}", val))
			.collect::<Vec<_>>()
			.join(", ")
	}
}

/// Shorthand for [`MultiSelect::new()`]
pub fn multi_select<M: Display, T: Clone, O: Display + Clone>(message: M) -> MultiSelect<M, T, O> {
	MultiSelect::new(message)
}
