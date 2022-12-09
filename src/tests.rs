#[cfg(test)]

use crate::shadowing;
// use crate::my_math;

#[test]
fn basic_test()
{
  assert!(1 == 1);
}

#[test]
#[should_panic]
fn fail_test()
{
  panic!("Oh no!");
}

#[test]
fn test_equals()
{
  assert_eq!(give_two(), 1 + 1);
  assert_ne!(3, 1 + 1);
  shadowing::run();
}

#[test]
#[ignore = "reason"]
fn ignore_me()
{
  panic!("You didn't ignore me!");
}

#[test]
#[should_panic]
fn test_struct()
{
  let rect = Rectangle { width: 50, height: 25 };

  assert!(rect.is_square());
}

struct Rectangle
{
  width: u8,
  height: u8,
}

impl Rectangle
{
  fn is_square(&self) -> bool
  {
    return self.width == self.height;
  }
}

fn give_two() -> i32
{
  return 2;
}
