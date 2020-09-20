// vectors - resizable arrays
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
  // with arrays can reassign values of elements but cant add to it
  // re-assign value
  numbers[2] = 20;

  // add onto vector
  numbers.push(5);
  numbers.push(6);

  // pop off last val
  numbers.pop();

  println!("{:?}", numbers);

  // get single val
  println!("single value: {}", numbers[0]);

  // get length
  println!("vector length: {}", numbers.len());

  // arrays are stack allocated
  println!("vector occupies {} bytes", mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[1..3];
  println!("slice: {:?}", slice);

  // loop through vector
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // loop and mutate vals (like map in js)
  for x in numbers.iter_mut() {
    *x *= 2; // multiplying by 2 
  }

  println!("new numbers vec: {:?}", numbers);
}