use crate::{error::ClackSelectError, style::chars};
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
	less: Option<u16>,
}

impl MultiSelect {
	pub fn new<S: Into<String>>(message: S) -> Self {
		MultiSelect {
			message: message.into(),
			options: vec![],
			less: None,
		}
	}

	pub fn option<S: Into<String>>(&mut self, val: S, label: S) -> &mut Self {
		// todo duplicate
		let opt = Opt::new(val, label, None);
		self.options.push(opt);
		self
	}

	pub fn option_hint<S: Into<String>>(&mut self, val: S, label: S, hint: S) -> &mut Self {
		let opt = Opt::new(val, label, Some(hint));
		self.options.push(opt);
		self
	}

	pub fn options(&mut self, options: Vec<Opt>) -> &mut Self {
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
	pub fn interact(&self) -> Result<Vec<String>, ClackSelectError> {
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
						let prev = idx;
						let prev_less = less_idx;

						if idx > 0 {
							idx -= 1;
							less_idx = less_idx.saturating_sub(1);
						} else {
							let less = self.less.expect("less should unwrap if is_less");
							idx = max - 1;
							less_idx = less - 1;
						}

						self.draw_less(&options, idx, prev, less_idx, prev_less);
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
						let prev = idx;
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

						self.draw_less(&options, idx, prev, less_idx, prev_less);
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
						self.w_out_less(idx, less_idx, &selected_opts);
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

impl MultiSelect {
	fn draw_focus(&self, options: &[Opt], idx: usize) {
		let opt = options.get(idx).expect("idx should always be in bound");
		let line = opt.focus();
		MultiSelect::draw(&line);
	}

	fn draw_unfocus(&self, options: &[Opt], idx: usize) {
		let opt = options.get(idx).expect("idx should always be in bound");
		let line = opt.unfocus();
		MultiSelect::draw(&line);
	}

	fn draw(line: &str) {
		let mut stdout = stdout();
		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		print!("{}  {}", style(*chars::BAR).cyan(), line);
		let _ = stdout.flush();
	}

	fn draw_less(&self, opts: &[Opt], idx: usize, prev_idx: usize, less_idx: u16, prev_less: u16) {
		let mut stdout = stdout();
		if prev_less > 0 {
			let _ = stdout.queue(cursor::MoveToPreviousLine(prev_less));
		}

		let _ = stdout.queue(cursor::MoveToColumn(0));
		let _ = stdout.flush();

		let less = self.less.expect("less should unwrap if is_less");
		for i in 0..less.into() {
			let prev = prev_idx + i - prev_less as usize;
			let prev_opt = opts.get(prev).unwrap();
			let len = prev_opt.len();
			print!("   {}", " ".repeat(len));

			let _ = stdout.queue(cursor::MoveToColumn(0));
			let _ = stdout.flush();

			let i_idx = idx + i - less_idx as usize;
			let opt = opts.get(i_idx).unwrap();
			let line = opt.unfocus();
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

impl MultiSelect {
	fn w_init_less(&self) {
		println!("{}", *chars::BAR);
		println!("{}  {}", style(*chars::STEP_ACTIVE).cyan(), self.message);

		self.draw_less(&self.options, 0, 0, 0, 0);

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

	fn w_out(&self, idx: usize, selected: &[&Opt]) {
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

		let vals = selected
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

	fn w_out_less(&self, idx: usize, less_idx: u16, selected: &[&Opt]) {
		let mut stdout = stdout();
		if less_idx > 0 {
			let _ = stdout.queue(cursor::MoveToPreviousLine(less_idx));
		}

		// let _ = stdout.queue(cursor::MoveToColumn(0));
		// let _ = stdout.flush();

		let _ = stdout.queue(cursor::MoveToPreviousLine(1));
		let _ = stdout.flush();

		println!("{}  {}", style(*chars::STEP_SUBMIT).green(), self.message);

		let less = self.less.expect("less should unwrap if is_less");
		for i in 0..less.into() {
			let prev = idx + i - less_idx as usize;
			let prev_opt = self.options.get(prev).unwrap();
			let len = prev_opt.len();
			println!("   {}", " ".repeat(len));
		}
		println!("            ");
		println!(" ");

		let mv = less + 2;
		let _ = stdout.queue(cursor::MoveToPreviousLine(mv));

		let vals = selected
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

/// Shorthand for [`MultiSelect::new()`]
pub fn multi_select<S: Into<String>>(message: S) -> MultiSelect {
	MultiSelect::new(message)
}
