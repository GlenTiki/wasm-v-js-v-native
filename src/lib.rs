fn is_prime(p: i32) -> bool {
  let upper_limit = (p as f64).sqrt() as i32;
  for i in 2..(upper_limit+1) {
    if p % i == 0  {
      return false;
    }
  }
  return true
}

#[no_mangle]
pub extern fn prime(n: i32) -> i32 {
  if n <= 0 { panic!("n too small") }
  let mut result = 1;
  let mut count = 0;
  while count < n {
    result = result + 1;        
    if is_prime(result) {
      count = count + 1;
    }
  }
  return result
}
