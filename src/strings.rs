// primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when need to modify or own string data

pub fn run() {
  let mut hello = String::from("Hello ");
  
  // get length
  println!("Length: {}", hello.len());

  // push is only for adding one char
  hello.push('W'); 

  hello.push_str("orld"); // for more than one char

  //capacity in bytes
  println!("Capacity {}", hello.capacity());

  // check if empty
  println!("Is empty {}", hello.is_empty());

  // contains
  println!("Contains 'World': {}", hello.contains("World"));

  // replace
  println!("Replace: {} ", hello.replace("World", "There"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}", s);

  // Assertion testing (only prints if false)
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", hello);
}