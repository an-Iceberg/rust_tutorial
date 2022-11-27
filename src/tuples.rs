/* Tuples group values of different types together
Max 12 elements */

pub(crate) fn run()
{
  println!("{}", "  Tuples".to_uppercase());

  let person: (&str, &str, i8) = ("Brad", "Mass", 37);

  let touple = ("First Name", "Last Name", ("Street Name", 12), ("City Name", 1234));

  let (first_name, last_name, street_name, city_name) = (touple.0, touple.1, touple.2.0, (touple.3).0);

  println!("{} is from {} and is {} years old.", person.0, person.1, person.2);

  println!("{}\n{}\n{} {}\n{} {}", touple.0, touple.1, (touple.2).0, touple.2.1, touple.3.0, (touple.3).1);

  println!("first_name: {}, last_name: {}, street_name: {}, city_name: {}", first_name, last_name, street_name, city_name);

  // Destructuring
  let (first_name, last_name, age) = person;

  println!("{first_name} {last_name} {age}");
}
