
#[macro_use]
extern crate lazy_static;
extern crate libc;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::RwLock;

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

#[derive(Default)]
struct Registry {
    map: RwLock<HashMap<String, ScampFn>>,
}

enum ScampFn {
    External(extern fn(CString) -> isize),
}

unsafe impl Send for Registry { }
unsafe impl Sync for Registry { }

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&self, action: String, method: ScampFn) {
        if let Ok(mut locked) = self.map.write() {
            locked.insert(action, method);
        }
    }

    pub fn call(&self, action: String, message: String) -> Option<isize> {
        if let Ok(locked) = self.map.read() {
            match locked.get(&action) {
                Some(&ScampFn::External(fnptr)) => {
                    let input = CString::new(message).unwrap();
                    unsafe {
                        return Some(fnptr(input));
                    }
                },
                _ => unimplemented!(),
            }
        }

        None
    }
}

lazy_static! {
    static ref REGISTERED: Registry = Registry::default();
}

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello {}!", str_name);
}

#[no_mangle]
pub extern "C" fn register_action(action: *const libc::c_char, function: extern fn(CString) -> isize) {
    let buf_name = unsafe { CStr::from_ptr(action).to_bytes() };
    let action_name = String::from_utf8(buf_name.to_vec()).unwrap();
    REGISTERED.insert(action_name, ScampFn::External(function));
}

#[no_mangle]
pub extern "C" fn call_action(action: *const libc::c_char, input: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(action).to_bytes() };
    let action_name = String::from_utf8(buf_name.to_vec()).unwrap();

    /* ignoring the input for now, just sending the action name as input
    let buf_input = unsafe { CStr::from_ptr(input).to_bytes() };
    let input = String::from_utf8(buf_input.to_vec()).unwrap();
    */

    REGISTERED.call(action_name.clone(), action_name);
}

