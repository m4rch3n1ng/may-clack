use crate::style::chars;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
pub struct Opt {
	pub value: String,
	pub label: String,
	pub hint: Option<String>,
	pub active: bool,
}

impl Opt {
	pub fn new<S: Into<String>>(value: S, label: S, hint: Option<S>) -> Self {
		Opt {
			value: value.into(),
			label: label.into(),
			hint: hint.map(|st| st.into()),
			active: false,
		}
	}

	pub fn simple<S: Into<String>>(value: S, label: S) -> Self {
		Opt::new(value, label, None)
	}

	fn toggle(&mut self) {
		self.active = !self.active;
	}

	fn len(&self) -> usize {
		let check_len = chars::CHECKBOX_INACTIVE.len();
		let label_len = self.label.len();
		let hint_len = self.hint.as_ref().map_or(0, |hint| hint.len() + 2 + 1);

		check_len + 1 + label_len + hint_len
	}

	fn select(&self) -> String {
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

	fn unselect(&self) -> String {
		let fmt = if self.active {
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
		};

		if let Some(hint) = &self.hint {
			let len = hint.len() + 2;
			format!("{} {}", fmt, " ".repeat(len))
		} else {
			fmt
		}
	}
}

#[derive(Debug, Clone)]
pub struct MultiSelect {
	message: String,
	options: Vec<Opt>,
}

impl MultiSelect {
	#[must_use]
	pub fn new<S: Into<String>>(message: S) -> Self {
		MultiSelect {
			message: message.into(),
			options: vec![],
		}
	}

	#[must_use]
	pub fn option<S: Into<String>>(mut self, val: S, label: S) -> Self {
		// todo duplicate
		let opt = Opt::new(val, label, None);
		self.options.push(opt);
		self
	}

	#[must_use]
	pub fn option_hint<S: Into<String>>(mut self, val: S, label: S, hint: S) -> Self {
		let opt = Opt::new(val, label, Some(hint));
		self.options.push(opt);
		self
	}

	#[must_use]
	pub fn options(mut self, options: Vec<Opt>) -> Self {
		self.options = options;
		self
	}

	// todo error
	#[must_use]
	pub fn interact(mut self) -> Option<Vec<String>> {
		if self.options.is_empty() {
			return None;
		}

		self.w_init();
		self.draw_select(0);

		let term = Term::stdout();

		let mut idx = 0;
		let max = self.options.len();
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp | Key::ArrowLeft => {
					self.draw_unselect(idx);
					let mut stdout = stdout();

					if idx > 0 {
						idx -= 1;
						let _ = stdout.queue(cursor::MoveUp(1));
					} else {
						idx = max - 1;
						let _ = stdout.queue(cursor::MoveDown(max as u16 - 1));
					}

					let _ = stdout.flush();
					self.draw_select(idx);
				}
				Key::ArrowDown | Key::ArrowRight => {
					self.draw_unselect(idx);
					let mut stdout = stdout();

					if idx < max - 1 {
						idx += 1;
						let _ = stdout.queue(cursor::MoveDown(1));
					} else {
						idx = 0;
						let _ = stdout.queue(cursor::MoveUp(max as u16 - 1));
					}

					let _ = stdout.flush();
					self.draw_select(idx);
				}
				Key::Char(' ') => {
					let opt = self.options.get_mut(idx).unwrap();
					opt.toggle();
					self.draw_select(idx);
				}
				Key::Enter => {
					let indices = self
						.options
						.iter()
						.filter(|opt| opt.active)
						.collect::<Vec<_>>();

					self.w_out(idx, &indices);

					let all = self
						.options
						.into_iter()
						.filter(|opt| opt.active)
						.map(|opt| opt.value)
						.collect();

					return Some(all);
				}
				_ => {}
			}
		}
	}
}

impl MultiSelect {
	fn draw_select(&self, idx: usize) {
		let opt = self.options.get(idx).unwrap();
		let line = opt.select();
		MultiSelect::draw(&line);
	}

	fn draw_unselect(&self, idx: usize) {
		let opt = self.options.get(idx).unwrap();
		let line = opt.unselect();
		MultiSelect::draw(&line);
	}

	fn draw(line: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}
}

impl MultiSelect {
	fn w_init(&self) {
		let mut stdout = stdout();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		for opt in &self.options {
			let line = opt.unselect();
			println!("{}  {}", style(*chars::BAR).cyan(), line);
		}

		print!("{}", style(*chars::BAR_END).cyan());

		let len = self.options.len() as u16;
		let _ = stdout.queue(cursor::MoveToPreviousLine(len));
		let _ = stdout.flush();
	}

	fn w_out(&self, idx: usize, values: &[&Opt]) {
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
		let _ = stdout.queue(cursor::MoveUp(mv));

		let vals = values
			.iter()
			.map(|&opt| opt.label.clone())
			.collect::<Vec<_>>();

		let val_string = if vals.is_empty() {
			"none".into()
		} else {
			vals.join(", ")
		};
		println!("{}  {}", *chars::BAR, style(val_string).dim());
	}
}

#[must_use]
pub fn multi_select<S: Into<String>>(message: S) -> MultiSelect {
	MultiSelect::new(message)
}
