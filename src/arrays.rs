// arrays - fixed list where elements are same type

use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];
  // with arrays can reassign values of elements but cant add to it
  // re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);

  // get single val
  println!("single value: {}", numbers[0]);

  // get length
  println!("arr length: {}", numbers.len());

  // arrays are stack allocated
  println!("array occupies {} bytes", mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[1..3];
  println!("slice: {:?}", slice);
}