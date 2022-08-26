/* Arrays are a fixed list where elements are of the same data type */

use std::mem;

pub fn run()
{
  println!("  Arrays and Slices");

  let mut numbers: [i16; 5] = [11, 84, 39, 93, 149];

  // Re-assign value
  numbers[0] = 20;

  println!("{:?}", numbers);

  // Get a single array element
  println!("Single Value:{:?}", numbers[0]);

  // Array length
  println!("Length:{}", numbers.len());

  // Amount of memory
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i16] = &numbers[1..3];
  println!("Slice:{:?}", slice);
}
