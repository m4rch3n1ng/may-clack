//! this is a rust port of <https://www.npmjs.com/package/@clack/prompts>
//!
//! ## Setup
//!
//! you can setup the start and end of a prompt session with [`intro`] and [`outro`], respectively
//!  
//! ```
//! use may_clack::{intro, outro};
//!
//! intro("intro");
//! // do stuff
//! outro("outro");
//! ```
//!
//! ## Cancel
//!
//! when the user cancels a question, you can use the [`cancel`] utility to provide a cancellation message.
//!   
//! when cancelled will return a [`error::ClackError::Cancelled`], a [`error::ClackInputError::Cancelled`]
//! or a [`error::ClackSelectError::Cancelled`], depending on input type, Err in a Result.
//!
//! all input types that can return a `Cancelled` Err will also have the option to add a `.cancel` callback
//!
//! ```no_run
//! use may_clack::{cancel, input, error::ClackInputError};
//!
//! let text = input("todo").interact();
//! if let Err(ClackInputError::Cancelled) = text {
//!     cancel("operation cancelled")
//! }
//! ```
//!
//! ## General
//!
//! there are 6 components: [input](#input), [confirm](#confirm),
//! [select](#select), [multi_select](#multi_select), [multi_input](#multi_input)
//!
//! each of the input types returns a struct, that allows you to setup the prompt.  
//! since every prompt needs a message the initial
//!
//! to actually prompt the user after setting up you have to call `.interact()`
//!
//! ```no_run
//! use may_clack::confirm;
//!
//! let answer = confirm("Yes or No?").interact();
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
//! let answer = input("what is the meaning of life?").initial_value("42").interact();
//! println!("{:?}", answer);
//! ```
//!
//! ## Confirm
//!
//! The [`confirm::Confirm`] component accepts a yes or no answer.
//!
//! ```no_run
//! use may_clack::confirm;
//!
//! let answer = confirm("do you want to continue?").interact();
//! println!("answer {:?}", answer);
//! ```
//!
//! ## Select
//!
//! The [`select::Select`] component allows the user to choose one value from a list of options.
//!
//! ```no_run
//! use may_clack::select;
//! 
//! let fruit = select("Pick a fruit")
//!     .option_hint("mango", "Mango", "The best one")
//!     .option("peach", "Peach")
//!     .option("passionfruit", "Passion fruit")
//!     .interact();
//! println!("fruit {:?}", fruit);
//! ```
//! 
//! ## MultiSelect
//! 
//! The [`multi_select::MultiSelect`] component allows the user to choose multiple values from a list of options.
//! 
//! ```no_run
//! use may_clack::multi_select;
//! 
//! let toppings = multi_select("Choose your toppings")
//!     .option("fruits", "Dried fruits")
//!     .option("chocolate", "Chocolate Chips")
//!     .option_hint("idk", "idk", "idk")
//!     .interact();
//! println!("toppings {:?}", toppings);
//! ```
//! 
//! ## MultiInput
//! 
//! The [`multi_input::MultiInput`] component accepts multiple lines of text.
//! 
//! ```no_run
//! use may_clack::multi_input;
//! 
//! let lines = multi_input("idk").interact();
//! println!("lines {:?}", lines);
//! ```
//! 

pub mod error;
mod prompt;
pub mod style;

pub use prompt::*;

pub use prompt::confirm::confirm;
pub use prompt::input::input;
pub use prompt::multi_input::multi_input;
pub use prompt::multi_select::multi_select;
pub use prompt::select::select;
