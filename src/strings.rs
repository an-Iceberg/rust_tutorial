/* Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or own string data */

pub fn run()
{
  println!("{}", "  Strings".to_uppercase());

  let hello = "Hello";

  let mut hello_world : String = String::from("Hello World");
  let some_string = String::from(format!("some number: {}", 13));
  let some_other_string = format!("some other number: {}", 84);

  println!("{some_string}");
  println!("{}", some_other_string);

  // Get string length
  println!("Length:{}", hello.len());
  println!("Length:{}", hello_world.len());

  // Append character
  hello_world.push('!');

  // Append string
  hello_world.push_str(hello);

  // Capacity in bytes
  println!("Capacity:{}", hello_world.capacity());

  // Check if empty
  println!("Is Empty:{}", hello_world.is_empty());

  // Contains a substring
  println!("Contains 'World':{}", hello_world.contains("World"));

  // Replace
  println!("Replace 'W' with 'w':{}", hello_world.replace("W", "w"));

  // Loop over string by whitespace
  for word in hello_world.split_whitespace() { println!("{word}"); }

  // Create string with capacity
  let mut some_string = String::with_capacity(10);
  some_string.push('a');
  some_string.push('b');
  println!("{some_string}");

  // Assertions
  assert_eq!(some_string.len(), 2);
  assert_eq!(some_string.capacity(), 10);

  println!("{hello} {hello_world}");
}
