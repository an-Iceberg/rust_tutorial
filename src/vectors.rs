pub(crate) fn run()
{
  println!("{}", "  Vectors".to_uppercase());

  let my_vector: Vec<i32> = Vec::new();
  println!("{:?}", my_vector);

  let mut numbers: Vec<i16> = vec![29, 72, 14, 147, 20, 92];

  numbers.push(5);
  numbers.push(200);
  numbers.pop();

  println!("Vector:{:?}", numbers);
  println!("Length:{}", numbers.len());
  println!("Vector occupies {} bytes in memory", std::mem::size_of_val(&numbers));

  // Slices also work on vectors

  // Loop over Vector
  for value in numbers.iter() { print!("{} ", value); }
  println!();

  // Change vector values
  for value in numbers.iter_mut() { *value *= 2; }
  println!("Mutated Vector:{:?}", numbers);
}
