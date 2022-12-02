use regex::Regex;

pub(crate) fn run()
{
  println!("{}", "  Regular Expressions".to_uppercase());

  // Matches a 5 letter word
  let five_letter_word = Regex::new(r"(\w{5})").unwrap();
  let text = "dcode";

  println!("Found match? {}", five_letter_word.is_match(text));

  match five_letter_word.captures(text)
  {
    Some(captures) =>
      println!("Found match: {}", captures.get(0).unwrap().as_str()), // same as &captures[0]

    None =>
      println!("Could not find a match")
  }

  println!();
}
