//! This is a rust port of <https://www.npmjs.com/package/@clack/prompts>
//!
//! ## Setup
//!
//! You can setup the start and end of a prompt session with the macros [`intro!`] and [`outro!`], respectively
//!  
//! ```
//! use may_clack::{intro, outro};
//!
//! intro!("intro");
//! // do stuff
//! outro!("outro");
//! ```
//!
//! ## Cancel
//!
//! When the user cancels a question, you can use the [`cancel!`] utility to provide a cancellation message.
//!
//! When cancelled the will return a [`error::ClackError::Cancelled`],
//! or you can check if it was cancelled using the [`traits::IsCancel`] trait extension.
//!
//! All input types that can return a `Cancelled` Err will also have the option to add a `.cancel` closure
//!
//! ```no_run
//! use may_clack::{cancel, error::ClackError, input};
//!
//! let text = input("todo").interact();
//! if let Err(ClackError::Cancelled) = text {
//!     cancel!("operation cancelled");
//! }
//! ```
//!
//! ```
//! use may_clack::{cancel, input, traits::IsCancel};
//! let text = input("todo").interact();
//! if text.is_cancel() {
//!     cancel!("operation cancelled");
//! }
//! ```
//!
//! ## Info
//!
//! If you want to write a message in a prompting session you can use the [`info!`], [`warn!`] or [`err!`] utility.
//!
//! ```
//! use may_clack::{err, info, intro, outro, warn};
//!
//! intro!("intro");
//! // do stuff
//! info!("info");
//! // do stuff
//! warn!("warn");
//! // do stuff
//! err!("err");
//! // do stuff
//! outro!("outro");
//! ```
//!
//! ## General
//!
//! There are 6 components: [`input`](#input), [`confirm`](#confirm),
//! [`select`](#select), [`multi_select`](#multi_select), [`multi_input`](#multi_input)
//!
//! Each of the input types returns a struct, that allows you to setup the prompt.  
//! since every prompt needs a message the initial
//!
//! To actually prompt the user after setting up you have to call `.interact()`
//!
//! ```no_run
//! use may_clack::confirm;
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let answer = confirm("Yes or No?").interact()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Components
//!
//! ## Input
//!
//! The [`input::Input`] component accepts a single line of text.
//!
//! ```no_run
//! use may_clack::input;
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let answer = input("what is the meaning of life?")
//!     .initial_value("42")
//!     .interact()?;
//! println!("{:?}", answer);
//! # Ok(())
//! # }
//! ```
//!
//! ## Confirm
//!
//! The [`confirm::Confirm`] component accepts a yes or no answer.
//!
//! ```no_run
//! use may_clack::confirm;
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let answer = confirm("do you want to continue?").interact()?;
//! println!("answer {:?}", answer);
//! # Ok(())
//! # }
//! ```
//!
//! ## `Select`
//!
//! The [`select::Select`] component allows the user to choose one value from a list of options.
//!
//! ```no_run
//! use may_clack::select;
//!
//! #[derive(Debug, Clone)]
//! enum Fruit {
//!     Mango,
//!     Peach,
//!     PassionFruit,
//! }
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let fruit = select("pick a fruit")
//!     .option_hint(Fruit::Mango, "Mango", "The best one")
//!     .option(Fruit::Peach, "Peach")
//!     .option(Fruit::PassionFruit, "Passion fruit")
//!     .interact()?;
//! println!("fruit {:?}", fruit);
//! # Ok(())
//! # }
//! ```
//!
//! ## `MultiSelect`
//!
//! The [`multi_select::MultiSelect`] component allows the user to choose multiple values from a list of options.
//!
//! ```no_run
//! use may_clack::multi_select;
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let toppings = multi_select("Choose your toppings")
//!     .option("fruits", "Dried fruits")
//!     .option("chocolate", "Chocolate chips")
//!     .option_hint("sauce", "Chocolate sauce", "it's warm")
//!     .interact()?;
//! println!("toppings {:?}", toppings);
//! # Ok(())
//! # }
//! ```
//!
//! ## `MultiInput`
//!
//! The [`multi_input::MultiInput`] component accepts multiple lines of text.
//!
//! ```no_run
//! use may_clack::multi_input;
//!
//! # fn main() -> Result<(), may_clack::error::ClackError> {
//! let lines = multi_input("idk").interact()?;
//! println!("lines {:?}", lines);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

pub mod error;
mod prompt;
pub mod style;
pub mod traits;

pub use prompt::*;

pub use prompt::confirm::confirm;
pub use prompt::input::input;
pub use prompt::multi_input::multi_input;
pub use prompt::multi_select::multi_select;
pub use prompt::select::select;

#[doc(hidden)]
pub use owo_colors;
