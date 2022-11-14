pub fn run()
{
  println!("  Shadowing");

  let x = 10;
  println!("x: {}", x);

  {
    let x = 15;
    println!("x: {}", x);
  }

  {
    let x = "String";
    println!("x: {}", x);
  }

  println!("x: {}", x);

  let x = 100_000;

  println!("x: {}", x);
}
