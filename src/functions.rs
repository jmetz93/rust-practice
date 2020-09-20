pub fn run() {
  greeting("Hello", "nice to meet you");

  let get_sum = add(5, 5);
  println!("sum: {}", get_sum);

  // closure
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  // return n1 + n2;
  n1 + n2 // no semicolon means return
}