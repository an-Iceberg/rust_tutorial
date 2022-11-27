#![allow(dead_code)]

enum Direction
{
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(movement: Direction)
{
  match movement
  {
    Direction::Up => println!("{}", "UP"),
    Direction::Down => println!("{}", "DOWN"),
    Direction::Left => println!("{}", "LEFT"),
    Direction::Right => println!("{}", "RIGHT")
  }
}

enum Day
{
  Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl Day
{
  fn is_weekday(&self) -> bool
  {
    match self
    {
      &Day::Saturday | &Day::Sunday => return false,
      _ => return true
    }
  }

  fn is_weekend(&self) -> bool
  {
    match self
    {
      &Day::Saturday | &Day::Sunday => return true,
      _ => return false
    }
  }
}

pub(crate) fn run()
{
  println!("{}", "  Enums".to_uppercase());

  let avatar1:Direction = Direction::Left;
  let avatar2 = Direction::Up; // Shorthand
  let avatar3 = Direction::Down;
  let avatar4 = Direction::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);

  let day = Day::Tuesday;

  println!("Is Tuesday a weekday? {}", day.is_weekday());
}
