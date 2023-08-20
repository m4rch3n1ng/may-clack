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
pub struct Opt<T: Clone> {
	value: T,
	label: String,
	hint: Option<String>,
}

impl<T: Clone> Opt<T> {
	pub fn new<S: Into<String>>(value: T, label: S, hint: Option<S>) -> Self {
		Opt {
			value,
			label: label.into(),
			hint: hint.map(|hint| hint.into()),
		}
	}

	pub fn simple<S: Into<String>>(value: T, label: S) -> Self {
		Opt::new(value, label, None)
	}

	fn focus(&self) -> String {
		let fmt = format!("{} {}", style(*chars::RADIO_ACTIVE).green(), self.label);

		if let Some(hint) = &self.hint {
			let hint = format!("({})", hint);
			format!("{} {}", fmt, style(hint).dim())
		} else {
			fmt
		}
	}

	fn unfocus(&self) -> String {
		format!("{} {}", *chars::RADIO_INACTIVE, self.label)
	}
}

#[derive(Debug, Clone)]
pub struct Select<M: Display, T: Clone> {
	message: M,
	options: Vec<Opt<T>>,
	less: Option<u16>,
}

impl<M: Display, T: Clone> Select<M, T> {
	pub fn new(message: M) -> Self {
		Select {
			message,
			less: None,
			options: vec![],
		}
	}

	// todo check for max amt of options
	// todo check duplicates
	pub fn option<S: Into<String>>(&mut self, value: T, label: S) -> &mut Self {
		let opt = Opt::new(value, label, None);
		self.options.push(opt);
		self
	}

	pub fn option_hint<S: Into<String>>(&mut self, value: T, label: S, hint: S) -> &mut Self {
		let opt = Opt::new(value, label, Some(hint));
		self.options.push(opt);
		self
	}

	pub fn options(&mut self, options: Vec<Opt<T>>) -> &mut Self {
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

	pub fn interact(&self) -> Result<T, ClackSelectError> {
		if self.options.is_empty() {
			return Err(ClackSelectError::NoOptions);
		}

		let term = Term::stdout();

		let max = self.options.len();
		let is_less = self.less.is_some() && self.options.len() as u16 > self.less.unwrap();

		let mut idx = 0;
		let mut less_idx: u16 = 0;

		if !is_less {
			self.w_init();
			self.draw_focus(0);
		} else {
			self.w_init_less();
		}

		loop {
			match term.read_key()? {
				Key::ArrowUp | Key::ArrowLeft => {
					if !is_less {
						self.draw_unfocus(idx);
						let mut stdout = stdout();

						if idx > 0 {
							idx -= 1;
							let _ = stdout.queue(cursor::MoveUp(1));
						} else {
							idx = max - 1;
							let _ = stdout.queue(cursor::MoveDown(max as u16 - 1));
						}

						let _ = stdout.flush();
						self.draw_focus(idx);
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

						self.draw_less(idx, less_idx, prev_less);
					}
				}
				Key::ArrowDown | Key::ArrowRight => {
					if !is_less {
						self.draw_unfocus(idx);
						let mut stdout = stdout();

						if idx < max - 1 {
							idx += 1;
							let _ = stdout.queue(cursor::MoveDown(1));
						} else {
							idx = 0;
							let _ = stdout.queue(cursor::MoveUp(max as u16 - 1));
						}

						let _ = stdout.flush();
						self.draw_focus(idx);
					} else {
						let prev_less = less_idx;

						if idx < max - 1 {
							idx += 1;
							let less = self.less.expect("less should unwrap if is_less");
							if less_idx < less - 1 {
								less_idx += 1;
							}
						} else {
							idx = 0;
							less_idx = 0;
						}

						self.draw_less(idx, less_idx, prev_less);
					}
				}
				Key::Enter => {
					if !is_less {
						self.w_out(idx);
					} else {
						self.w_out_less(idx, less_idx);
					}

					let opt = self
						.options
						.get(idx)
						.cloned()
						.expect("idx should always be in bound");
					return Ok(opt.value);
				}
				_ => {}
			}
		}
	}
}

impl<M: Display, T: Clone> Select<M, T> {
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
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		print!("{}", ansi::CLEAR_LINE);
		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}

	fn draw_less(&self, idx: usize, less_idx: u16, prev_less: u16) {
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
			let opt = self.options.get(i_idx).unwrap();
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

		self.draw_focus(idx);
	}
}

impl<M: Display, T: Clone> Select<M, T> {
	fn w_init_less(&self) {
		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		self.draw_less(0, 0, 0);

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
	}

	fn w_out(&self, idx: usize) {
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

		let label = self
			.options
			.get(idx)
			.cloned()
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, style(label).dim());
	}

	fn w_out_less(&self, idx: usize, less_idx: u16) {
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

		let label = self
			.options
			.get(idx)
			.cloned()
			.expect("idx should always be in bound")
			.label;
		println!("{}  {}", *chars::BAR, style(label).dim());
	}
}

/// Shorthand for [`Select::new()`]
pub fn select<M: Display, T: Clone>(message: M) -> Select<M, T> {
	Select::new(message)
}
