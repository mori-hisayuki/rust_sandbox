pub fn run () {
  // Defult is "i32"
  let x = 1;

  // Defult is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 454545454545;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  //Boolean
  let is_active = true;

  // Get boolean from expression
  let is_greader = 10 < 5;

  // 
  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greader, a1, face));


}