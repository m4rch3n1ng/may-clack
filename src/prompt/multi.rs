use crate::style::chars;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
struct Opt {
	pub value: String,
	pub label: String,
	pub hint: Option<String>,
	pub select: bool,
}

impl Opt {
	pub fn new<S: Into<String>>(value: S, label: S, hint: Option<S>) -> Self {
		Opt {
			value: value.into(),
			label: label.into(),
			hint: hint.map(|st| st.into()),
			select: false,
		}
	}

	pub fn toggle(&mut self) {
		self.select = !self.select;
	}

	pub fn len(&self) -> usize {
		let label_len = self.label.len();
		let check_len = chars::CHECKBOX_INACTIVE.len();
		let hint_len = self.hint.as_ref().map_or(0, |hint| hint.len() + 2 + 1);

		label_len + 1 + check_len + hint_len
	}

	fn select(&self) -> String {
		let fmt = if self.select {
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
		let fmt = if self.select {
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

#[derive(Debug, Default, Clone)]
pub struct MultiSelect {
	message: Option<String>,
	options: Vec<Opt>,
}

impl MultiSelect {
	#[must_use]
	pub fn new() -> Self {
		MultiSelect::default()
	}

	#[must_use]
	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
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

	// todo error
	#[must_use]
	pub fn interact(mut self) -> Option<Vec<String>> {
		if self.options.is_empty() {
			return None;
		}

		self.init();
		self.draw_new(0);

		let term = Term::stdout();

		let mut idx = 0;
		let max = self.options.len();
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp => {
					if idx > 0 {
						self.draw_old(idx);
						idx -= 1;

						let mut stdout = stdout();
						let _ = stdout.queue(cursor::MoveUp(1));
						let _ = stdout.flush();

						self.draw_new(idx);
					}
				}
				Key::ArrowDown => {
					if idx < max - 1 {
						self.draw_old(idx);
						println!();

						idx += 1;
						self.draw_new(idx);
					}
				}
				Key::Char(' ') => {
					let opt = self.options.get_mut(idx).unwrap();
					opt.toggle();
					self.draw_new(idx);
				}
				Key::Enter => {
					let indices = self
						.options
						.iter()
						.filter(|opt| opt.select)
						.collect::<Vec<_>>();

					self.out(idx, &indices);

					let all = self
						.options
						.into_iter()
						.filter(|opt| opt.select)
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
	fn draw_new(&self, idx: usize) {
		let opt = self.options.get(idx).unwrap();
		let line = opt.select();
		MultiSelect::draw(&line);
	}

	fn draw_old(&self, idx: usize) {
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
	fn init(&self) {
		let mut stdout = stdout();
		let msg = self.message.as_ref().unwrap();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), msg);

		for opt in &self.options {
			let line = opt.unselect();
			println!("{}  {}", style(*chars::BAR).cyan(), line);
		}

		print!("{}", style(*chars::BAR_END).cyan());

		let len = self.options.len() as u16;
		let _ = stdout.queue(cursor::MoveToPreviousLine(len));
		let _ = stdout.flush();
	}

	fn out(&self, idx: usize, values: &[&Opt]) {
		let mut stdout = stdout();

		let _ = stdout.queue(cursor::MoveToColumn(0));
		if idx > 0 {
			let _ = stdout.queue(cursor::MoveUp(idx as u16));
		}

		let _ = stdout.flush();

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
pub fn prompt() -> MultiSelect {
	MultiSelect::new()
}
