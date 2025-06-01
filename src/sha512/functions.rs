
#[inline]
pub fn choose(x : u128, y : u128, z : u128) -> u128 {(x & y) ^ (!x & z)}

#[inline]
pub fn majority(x : u128, y : u128, z : u128) -> u128 {(x & y) ^ (x &  z) ^ (y & z)}

#[inline]
pub fn right_shift(x : u128, n : u128) -> u128 {x >> n}

#[inline]
pub fn right_rotate(x : u128, n : u128) -> u128 {
  (x >> n) | (x << (crate::sha512::constants::WSIZE - n)) 
}