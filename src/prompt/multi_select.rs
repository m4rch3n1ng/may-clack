use crate::{
	error::ClackSelectError,
	style::{ansi, chars},
};
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::{
	fmt::Display,
	io::{stdout, Write},
};

#[derive(Debug, Clone)]
pub struct Opt<T: Clone, O: Display + Clone> {
	pub value: T,
	pub label: O,
	pub hint: Option<String>,
	pub active: bool,
}

impl<T: Clone, O: Display + Clone> Opt<T, O> {
	pub fn new<S: Into<String>>(value: T, label: O, hint: Option<S>) -> Self {
		Opt {
			value,
			label,
			hint: hint.map(|st| st.into()),
			active: false,
		}
	}

	pub fn simple(value: T, label: O) -> Self {
		Opt::new(value, label, None::<String>)
	}

	fn toggle(&mut self) {
		self.active = !self.active;
	}

	fn focus(&self) -> String {
		let fmt = if self.active {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_SELECTED).green(),
				self.label
			)
		} else {
			format!("{} {}", style(*chars::CHECKBOX_ACTIVE).cyan(), self.label)
		};

		if let Some(hint) = &self.hint {
			let hint = format!("({})", hint);
			format!("{} {}", fmt, style(hint).dim())
		} else {
			fmt
		}
	}

	fn unfocus(&self) -> String {
		if self.active {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_SELECTED).green(),
				style(&self.label).dim()
			)
		} else {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_INACTIVE).dim(),
				style(&self.label).dim()
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
	pub fn new(message: M) -> Self {
		MultiSelect {
			message,
			options: vec![],
			less: None,
		}
	}

	pub fn option(&mut self, val: T, label: O) -> &mut Self {
		// todo duplicate
		let opt = Opt::new(val, label, None::<String>);
		self.options.push(opt);
		self
	}

	pub fn option_hint<S: Into<String>>(&mut self, val: T, label: O, hint: S) -> &mut Self {
		let opt = Opt::new(val, label, Some(hint));
		self.options.push(opt);
		self
	}

	pub fn options(&mut self, options: Vec<Opt<T, O>>) -> &mut Self {
		self.options = options;
		self
	}

	/// Enable paging.
	///
	/// # Panics
	///
	/// Panics when the given value is 0.
	pub fn less(&mut self, less: u16) -> &mut Self {
		assert!(less > 0, "less value has to be greater than zero");
		self.less = Some(less);
		self
	}

	// todo error
	// todo remove mut
	pub fn interact(&self) -> Result<Vec<T>, ClackSelectError> {
		if self.options.is_empty() {
			return Err(ClackSelectError::NoOptions);
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

		let _ = stdout.queue(cursor::MoveToPreviousLine(less));
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
