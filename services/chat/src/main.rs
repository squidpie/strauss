/*
main.rs
Copyright (C) 2023
Squidpie
*/

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chat_main() {
    main()
  }
}