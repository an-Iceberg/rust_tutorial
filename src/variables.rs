/* Variables hold primitive data or references to data
Variables are immutable by default
Rust is a block-scoped language */

pub fn run()
{
  println!("  Variables");

  let name = "Jack";
  let mut age = 24;

  println!("My name is {_name} and I am {_age}.", _name = name, _age = age);

  age = 25;

  println!("My name is {_name} and I am {_age}.", _name = name, _age = age);

  // Constants
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assigning multiple variables at once
  let (my_name, my_age) = ("Brad", 37);
  println!("My name is {} and I am {}.", my_name, my_age);

  // Large numbers can be written like so
  let some_large_number = 12_345;
  println!("large number: {}", some_large_number);
}
