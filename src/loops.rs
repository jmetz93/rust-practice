pub fn run() {
  let mut count = 0;

  // infinite loop
  // loop {
  //   count += 1;
  //   println!("Number: {}", count);

  //   if count == 20 {
  //     break;
  //   }
  // }

  // while loop (fizzbuzz)
  // while count <= 100 {
  //   let div_by_15 = count % 15 == 0;
  //   let div_by_3 = count % 3 == 0;
  //   let div_by_5 = count % 5 == 0;

  //   if div_by_15 {
  //     println!("fizzbuzz");
  //   } else if div_by_3 {
  //     println!("fizz");
  //   } else if div_by_5 {
  //     println!("buzz");
  //   } else {
  //     println!("{}", count);
  //   }

  //   count += 1;
  // }

  // for range loop fizzbuzz
  for x in 0..100 {
    let div_by_15 = x % 15 == 0;
    let div_by_3 = x % 3 == 0;
    let div_by_5 = x % 5 == 0;

    if div_by_15 {
      println!("fizzbuzz");
    } else if div_by_3 {
      println!("fizz");
    } else if div_by_5 {
      println!("buzz");
    } else {
      println!("{}", x);
    }
  }
}