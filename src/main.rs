pub use serde::{Serialize, Deserialize};

pub mod backend;

#[tokio::main]
pub async fn main() {
    println!("Hello")
}