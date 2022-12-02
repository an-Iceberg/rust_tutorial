use crate::constants::MAXIMUM_NUMBER;

pub(crate) fn run()
{
  println!("{}", "  Functions".to_uppercase());

  greet("Hello", "World");

  // Bind function values to variables
  let get_sum = add(12, 34);
  println!("{}", get_sum);

  // Closure
  let n3 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure:{}", add_nums(3, 3));

  let some_result = some_function();
  println!("{} {}", some_result.0, some_result.1);

  println!("Nested function definitions");
  do_something();

  println!();
}

/**
 * Greets someone with the specified greeting
 */
fn greet(greeting: &str, name: &str)
{
  println!("{} {}, nice to meet you.", greeting, name);
}

// The last line without a semicolon is going to be considered the return value in Rust
fn add(n1: i32, n2: i32) -> i32 { n1 + n2 }

// Apparently this is legal?
fn some_function() -> (i32, String)
{
  return (32, String::from("64"));
}

fn do_something()
{
  println!("do something");
  do_something_else();
  do_something_else_again();

  fn do_something_else()
  {
    println!("do something else");
  }

  fn do_something_else_again()
  {
    println!("do something else again");
  }

  println!("MAXIMUM_NUMBER from a different file: {}", MAXIMUM_NUMBER);
}
