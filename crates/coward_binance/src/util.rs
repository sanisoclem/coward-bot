use hmac::{Hmac, Mac, NewMac};
use std::{
  fmt::Write,
  time::{SystemTime, UNIX_EPOCH},
};

type HmacSha256 = Hmac<sha2::Sha256>;

// TODO: cleanup this module
pub fn get_timestamp() -> u128 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis()
}
pub fn encode_hex(bytes: &[u8]) -> String {
  let mut s = String::with_capacity(bytes.len() * 2);
  for &b in bytes {
    write!(&mut s, "{:02x}", b).expect("write to string should always succeed");
  }
  s
}

pub fn sign_bytes(secret: &[u8], bytes: &[u8]) -> Vec<u8> {
  let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
  mac.update(bytes);
  mac.finalize().into_bytes().to_vec()
}
