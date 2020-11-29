use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Read;

extern crate ureq;
extern crate serde_cbor;
use std::collections::HashMap;
use serde_cbor::{from_slice};

#[no_mangle]
pub extern "C" fn qget(m: *const c_char, c: *const c_char) -> *const c_char {
  // Build request from CBOR
  let mb = unsafe { CStr::from_ptr(m).to_bytes() };
  let cb = unsafe { CStr::from_ptr(c).to_bytes() };
  let mv: HashMap<String, String> = from_slice(mb).unwrap();
  let cv: HashMap<String, String> = from_slice(cb).unwrap();
  let mut get = ureq::get(&mv["url"]).build();
  let mut req: ureq::Request = get.set("User-Agent", "ureq.qget").build();
  for (k, v) in &cv {
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
pub extern "C" fn qpost(m: *const c_char, c: *const c_char) -> *const c_char {
  // Build request from CBOR
  let mb = unsafe { CStr::from_ptr(m).to_bytes() };
  let cb = unsafe { CStr::from_ptr(c).to_bytes() };
  let mv: HashMap<String, String> = from_slice(mb).unwrap();
  let cv: HashMap<String, String> = from_slice(cb).unwrap();
  let mut get = ureq::post(&mv["url"]).build();
  let mut req: ureq::Request = get.set("User-Agent", "ureq.qpost").build();
  for (k, v) in &cv {
    req = get.set(k, v).build();
  }
  // Block!
  let resp = req.send_string(&mv["data"]);
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
