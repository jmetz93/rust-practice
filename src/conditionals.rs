pub fn run() {
  let age = 18;
  let check_id: bool = true;
  let knows_person_of_age = false;

  let can_drink = age >= 21 && check_id;
  println!("can drink var: {}", can_drink);
  // if/else
  if can_drink || knows_person_of_age {
    println!("Over 21");
  } else if age < 21 && check_id {
    println!("under 21")
  } else {
    println!("need to see id");
  }
}