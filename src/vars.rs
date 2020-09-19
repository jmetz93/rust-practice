// variables hold primitive data or refs to data
// variables immutable by default, cant be reassigned
// rust is block-scoped

pub fn run() {
  let name = "Brad";
  let mut age = 37; // mut allow to be reassignable 
  // get warning for not using first version of age
  println!("My name is {} and I am {}", name, age);
  age = 30;
  println!("My name is {} and I am {}", name, age);

  // define constant
  const ID: i32 = 001; // const variable names to be in caps
  println!("ID: {}", ID);

  // assign multiple vars
  let ( my_name, my_age ) = ("Jacob", 26);
  println!("{} is {}", my_name, my_age);
}