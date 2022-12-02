#[path ="name.rs"]
mod name;
#[path ="address.rs"]
mod address;

pub(crate) struct Person
{
  name:    name::Name,
  height:  u8,
  weight:  u16,
  gender:  String,
  address: address::Address,
}

impl Person
{
  pub(crate) fn new(first: &str, last: &str, height: u8, weight: u16, gender: &str, street: &str, house_number: u32, zip_code: u32, city: &str) -> Person
  {
    return Person {
      name: name::Name::new(first, last),
      height,
      weight,
      gender: String::from(gender),
      address: address::Address::new(street, house_number, zip_code, city),
    };
  }

  pub(crate) fn print(self)
  {
    self.name.print();
    println!("{} cm", self.height);
    println!("{} kg", self.weight);
    println!("{}", self.gender);
    self.address.print();
  }
}
