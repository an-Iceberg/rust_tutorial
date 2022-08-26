/* Tuples group values of different types together
Max 12 elements */

pub fn run()
{
  println!("  Tuples");

  let person: (&str, &str, i8) = ("Brad", "Mass", 37);

  println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}
