pub(crate) fn run()
{
  // Print to console
  println!("{}", "  Printing and Formatting".to_uppercase());

  // Basic formatting
  println!("Number: {}", 1);
  println!("{} is from {}", "Someone", "somewhere");

  // Positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

  // Named arguments
  println!("{name} likes to play {activity}", name = "John", activity = "the piano");

  // Placeholder traits
  println!("Binary:{:b} Hex:{:x} Octal:{:o}", 100, 100, 100);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "Hello World"));

  // Basic math
  println!("10 + 23 = {}", 10 + 23);

  // String formatting
  let message = format!("{} {}", "Hello", "World");
  println!("{}", message);
}
