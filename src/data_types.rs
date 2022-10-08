/* Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take up in memory)
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays
 */

pub fn run()
{
  println!("  Data Types");

  // Default is 'i32'
  let number = 1;
  let number2: i64 = 2;
  let num: u32 = 32;

  // Default is 'f64'
  let x = 45e7;

  // Explicit typing
  let y: i64 = 4_572_075_481_982;

  // Max size
  println!("Max i32: {}", std::i32::MAX);

  // Boolean
  let is_active = true;

  // Boolean from expression
  let is_greater = 10 < 5;

  // Char
  let a1 = 'a';
  let rocket = '🚀';

  println!("{:?}", (number, number2, num, x, y, is_active, is_greater, a1, rocket));
}
