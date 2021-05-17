use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub fn hash<T: Hash>(t: T) -> u32 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish() as u32
}
