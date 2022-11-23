pub fn run()
{
  println!("{}", "  References".to_uppercase());

  let mut x = 10;

  // Immutable references
  let reference_to_x = &x;
  let nr = &x;
  let num = &x;
  let number = &x;

  println!("x: {}\nreference_to_x: {}\nnr: {}\nnum: {}\nnumber: {}", x, reference_to_x, nr, num, number);

  // Mutable reference
  let mutable_reference_to_x = &mut x;
  *mutable_reference_to_x += 11;

  println!("mutable_reference_to_x: {}", mutable_reference_to_x);
  println!("x: {}", x);

  mutable_reference();

  // Pass by referce
  let blue = Color{ red: 0, green: 0, blue: 255 };

  print_color(&blue);

  println!();
}

struct Color
{
  red: u8,
  green: u8,
  blue: u8,
}

fn print_color(color: &Color)
{
  println!("Colour - Red: {}, Green: {}, Blue: {}", color.red, color.green, color.blue);
}

fn mutable_reference()
{
  let mut number = 10;

  {
    let mutable_reference_to_number = &mut number;
    *mutable_reference_to_number += 11;
  }

  println!("number: {}", number);
}
