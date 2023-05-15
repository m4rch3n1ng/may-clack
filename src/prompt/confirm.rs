use console::{Key,Term,style};
use std::io::{stdout,Write};
use crossterm::{cursor, QueueableCommand};
use super::prompt::Prompt;

pub struct Confirm {
	message: Option<String>,
	initial_value: bool,
}

impl Confirm {
	pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
		self.message = Some(msg.into());
		self
	}

	pub fn initial_value ( mut self, b: bool ) -> Self {
		self.initial_value = b;
		self
	}

	// todo: Result
	pub fn interact (self) -> Option<bool> {
		self.init();

		let term = Term::stdout();
		// let _ = term.hide_cursor(); // todo

		let mut a = self.initial_value;
		loop {
			match term.read_key().ok()? {
				Key::ArrowUp | Key::ArrowDown | Key::ArrowLeft | Key::ArrowRight => {
					a = !a;
					self.draw(&a);
				},
				Key::Enter => {
					let _ = term.show_cursor();
					println!();
					self.out(&a);
					return Some(a);
				}
				_ => {}
			}
		};
	}
}

impl Prompt<bool> for Confirm {
	fn init(&self) {
		let mut stdout = stdout();
		let msg = self.message.as_ref().unwrap();

		println!("│");
		println!("{}  {}", style("◆").cyan(), msg);
		println!("{}", style("│").cyan());
		print!("{}", style("└").cyan());

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		self.draw(&self.initial_value);

		let _ = stdout.flush();
	}

	fn out(&self, value: &bool) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToPreviousLine(2));
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let msg = self.message.as_ref().unwrap();
		let answ = if *value { "Yes" } else { "No" };

		println!("{}  {}", style("◇").green(), msg);
		println!("{}  {}{}", "│", style(answ).dim(), " ".repeat(12 - answ.len()));
	}
}

impl Confirm {
	pub fn new() -> Confirm {
		Confirm { message: None, initial_value: false }
	}

	fn radio_pnt ( &self, b: &bool, w: &str ) -> String {
		if *b {
			format!("{} {w}", style("●").green())
		} else {
			style(format!("○ {w}")).dim().to_string()
		}
	}

	fn radio (&self, b: &bool) -> String {
		let yes = self.radio_pnt(b, "Yes");
		let no = self.radio_pnt(&!*b, "No");

		let a = format!("{} / {}", yes, no);
		a
	}

	fn draw ( &self, a: &bool ) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let r = self.radio(a);
		print!("{}  {}", style("│").cyan(), r);
		let _ = stdout.flush();
	}
}

pub fn main() -> Confirm {
	Confirm::new()
}
