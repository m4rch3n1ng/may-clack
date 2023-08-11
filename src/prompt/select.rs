use crate::{error::ClackSelectError, style::chars};
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
		let fmt = match &self.hint {
			Some(hint) => format!(
				"{} {} {}",
				*chars::RADIO_INACTIVE,
				self.label,
				" ".repeat(hint.len() + 2)
			),
			None => format!("{} {}", *chars::RADIO_INACTIVE, self.label),
		};
		style(fmt).to_string()
	}

	fn len(&self) -> usize {
		let check_len = chars::RADIO_ACTIVE.len();
		let label_len = self.label.len();
		let hint_len = self.hint.as_ref().map_or(0, |hint| hint.len() + 1 + 2);

		check_len + 1 + label_len + hint_len
	}
}

#[derive(Debug, Clone)]
pub struct Select<M: Display, T: Clone> {
	message: M,
	options: Vec<Opt<T>>,
}

// todo less mode
impl<M: Display, T: Clone> Select<M, T> {
	pub fn new(message: M) -> Self {
		Select {
			message,
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

	pub fn interact(&self) -> Result<T, ClackSelectError> {
		if self.options.is_empty() {
			return Err(ClackSelectError::NoOptions);
		}

		self.w_init();
		self.draw_focus(0);

		let term = Term::stdout();

		let mut idx = 0;
		let max = self.options.len();
		loop {
			match term.read_key()? {
				Key::ArrowUp | Key::ArrowLeft => {
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
				}
				Key::ArrowDown | Key::ArrowRight => {
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
				}
				Key::Enter => {
					self.w_out(idx);

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

		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}
}

impl<M: Display, T: Clone> Select<M, T> {
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

		for opt in &self.options {
			let len = opt.len();
			println!("   {}", " ".repeat(len));
		}
		println!(" ");

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
}

/// Shorthand for [`Select::new()`]
pub fn select<M: Display, T: Clone>(message: M) -> Select<M, T> {
	Select::new(message)
}
