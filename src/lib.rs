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
  let mut url = ureq::get(&v["url"]).build();
  let mut agent: ureq::Request;
  if v.contains_key("user-agent") {
    agent = url.set("User-Agent", &v["user-agent"]).build();
  } else {
    agent = url.set("User-Agent", "ureq.qget").build();
  }
  let resp = agent.call();
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

#[no_mangle]
pub extern "C" fn qpost(c: *const c_char) -> *const c_char {
  let b = unsafe { CStr::from_ptr(c).to_bytes() };
  let v: BTreeMap<String, String> = from_slice(b).unwrap();
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




