pub fn run()
{
  println!("  Pointers and References");

  // Primitive array
  let array1 = [45, 12, 95, 28];
  let array2 = array1;

  println!("Arrays:{:?}", (array1, array2));

  // Vector
  let vec1 = vec![12, 39, 82, 29];
  let vec2 = &vec1;

  println!("Vectors:{:?}", (&vec1, vec2));
}
