pub(crate) struct Name
{
  first: String,
  last:  String,
}

impl Name
{
  pub(crate) fn new(first: &str, last: &str) -> Name
  {
    Name { first: String::from(first), last: String::from(last) }
  }

  pub(crate) fn print(self)
  {
    println!("{} {}", self.first, self.last);
  }
}
