use std::collections::HashMap;

pub(crate) fn run()
{
  println!("{}", "  Hash Maps".to_uppercase());

  let mut grades = HashMap::new();

  // Add values
  grades.insert("Rust Programming", 5.5);
  grades.insert("Web Development", 5.0);
  grades.insert("UX Design", 4.5);
  grades.insert("Professional Computing Studies", 4.0);
  grades.insert("Databases", 5.0);

  // Size of hash map
  println!("How many subjects have you studied? {}", grades.len());

  // Get a single value
  match grades.get("Web Development")
  {
    Some(mark) => println!("You got {} for Web Dev.", mark),
    None => println!("You didn't study that subject.")
  }

  // Remove an entry
  grades.remove("UX Design");
  println!();

  // Loop through HashMap
  for (subject, mark) in &grades
  {
    println!("For {} you got a {}", subject, mark);
  }

  // Check if an entry exists in the HashMap
  println!("Did you study C#? {}", grades.contains_key("C#"));
}
