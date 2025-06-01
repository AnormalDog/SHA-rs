#[inline]
pub fn choose(x : u128, y : u128, z : u128) -> u128 {(x & y) ^ (!x & z)}

#[inline]
pub fn majority(x : u128, y : u128, z : u128) -> u128 {(x & y) ^ (x &  z) ^ (y & z)}

use crate::basic_functions::BasicFunctions;

#[inline]
pub fn upper_sigma0 (x : u128) -> u128 {
  BasicFunctions::right_rotation(&x, 28) ^
  BasicFunctions::right_rotation(&x, 34) ^
  BasicFunctions::right_rotation(&x, 39)
}

#[inline]
pub fn upper_sigma1 (x : u128) -> u128 {
  BasicFunctions::right_rotation(&x, 14) ^
  BasicFunctions::right_rotation(&x, 18) ^
  BasicFunctions::right_rotation(&x, 41)
}

#[inline]
pub fn lower_sigma0 (x : u128) -> u128 {
  BasicFunctions::right_rotation(&x, 1) ^
  BasicFunctions::right_rotation(&x, 8) ^
  BasicFunctions::right_rotation(&x, 7)
}

#[inline]
pub fn lower_sigma1 (x : u128) -> u128 {
  BasicFunctions::right_rotation(&x, 19) ^
  BasicFunctions::right_rotation(&x, 61) ^
  BasicFunctions::right_rotation(&x, 6)
}

