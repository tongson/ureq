use std::ffi::{CStr, CString};
use std::io::Read;
use std::os::raw::c_char;
use std::panic;

extern crate serde_json;
extern crate ureq;
use serde::Deserialize;
use serde_json::from_slice;
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn qget(c: *const c_char) -> *const c_char {
  panic::set_hook(Box::new(move |_| eprintln!("panic: ureq.qget()")));
  #[derive(Deserialize)]
  struct Args {
    url: String,
    headers: HashMap<String, String>,
  }
  // Build request from JSON
  let cb = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: Args = from_slice(cb).unwrap();
  let h: HashMap<String, String> = v.headers;
  let mut get = ureq::get(&v.url).build();
  let mut req: ureq::Request = get.set("User-Agent", "ureq.qget").build();
  for (k, v) in &h {
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
  return ptr;
}

#[no_mangle]
pub extern "C" fn qpost(c: *const c_char) -> *const c_char {
  panic::set_hook(Box::new(move |_| eprintln!("panic: ureq.qpost()")));
  #[derive(Deserialize)]
  struct Args {
    url: String,
    data: String,
    headers: HashMap<String, String>,
  }
  // Build request from JSON
  let cb = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: Args = from_slice(cb).unwrap();
  let h: HashMap<String, String> = v.headers;
  let mut get = ureq::post(&v.url).build();
  let mut req: ureq::Request = get.set("User-Agent", "ureq.qpost").build();
  for (k, v) in &h {
    req = get.set(k, v).build();
  }
  // Block!
  let resp = req.send_string(&v.data);
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
  return ptr;
}
