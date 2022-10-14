use std::{fs::File, io::Read};

pub fn run()
{
  println!("  Reading a file");

  let mut file = File::open("info.txt").expect("Can't open file");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Can't read file");

  println!("File Contents: {}", contents);
}
