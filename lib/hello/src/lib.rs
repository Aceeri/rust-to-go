
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

pub struct Response;

type ScampFn = *const fn(&Message, &Client);

#[derive(Default)]
struct Registry {
    map: RwLock<HashMap<String, ScampFn>>,
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

    pub fn call(&self, action: String, message: &Message) {
        if let Ok(locked) = self.map.read() {
            
        }
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

pub extern "C" fn register(action: *const libc::c_char, function: *const libc::c_void) {
    let buf_name = unsafe { CStr::from_ptr(action).to_bytes() };
    let action_name = String::from_utf8(buf_name.to_vec()).unwrap();
    let fn_ptr = function as ScampFn;
    REGISTERED.insert(action_name, fn_ptr);
}

pub extern "C" fn call(action: *const libc::c_char, message: Message) {

}

