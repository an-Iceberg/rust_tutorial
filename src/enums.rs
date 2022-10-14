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

/* TODO: this is where we left off: https://www.youtube.com/watch?v=JKmkKae-EhM&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=37 */
