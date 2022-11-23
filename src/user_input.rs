use std::io::stdin;

pub fn run()
{
  println!("{}", "  Reading user input".to_uppercase());

  let mut input = String::new();

  println!("User input: ");

  match stdin().read_line(&mut input)
  {
    Ok(_) => println!("Success! Input was: {}", input),
    Err(error) => println!("Error: {}", error),
  }
}
