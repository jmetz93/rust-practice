// reference pointers - point to a resource in memory

pub fn run() {
  // primitive array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("arr1 vals: {:?}", arr1);
  println!("arr2 vals: {:?}", arr2);

  /* with non-primitives, if assign another var to a piece of data, 
  the first var will no longer hold that value. You'll need to use a reference (using the & symbol) 
  to point to the resource*/

  // vectors
  let vec1 = vec![1, 2, 3];
  // let vec2 = vec1; throws error
  let vec2 = &vec1; // how to point to it

  println!("vec1 vals: {:?}", vec1);
  println!("vec2 vals: {:?}", vec2);
}