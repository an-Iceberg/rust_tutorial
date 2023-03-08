#[cfg(test)]

#[path ="../src/my_math.rs"]
mod my_math;

#[test]
fn my_add_tests()
{
  // Simple maths
  assert_eq!(my_math::my_add(2, 2), 4);
  assert_eq!(my_math::my_add(23, 87), 110);
  assert_eq!(my_math::my_add(19, 52), 71);
}

#[test]
fn my_add_with_negative_numers_tests()
{
  // Negative numbers
  assert_eq!(my_math::my_add(-43, 9), -34);
  assert_eq!(my_math::my_add(-14, 60), 46);
  assert_eq!(my_math::my_add(-4, -54), -58);
}

#[test]
#[should_panic] // TODO: consider reworking the function so that it does not panic
fn my_add_limits_tests()
{
  // i32 limits
  assert_eq!(my_math::my_add(2_147_483_647, 1), 0);
  assert_eq!(my_math::my_add(2_147_483_647, 2), 1);
}
