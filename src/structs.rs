#[path = "person/person.rs"]
mod person;

// Traditional struct
struct Color
{
  r: u8,
  g: u8,
  b: u8
}

// Tuple struct
struct TupleColor(u8, u8, u8);

struct Vector2d
{
  x: i32,
  y: i32
}

impl Vector2d
{
  // This is the shorthand struct initialisation
  fn new(x: i32, y: i32) -> Vector2d { Vector2d{x, y} }

  fn to_string(&self) -> String { format!("({}, {})", self.x, self.y) }

  fn print(&self) { println!("({}, {})", self.x, self.y) }

  fn add_scalar(&mut self, scalar: i32)
  {
    self.x += scalar;
    self.y += scalar;
  }
}

pub(crate) fn run()
{
  println!("{}", "  Structs".to_uppercase());

  let mut purple = Color{ r: 128, g: 128, b: 255 };

  purple.g = 0;

  println!("Purple: {} {} {}", purple.r, purple.g, purple.b);

  let pink = TupleColor(255, 0, 128);

  println!("Pink: {} {} {}", pink.0, pink.1, pink.2);

  let mut vec1 = Vector2d::new(23, 49);
  println!("{}", vec1.to_string());

  vec1.add_scalar(34);

  println!("{}", vec1.to_string());
  vec1.print();

  println!("Creating a complex composite type");

  let some_person = person::Person::new("Knight", "Kazul", 178, 82, "unknown", "Streetstreet", 28, 998877, "Blighttown");

  some_person.print();
}
