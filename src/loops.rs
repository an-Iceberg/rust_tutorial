// Loops are used to repeat code until a condition is met

pub(crate) fn run()
{
  println!("{}", "  Loops".to_uppercase());

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

  println!();

  let animals = vec!["Cat", "Dog", "Rabbit", "Ant", "Horse", "Crocodile"];

  for (index, animal) in animals.iter().enumerate()
  {
    println!("{} -> {}", index, animal);
  }

  // Ranges
  // This ranges from 0 to 10 exclusive
  for number in 0..10
  {
    print!("{} ", number);
  }

  println!();

  // This ranges from 0 to 10 inclusive
  for number in 0..=10
  {
    print!("{} ", number);
  }

  println!();

  // This ranges with a custom step
  for number in (0..=10).step_by(2)
  {
    print!("{} ", number);
  }

  println!();
}
