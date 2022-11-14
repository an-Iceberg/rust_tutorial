pub(crate) const MAXIMUM_NUMBER: u8 = 10;

pub fn run() {
  println!("  Constants");

  for number in 1..=MAXIMUM_NUMBER { print!("{} ", number); }

  println!();
}
