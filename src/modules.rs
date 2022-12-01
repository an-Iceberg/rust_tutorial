pub(crate) mod some_mod
{
  pub(crate) fn print_message()
  {
    println!("{}", "  Packages, Crates and Modules".to_uppercase());

    println!("This is a nested module");

    println!();
  }
}

// Modules
mod front_of_house;

fn eat_at_restaurant()
{
  // Absolute path
  crate::modules::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}

fn serve_order()
{}

// Nested enums and structs
mod back_of_house
{
  pub enum Appetizer
  {
    Soup,
    Salad,
  }

  pub struct Breakfast
  {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast
  {
    pub fn summer(toast: &str) -> Breakfast
    {
      return Breakfast
      {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      };
    }
  }

  fn fix_incorrect_order()
  {
    cook_order();
    super::serve_order();
  }

  fn cook_order()
  {}
}

fn eat_at_restaurant_1()
{
  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
}

fn eat_at_restaurant_2()
{
  let _order_1 = back_of_house::Appetizer::Soup;
  let _order_2 = back_of_house::Appetizer::Salad;
}

// The "use" keyword
fn eat_at_restaurant_3()
{
  use crate::modules::front_of_house::hosting;

  fn eat_at_restaurant_3()
  {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
  }
}

// "use" with "self"
fn eat_at_restaurant_4()
{
  pub use self::front_of_house::hosting;

  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

// Avoiding namespace contamination by brining in the parent into scope
fn a()
{
  use std::fmt;
  use std::io;

  fn function_1() -> fmt::Result
  {
    // --snip--
    return Ok(());
  }

  fn function_2() -> io::Result<()>
  {
    // --snip--
    return Ok(());
  }
}

// Another way is to alias imports
fn b()
{
  use std::fmt::Result;
  use std::io::Result as IoResult;

  fn function_1() -> Result
  {
    // --snip--
    return Ok(());
  }

  pub(self) fn function_2() -> IoResult<()>
  {
    return Ok(());
  }
}

#[allow(unused_imports)]
fn c()
{
  use rand::{Rng, Error, CryptoRng};
  use std::io::{self, Write};
  use std::io::*;

  let _secret_number = rand::thread_rng().gen_range(1..=10);
}
