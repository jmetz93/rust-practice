pub fn run() {
  println!("Hello from print file");
  // basic formatting
  println!("Number {} {}", 1, 2); // using literals to plug in to print

  // positional arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Brad", "Maryland", "code"
  );

  // named arguments
  println!(
    "{name} likes to play {activity}",
    name = "John",
    activity = "baseball"
  );

  // placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // basic math
  println!("10 + 10 = {}", 10 + 10)
}
