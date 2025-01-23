mod comic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;
use std::net::SocketAddr;

type Cache = Arc<RwLock<HashMap<String, comic::Comic>>>;

fn main() {
    println!("Hello, world!");

}