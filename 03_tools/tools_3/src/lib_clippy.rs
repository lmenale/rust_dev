
pub fn clippy_test() {
  let x = 5;

  if x == 5 {  // Clippy will catch this unnecessary comparison
      println!("x is equal to 5");
  } else {
      println!("x is not equal to 5");
  }

  let mut y = 10;

  if y > 5 {  // Clippy will catch this comparison that can be simplified
      y -= 5;
  } else {
      y += 5;
  }

  println!("y is now: {}", y);

  let z: u32 = 25;

  let _ = z as u8;  // Clippy will catch this unnecessary cast

  let numbers = vec![1, 2, 3];
  let _ = numbers[3];  // Clippy will catch this indexing error

  let mut vector = Vec::new();
  vector.push(1);
  vector.push(2);
  vector.push(3);
  let _ = vector.pop();  // Clippy will catch this unused result
  // let number = 5;

  // if number % 2 == 0 {
  //     println!("Number is even");
  // } else {
  //     println!("Number is odd");
  // }
}
