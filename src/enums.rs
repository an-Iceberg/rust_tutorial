enum Movement
{
  // Variants
  Up,
  Down,
  Left,
  Right
}

fn move_avatar(movement: Movement)
{
  match movement
  {
    Movement::Up => println!("{}", "UP"),
    Movement::Down => println!("{}", "DOWN"),
    Movement::Left => println!("{}", "LEFT"),
    Movement::Right => println!("{}", "RIGHT")
  }
}

pub fn run()
{
  println!("  Enums");

  let avatar1 = Movement::Left;
  let avatar2 = Movement::Up;
  let avatar3 = Movement::Down;
  let avatar4 = Movement::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
