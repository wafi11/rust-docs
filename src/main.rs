mod encryption;
mod port_scanner;

use crate::encryption::encryption::{all_encryption};
use crate::encryption::hashing::{all_hashing};
use crate::port_scanner::ports::{scanning_port};

// #[tokio::main]
fn main()  {
    all_encryption();
    scanning_port();
    all_hashing()
}