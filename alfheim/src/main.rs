mod comic;
use axum::{
    extract::{Path, Json},
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, hash::Hash};
use std::sync::{Arc, Mutex};
use std::sync::RwLock;
use std::net::SocketAddr;

type Cache = Arc<RwLock<HashMap<String, comic::Comic>>>;

#[tokio::main]
async fn main() {
    let comic_cache: Cache = Arc::new(RwLock::new(HashMap::new()));
    
    let app = Router::new();
    
    
    println!("Hello, world!");
}