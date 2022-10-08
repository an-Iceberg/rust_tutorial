// Loops are used to repeat code until a condition is met

pub fn run()
{
  println!("  Loops");

  let mut counter = 0;

  // Infinite loop
  loop
  {
    counter += 1;
    print!("{} ", counter);

    if counter > 20 { break; }
  }

  println!();

  counter = 1;
  let mut string;

  // While loop (FizzBuzz)
  while counter < 20
  {
    string = String::from("");

    if counter % 3 == 0 { string.push_str("Fizz"); }
    if counter % 5 == 0 { string.push_str("Buzz"); }

    if string.is_empty() { string.push_str(&counter.to_string()) }

    print!("{} ", string);

    counter += 1;
  }

  println!();

  // For range loop
  for x in 1..20
  {
    string = String::from("");

    if x % 3 == 0 { string.push_str("Fizz"); }
    if x % 5 == 0 { string.push_str("Buzz"); }

    if string.is_empty() { string.push_str(&x.to_string()) }

    print!("{} ", string);
  }

  let animals = vec!["Cat", "Dog", "Rabbit", "Ant", "Horse", "Crocodile"];

  for (index, animal) in animals.iter().enumerate() {
    println!("{} -> {}", index, animal)
  }

  println!()
}
