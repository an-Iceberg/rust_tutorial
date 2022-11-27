use std::{io::{self, Write}, cmp::Ordering};

use rand::Rng;

/**
  # This is really bad code
*/
pub(crate) fn run()
{
  println!("  Chapter 2");

  let mut user_guess = String::new();
  let secret_number = rand::thread_rng().gen_range(1..=100);

  loop
  {
    print!("Guess: ");

    io::stdout()
      .flush()
      .unwrap()
    ;

    io::stdin().read_line(&mut user_guess).expect("Failed to read line.");

    let guess: u32 = match user_guess.trim().parse()
    {
      Ok(number) => number,
      Err(_) => {
        user_guess = String::from("");
        continue;
      }
    };

    match guess.cmp(&secret_number)
    {
      Ordering::Less => println!("{guess} is too low"),
      Ordering::Greater => println!("{guess} is too high"),
      Ordering::Equal => {
        println!("{guess} is correct");
        break;
      }
    }

    user_guess = String::from("");
  }

  println!("The secret number is {secret_number}");
}
