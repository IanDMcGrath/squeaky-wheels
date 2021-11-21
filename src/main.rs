fn main() {
  println!("{}", say_hello());
}

fn say_hello() -> &'static str {
  return "Hello, world!";
}

#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn should_say_hello() {
    assert_eq!(say_hello(), "Hello, world!");
  }
}