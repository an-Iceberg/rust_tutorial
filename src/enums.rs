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

pub fn run()
{
  println!("  Enums");

  let avatar1:Direction = Direction::Left;
  let avatar2 = Direction::Up; // Shorthand
  let avatar3 = Direction::Down;
  let avatar4 = Direction::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
