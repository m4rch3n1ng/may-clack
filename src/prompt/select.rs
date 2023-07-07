use crate::style::chars;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
struct Opt {
	value: String,
	label: String,
	hint: Option<String>,
}

impl Opt {
	pub fn new<S: Into<String>>(value: S, label: S, hint: Option<S>) -> Self {
		Opt {
			value: value.into(),
			label: label.into(),
			hint: hint.map(|hint| hint.into()),
		}
	}

	pub fn select(&self) -> String {
		let fmt = format!("{} {}", style(*chars::RADIO_ACTIVE).green(), self.label);

		if let Some(hint) = &self.hint {
			let hint = format!("({})", hint);
			format!("{} {}", fmt, hint)
		} else {
			fmt
		}
	}

	pub fn unselect(&self) -> String {
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

	pub fn len(&self) -> usize {
		let check_len = chars::RADIO_ACTIVE.len();
		let label_len = self.label.len();
		let hint_len = self.hint.as_ref().map_or(0, |hint| hint.len() + 1 + 2);

		check_len + 1 + label_len + hint_len
	}
}

#[derive(Debug, Clone)]
pub struct Select {
	message: String,
	options: Vec<Opt>,
}

impl Select {
	#[must_use]
	pub fn new<S: Into<String>>(message: S) -> Self {
		Select {
			message: message.into(),
			options: vec![],
		}
	}

	#[must_use]
	pub fn option<S: Into<String>>(mut self, value: S, label: S) -> Self {
		let opt = Opt::new(value, label, None);
		self.options.push(opt);
		self
	}

	#[must_use]
	pub fn option_hint<S: Into<String>>(mut self, value: S, label: S, hint: S) -> Self {
		let opt = Opt::new(value, label, Some(hint));
		self.options.push(opt);
		self
	}

	#[must_use]
	pub fn interact(self) -> Option<String> {
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
				Key::ArrowUp | Key::ArrowLeft => {
					if idx > 0 {
						self.draw_old(idx);
						idx -= 1;

						let mut stdout = stdout();
						let _ = stdout.queue(cursor::MoveUp(1));
						let _ = stdout.flush();

						self.draw_new(idx);
					}
				}
				Key::ArrowDown | Key::ArrowRight => {
					if idx < max - 1 {
						self.draw_old(idx);
						println!();

						idx += 1;
						self.draw_new(idx);
					}
				}
				Key::Enter => {
					self.out(idx);

					let opt = self.options.get(idx).cloned().unwrap();
					return Some(opt.value);
				}
				_ => {}
			}
		}
	}
}

impl Select {
	fn draw_new(&self, idx: usize) {
		let opt = self.options.get(idx).unwrap();
		let line = opt.select();
		Select::draw(&line);
	}

	fn draw_old(&self, idx: usize) {
		let opt = self.options.get(idx).unwrap();
		let line = opt.unselect();
		Select::draw(&line);
	}

	fn draw(line: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}
}

impl Select {
	fn init(&self) {
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

	fn out(&self, idx: usize) {
		let mut stdout = stdout();

		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.queue(cursor::MoveUp(idx as u16 + 1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		for opt in &self.options {
			let len = opt.len();
			println!("   {}", " ".repeat(len));
		}
		println!(" ");

		let mv = self.options.len() as u16 + 1;
		let _ = stdout.queue(cursor::MoveUp(mv));

		let label = self.options.get(idx).cloned().unwrap().label;
		println!("{}  {}", *chars::BAR, style(label).dim());
	}
}

#[must_use]
pub fn prompt<S: Into<String>>(message: S) -> Select {
	Select::new(message)
}
