pub(crate) const MAXIMUM_NUMBER: u8 = 10;

pub(crate) fn run() {
  println!("{}", "  Constants".to_uppercase());

  for number in 1..=MAXIMUM_NUMBER { print!("{} ", number); }

  println!();
}
