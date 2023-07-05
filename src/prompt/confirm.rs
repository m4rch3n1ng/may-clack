use crate::style::chars;

use super::traits::Prompt;
use console::{style, Key, Term};
use crossterm::{cursor, QueueableCommand};
use std::io::{stdout, Write};

pub struct Confirm {
	message: Option<String>,
	initial_value: bool,
	prompts: (String, String),
}

impl Default for Confirm {
	fn default() -> Self {
		Self::new()
	}
}

impl Confirm {
	pub fn new() -> Confirm {
		Confirm {
			message: None,
			initial_value: false,
			prompts: ("Yes".into(), "No".into()),
		}
	}

	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	pub fn initial_value(mut self, b: bool) -> Self {
		self.initial_value = b;
		self
	}

	pub fn prompts<S: Into<String>>(mut self, yes: S, no: S) -> Self {
		self.prompts = (yes.into(), no.into());
		self
	}

	// todo: Result
	pub fn interact(self) -> Option<bool> {
		self.init();

		let term = Term::stdout();
		// let _ = term.hide_cursor(); // todo

		let mut a = self.initial_value;
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight => {
					a = !a;
					self.draw(a);
				}
				Key::Enter => {
					let _ = term.show_cursor();
					println!();
					self.out(a);
					return Some(a);
				}
				_ => {}
			}
		}
	}
}

impl Confirm {
	fn radio_pnt(b: bool, w: &str) -> String {
		if b {
			format!("{} {}", style(*chars::RADIO_ACTIVE).green(), w)
		} else {
			style(format!("{} {}", *chars::RADIO_INACTIVE, w))
				.dim()
				.to_string()
		}
	}

	fn radio(&self, b: bool) -> String {
		let yes = Confirm::radio_pnt(b, &self.prompts.0);
		let no = Confirm::radio_pnt(!b, &self.prompts.1);

		format!("{} / {}", yes, no)
	}

	fn draw(&self, a: bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let r = self.radio(a);
		print!("{}  {}", style("│").cyan(), r);
		let _ = stdout.flush();
	}
}

impl Prompt<bool> for Confirm {
	fn init(&self) {
		let mut stdout = stdout();
		let msg = self.message.as_ref().unwrap();

		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), msg);
		println!("{}", style(*chars::BAR).cyan());
		print!("{}", style(*chars::BAR_END).cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		self.draw(self.initial_value);

		let _ = stdout.flush();
	}

	fn out(&self, value: bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let msg = self.message.as_ref().unwrap();
		let answ = if value {
			&self.prompts.0
		} else {
			&self.prompts.1
		};

		let len = 2 + self.prompts.0.chars().count() + 3 + 2 + self.prompts.1.chars().count();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), msg);
		println!(
			"{}  {}{}",
			*chars::BAR,
			style(answ).dim(),
			" ".repeat(len - answ.len())
		);
	}
}

#[must_use]
pub fn prompt() -> Confirm {
	Confirm::new()
}
