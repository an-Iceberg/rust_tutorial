pub(crate) struct Address
{
  street:       String,
  house_number: u32,
  zip_code:     u32,
  city:         String,
}

impl Address
{
  pub(crate) fn new(street: &str, house_number: u32, zip_code: u32, city: &str) -> Address
  {
    Address { street: String::from(street), house_number, zip_code, city: String::from(city) }
  }

  pub(crate) fn print(self)
  {
    println!("{} {}", self.street, self.house_number);
    println!("{} {}", self.zip_code, self.city)
  }
}
