use rand::Rng;

pub fn run()
{
  println!("{}", "  Random Numbers".to_uppercase());

  let random_number = rand::thread_rng().gen_range(10..=50);

  println!("Random number: {}", random_number);

  let random_boolean = rand::thread_rng().gen_bool(0.5);

  println!("Random boolean: {}", random_boolean);

  println!();
}
