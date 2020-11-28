use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Read;

extern crate ureq;
extern crate serde_cbor;
use std::collections::BTreeMap;
use serde_cbor::{from_slice};

#[no_mangle]
pub extern "C" fn qget(c: *const c_char) -> *const c_char {
  let b = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: BTreeMap<String, String> = from_slice(b).unwrap();
  let resp = ureq::get(&v["url"])
    .set("User-Agent", "ureq.qget")
    .call();
  let mut bytes = vec![];
  if resp.status().to_string() == "200" {
    let mut reader = resp.into_reader();
    let _ = reader.read_to_end(&mut bytes);
  } else {
    bytes = resp.status().to_string().as_bytes().to_vec();
  }

  let c_str = CString::new(bytes).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  return ptr
}




