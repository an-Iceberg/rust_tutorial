mod print_and_formatting;
mod variables;
mod data_types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;
mod constants;
mod shadowing;
mod references;
mod traits;
mod reading_a_file;
mod write_to_file;
mod match_keyword;
mod user_input;
mod hash_maps;
mod random_numbers;
mod string_methods;
mod multiple_source_files;
mod regex;
mod modules;
mod option;

fn main()
{
  // Comments

  /* This is also a comment
  A multiline comment */

  println!();
  print_and_formatting::run();
  println!();
  variables::run();
  println!();
  data_types::run();
  println!();
  strings::run();
  println!();
  tuples::run();
  println!();
  arrays::run();
  println!();
  vectors::run();
  println!();
  conditionals::run();
  println!();
  loops::run();
  println!();
  functions::run();
  println!();
  pointers::run();
  println!();
  structs::run();
  println!();
  enums::run();
  println!();
  constants::run();
  println!();
  shadowing::run();
  println!();
  references::run();
  println!();
  traits::run();
  println!();
  reading_a_file::run();
  println!();
  write_to_file::run();
  println!();
  match_keyword::run();
  println!();
  user_input::run();
  println!();
  hash_maps::run();
  println!();
  random_numbers::run();
  println!();
  string_methods::run();
  println!();
  multiple_source_files::run();
  println!();
  regex::run();
  println!();
  modules::some_mod::print_message();
  println!();
  option::run();
  println!();

  /* Tests */

  println!("Hello, world!");
}

struct Rectangle {
  width: u8,
  height: u8
}

impl Rectangle {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn give_two() -> i32 {
  2
}

#[cfg(test)]
mod tests {
  use crate::{shadowing, Rectangle};

  #[test]
  fn basic_test() {
    assert!(1 == 1);
  }

  #[test]
  #[should_panic]
  fn fail_test() {
    panic!("Oh no!");
  }

  #[test]
  fn test_equals() {
    assert_eq!(super::give_two(), 1 + 1);
    assert_ne!(3, 1 + 1);
    shadowing::run();
  }

  #[test]
  #[ignore = "reason"]
  fn ignore_me() {
    panic!("You didn't ignore me!");
  }

  #[test]
  #[should_panic]
  fn test_struct() {
    let rect = Rectangle { width: 50, height: 25 };

    assert!(rect.is_square());
  }
}

/* TODO: this is where we left off => https://www.youtube.com/watch?v=OX9HJsJUDxA&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8 */
