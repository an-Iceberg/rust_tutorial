use std::{fs::File, io::Write};

pub(crate) fn run()
{
  println!("{}", "  Writing to a file".to_uppercase());

  let mut file = File::create("output.txt").expect("Could not create file");

  file.write_all(b"Welcome to Rust\n").expect("Could not write to file");

  match file.write(b"Hello")
  {
    Ok(_) => {},

    Err(error) =>
      println!("{}", error)
  }

  println!();
}
