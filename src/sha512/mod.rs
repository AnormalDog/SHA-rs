use std::str::FromStr;

// First 64 bits of the fractional part of the cube roots of the first eighty prime numbers
mod constants;

mod functions;

fn padding(bytes : usize) -> usize {
  let x = i64::try_from(bytes * 8)
    .expect("Imposible handle the number of bytes");
  let result : i64 = (896 - (x + 1)) % 1024;
  result as usize
}

pub fn hash_from_file(file : & std::fs::File) -> String {
  
  // Temporal
  String::from("something")
}