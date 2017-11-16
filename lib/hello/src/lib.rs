extern crate libc;

use std::collections::HashMap;
use std::ffi::{CStr, CString};

pub struct Message {
    pub action: String,
    //pub envelope: Envelope,
    pub request_id: i64,
    pub version: i64,
    //pub message_type: MessageType,
    pub ticket: String,
    pub token: String,
    pub error: String,
    pub code: String,

    //packets: &[Packet],
    written: u64,
}

pub struct Client {
    pub blah: u64,
}

pub struct Response;

static mut registered: HashMap<String, Box<Fn(&Message, &Client) -> Response>> = HashMap::new();

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello {}!", str_name);
}

pub extern "C" fn register(action: *const libc::c_char, function: *const libc::c_void) {
    let buf_name = unsafe { CStr::from_ptr(action).to_bytes() };
    let action_name = String::from_utf8(buf_name.to_vec()).unwrap();
    let fn_ptr = Box::new(function as *const fn(&Message, &Client) -> Response);
    let casted = fn_ptr as Box<Fn(&Message, &Client) -> Response>;
    //let trait_ptr = fn_ptr as *const Fn(&Message, &Client) -> Response;
    /*
    registered.insert(action_name, Box::new(function as *const () as *const Fn(&Message, &Client) -> Response) as Box<Fn(&Message, &Client) -> Response>);
    */
}

