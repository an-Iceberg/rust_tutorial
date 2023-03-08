#![allow(dead_code)]

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
mod type_casting;
mod match_like_a_pro;
mod my_math;
mod generics;

#[path ="../tests/my_math_tests.rs"]
mod my_math_tests;

#[path ="the_rust_book/chapter_1.rs"]
mod chapter_1;
#[path ="the_rust_book/chapter_2.rs"]
mod chapter_2;
#[path ="the_rust_book/chapter_3.rs"]
mod chapter_3;
#[path ="the_rust_book/chapter_4_owndership.rs"]
mod chapter_4_owndership;

#[path ="../tests/tests.rs"]
#[cfg(test)]
mod tests;

// TODO: consider moving all tests into a separate directory "tests"
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
  type_casting::run();
  println!();
  generics::run();
  println!();

  println!("  The Rust Book");
  println!();

  chapter_1::run();
  println!();
  // chapter_2::run();
  println!();
  chapter_3::run();
  println!();
  chapter_4_owndership::run();
  println!();

  match_like_a_pro::run();
}
