use std::env;

pub fn run() {
  // use env args to get from cli
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "jacob";
  let status = "100%";

  // println!("Command: {:?}", command);

  if command == "hello" {
    println!("{} {}", command, name);
  } else if command == "status" {
    println!("status is {}", status);
  } else {
    println!("that is not a valid command");
  }
}