pub mod sha512;

mod basic_functions;

#[cfg(test)]
mod tests {

  #[test]
  fn right_shift_test() {
    let x : u64 = crate::basic_functions::BasicFunctions::right_shift(&1000, 30);
    assert_eq!(x, (1000 >> 30))
  }

  #[test]
  fn right_rotation_test() {
    let x : u32 = crate::basic_functions::BasicFunctions::right_rotation(&1000, 30);
    assert_eq!(x, 4000);
  }

}