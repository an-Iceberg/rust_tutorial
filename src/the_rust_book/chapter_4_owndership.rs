pub(crate) fn run()
{
  println!("  Chapter 4: Ownership");

  /* ----- Ownership rules -----
  1. Each value in Rust has a variable that's called its owner.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.
  */

  // s is not valid here, it has not yet been declared

  {
    let _s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
  }

  // this scope is now over and s is no longer valid

  {
    let x = 5;
    let _y = x; // y is a copy of x

    let str1 = String::from("hello");
    // let _str2 = str1; // This invalidates the original pointer (the value of str1 has changed owner, it got moved to _str2)

    // println!("{}", str1); // This won't compile

    let str3 = str1.clone();

    println!("{str3}");
  }

  {
    let s = String::from("hello");
    // If one passed in "s.clone()" then this would compile
    takes_ownership(s); // Passing arguments to a function also invalidates the original variable (they are moved; the function takes ownership of the value)
    // Remember: a String type is allocated on the heap because it's not a primitive type
    // println!("{s}"); // This won't compile
  }

  {
    let x = 5;
    // Remember: integers are copied
    makes_copy(x);
    println!("{x}");
  }

  {
    let str1 = gives_ownership();
    println!("str1 = {str1}");
  }

  {
    let mut str = String::from("hello");
    str = takes_and_gives_back(str);
    println!("str:{str}");
  }

  {
    let str1 = String::from("hello");
    let (str2, len) = calculate_length(str1);
    println!("The length of '{str2}' is {len}.");
  }

  {
    let str1 = String::from("hello");
    // References don't take ownership of the underlying value
    let len = calculate_length_ref(&str1); // References are immutable by default
    println!("The length of '{str1}' is {len}.");
  }

  {
    let mut str = String::from("hello");
    change(&mut str);
    println!("{str}");
  }

  {
    // One cannot borrow a variable as mutable more than once in a scope
    let mut str = String::from("hello");

    // A reference lasts from its initialization till its last use case
    let ref1 = &str;
    let ref2 = &str;

    println!("ref1:{ref1} ref2:{ref2}");

    // After this point ref1 and ref2 no longer exist

    let ref3 = &mut str;
    println!("ref3:{ref3}");
  }

  // One cannot have a mutable reference to a variable if an immutable already exists

  /*
  1. At any given time for a particular piece of data in a particular scope you can either have one mutable reference or any number of immutable references.
  2. References must always be valid. The data they point to must be valid.
  */

  // Slices

  // TODO: continue here => https://youtu.be/VFIOSWy93H0?t=1114
}

fn takes_ownership(some_string: String)
{
  println!("{some_string}");
}

fn makes_copy(some_integer: i32)
{
  println!("{some_integer}");
}

fn gives_ownership() -> String
{
  let some_string = String::from("hello");

  some_string
}

fn takes_and_gives_back(string: String) -> String
{
  let string_copy = string.clone();
  println!("{string_copy}");

  string
}

fn calculate_length(string: String) -> (String, usize)
{
  let length = string.len();
  (string, length)
}

fn calculate_length_ref(string: &String) -> usize
{
  let length = string.len();
  length
}

fn change(some_string: &mut String)
{
  some_string.push_str(" world!");
}
