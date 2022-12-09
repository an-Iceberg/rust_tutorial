pub(crate) fn run()
{
  println!("{}", "  String Methods".to_uppercase());

  // Replace
  {
    let my_string = String::from("Rust is fantastic");

    println!("{}", my_string.replace("fanta", "coca-cola"))
  }

  // Lines
  {
    let my_string = String::from("The weather is\nnice\noutside\nm8");

    for line in my_string.lines()
    {
      println!("[ {} ]", line);
    }
  }

  // Split
  {
    let my_string = String::from("Leave+a+like+if+you+enjoyed");
    let tokens: Vec<&str> = my_string.split("+").collect();

    println!("{:?}", tokens);
  }

  // Trim
  {
    let my_string = String::from("      Hello World             \n\r");

    println!("before trim: {}", my_string);
    println!("after trim: {}", my_string.trim());
  }

  // Chars
  {
    let my_string = String::from("Hello World, how are you doing tonight?");

    // Get character at index
    match my_string.chars().nth(11)
    {
      Some(character) => println!("Character at index 11: \"{}\"", character),
      None => println!("No character at index 11")
    }

    // Also possible to do it like this
    if let Some(character) = my_string.chars().nth(11)
    {
      println!("Character at index 11: \"{}\"", character)
    }
    else
    {
      println!("No character at index 11")
    }
  }

  // String formatting
  {
    let number = 74;
    println!("0d{0} {0:#x} {0:#X} {0:#b} {0:#o}", number);
  }

  println!()
}
