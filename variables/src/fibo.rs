pub fn main(num: i32) {
  println!("{}", fibo(num, 0, 1))
}

fn fibo(iter: i32, num: i32, num2: i32) -> i32 {
  if iter == 0 {
    num
  } else {
    fibo(iter - 1, num2, num + num2)
  }
}
