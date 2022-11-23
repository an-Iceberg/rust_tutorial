pub fn run()
{
  println!("{}", "  Match keyword".to_uppercase());

  let number = 2;

  match number {
    2 => println!("{}", "2"),
    10 | 11 => println!("{}", "10 or 11"),
    2..=20 => println!("{}", "between 2 and 20"),
    _ => println!()
  }

  let name = "John";

  match name {
    "Chris" => println!(""),
    "Dominic" => println!("Boo, you suck"),
    _ => println!("yeh")
  }

  println!();
}
