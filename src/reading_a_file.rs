use std::{fs::File, io::Read};

pub(crate) fn run()
{
  println!("{}", "  Reading a file".to_uppercase());

  let mut file = File::open("info.txt").expect("Can't open file");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Can't read file");

  println!("File Contents: {}", contents);

  /* Reading a file byte by byte */
  file = File::open("info.txt").expect("Can't open file");

  for byte in file.bytes()
  {
    match byte
    {
      Ok(ok_byte) => print!("{} ", ok_byte as char),
      Err(error) => print!("{} ", error)
    }
  }
}
