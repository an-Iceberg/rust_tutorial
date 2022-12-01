pub(crate) fn run()
{
  println!("{}", "  Options".to_uppercase());

  let name = String::from("John");

  println!("Character at index 8: {}",
    match name.chars().nth(8)
    {
      Some(character) => character.to_string(),
      None => "No character at index 8".to_string()
    }
  );

  println!("John is a {}", match get_occupation("John")
    {
      Some(occupation) => occupation,
      None => "This person has no ocupation"
    }
  );

  match get_occupation("Michael")
  {
    Some(occupation) =>
    {
      println!("Michael is a {}", occupation);
      let some_int = 10;
      println!("some integer: {}", some_int);
    }
    None =>
    {
      println!("Michael does not have an occupation");
    }
  }

  println!();
}

fn get_occupation(name: &str) -> Option<&str>
{
  return match name
  {
    "John" => Some("Software Developer"),
    "Michael" => Some("Dentist"),
    _ => None
  };
}
