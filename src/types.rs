/* 
Primitive types--
Integers: u8, i8, u16, i16, u32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Charaters (char)
Tuples
Arrays (fixed length)
Vectors (dynamic sized arrays)

Rust is statically typed so needs to know type of all vars at compile time,
however the compiler can usually infer what type we want to use based on value of var
*/

pub fn run() {
  // default is "i32"
  let x = 1;

  // default is "f64"
  let y = 2.5;

  // add explicit type
  let z: i64 = 4345354343434343434;

  // find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // boolean
  let is_active: bool = true;

  // get bool from expression
  let is_greater = 10 > 5;

  let a1 = 'a';
  let face = '\u{1F600}';
  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}