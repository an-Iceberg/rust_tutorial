/* Public funtion */
pub fn run()
{
  println!("  Multiple Source Files");

  print_something();

  println!();
}

/* By default, all functions of a module are private */
fn print_something()
{
  println!("Hello World")
}
