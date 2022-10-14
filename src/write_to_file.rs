use std::{fs::File, io::Write};

pub fn run()
{
  println!("  Writing to a file");

  let mut file = File::create("output.txt").expect("Could not create file");

  file.write_all(b"Welcome to Rust").expect("Could not write to file");

  println!();
}
