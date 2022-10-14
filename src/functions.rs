pub fn run()
{
  println!("  Functions");

  greet("Hello", "World");

  // Bind function values to variables
  let get_sum = add(12, 34);
  println!("{}", get_sum);

  // Closure
  let n3 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure:{}", add_nums(3, 3));

  some_function();
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
fn some_function() -> (i32, i32) { return (32, 64); }
