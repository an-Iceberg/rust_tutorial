pub(crate) fn run()
{
  println!("{}", "  Type Casting".to_uppercase());

  let integer: i32 = 34;

  // Type casting can be performed with the "as" keyword
  println!("u32 to u16: {}", integer as u16);
  println!("or as a character: {}", integer as u8 as char);
}
