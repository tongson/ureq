use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Read;
use std::str;

extern crate ureq;
extern crate serde_cbor;
use std::collections::HashMap;
use serde_cbor::{from_slice};

#[no_mangle]
pub extern "C" fn qget(u: *const c_char, c: *const c_char) -> *const c_char {
  // Build request from CBOR
  let url = unsafe { CStr::from_ptr(u).to_bytes() };
  let b = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: HashMap<String, String> = from_slice(b).unwrap();
  let mut get = ureq::get(str::from_utf8(url).unwrap()).build();
  let mut req: ureq::Request = get.set("User-Agent", "ureq.qget").build();
  for (k, v) in &v {
    req = get.set(k, v).build();
  }
  // Block!
  let resp = req.call();
  // Process response
  let mut bytes = vec![];
  if resp.status().to_string() == "200" {
    let mut reader = resp.into_reader();
    let _ = reader.read_to_end(&mut bytes);
  } else {
    bytes = resp.status().to_string().as_bytes().to_vec();
  }
  // Return response
  let c_str = CString::new(bytes).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  return ptr
}

#[no_mangle]
pub extern "C" fn qpost(c: *const c_char) -> *const c_char {
  let b = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: HashMap<String, String> = from_slice(b).unwrap();
  let resp = ureq::post(&v["url"])
    .set("User-Agent", "ureq.qpost")
    .send_string(&v["payload"]);
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




