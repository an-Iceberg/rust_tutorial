struct Person
{
  name: String,
  age: u8
}

impl ToString for Person
{
  fn to_string(&self) -> String
  {
    return format!("My name is {} and i am {}.", self.name, self.age);
  }
}

trait Print
{
  fn print(&self);
}

impl Print for Person
{
  fn print(&self)
  {
    println!("My name is {} and i am {}.", self.name, self.age);
  }
}

pub fn run()
{
  println!("{}", "  Traits".to_uppercase());

  let kazul = Person{ name: String::from("Kazul"), age: 24 };

  println!("{}", kazul.to_string());
  kazul.print();

  println!();
}
