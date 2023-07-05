use super::traits::Prompt;
use crate::style::chars;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

struct Opt {
	pub value: String,
	pub label: String,
	pub select: bool,
}

impl Opt {
	pub fn new<S: Into<String>>(val: S, label: S) -> Self {
		Opt {
			value: val.into(),
			label: label.into(),
			select: false,
		}
	}

	pub fn toggle(&mut self) {
		self.select = !self.select;
	}

	pub fn len(&self) -> usize {
		let len1 = self.label.len();
		let len2 = chars::CHECKBOX_INACTIVE.len();
		len1 + 1 + len2
	}

	fn select(&self) -> String {
		if self.select {
			format!(
				"{} {}",
				style(*chars::CHECKBOX_SELECTED).green(),
				self.label
			)
		} else {
			format!("{} {}", style(*chars::CHECKBOX_ACTIVE).cyan(), self.label)
		}
	}

	fn unselect(&self) -> String {
		if self.select {
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

pub struct MultiSelect {
	message: Option<String>,
	options: Vec<Opt>,
}

impl Default for MultiSelect {
	fn default() -> Self {
		MultiSelect::new()
	}
}

impl MultiSelect {
	#[must_use]
	pub fn new() -> Self {
		MultiSelect {
			message: None,
			options: vec![],
		}
	}

	#[must_use]
	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	#[must_use]
	pub fn option<S: Into<String>>(mut self, val: S, label: S) -> Self {
		// todo duplicate
		let opt = Opt::new(val, label);
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
					if idx < max {
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
					let indices = self.options.iter().filter(|opt| opt.select).collect();
					self.out((idx, indices));

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

impl Prompt<(usize, Vec<&Opt>)> for MultiSelect {
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

	fn out(&self, (idx, values): (usize, Vec<&Opt>)) {
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
