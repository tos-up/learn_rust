fn main() {
  fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
      let addition: u32 = match i%2 == 1 {
        true => i, // u32をリターンするのでok
        false => continue, // そもそもリターンしないので、matchの要求される型へは違反しない
      };
      acc += addition;
    }
    acc
  }

  println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}