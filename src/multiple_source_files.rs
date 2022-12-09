/**
Public function
*/
pub(crate) fn run()
{
  println!("{}", "  Multiple Source Files".to_uppercase());

  print_something();

  println!();
}

/**
By default, all functions of a module are private
*/
fn print_something()
{
  println!("Hello World")
}
