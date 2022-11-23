// Conditionals are used to check the condition of something and act on the result

pub fn run()
{
  println!("{}", "  Conditionals".to_uppercase());

  let age: u8 = 17;
  let check_id = false;
  let knows_person_of_age = false;

  // If/Else
  if (age >= 18 && check_id) || knows_person_of_age
  {
    println!("Bartender: What would you like to drink");
  }
  else if age < 18 && check_id
  {
    println!("Bartender: Sorry, no drink for you");
  }
  else
  {
    println!("Bartender: I need to see your ID")
  }

  // Shorthand if
  let is_of_age = if age >= 18 { true } else { false };
  println!("Person is of age: {}", is_of_age);
}
