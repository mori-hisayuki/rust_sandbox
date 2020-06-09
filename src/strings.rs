
pub fn run() {
  let mut hello = String::from("Hello ");

  // Get Length
  println!("Length: {}", hello.len());

  // push char
  hello.push('W');

  // push string
  hello.push_str("orld!");

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if empty
  println!("Is Empty: {}", hello.is_empty());

  // Contains
  println!("Contains 'World' {}", hello.contains("World"));

  

  println!("{}", hello);
}
