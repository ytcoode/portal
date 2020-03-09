use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Global {
    tunnels: HashMap<String, Vec<String>>,
}

pub fn new() -> Arc<Mutex<Global>> {
    Arc::new(Mutex::new(Global {
        tunnels: HashMap::new(),
    }))
}

mod http;
mod sync;

pub use http::serve;
pub use sync::sync;

pub fn start() {}
