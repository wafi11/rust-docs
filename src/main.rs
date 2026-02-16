mod encryption;

use crate::encryption::encryption::{all_encryption};
use crate::encryption::hashing::{all_hashing};

// #[tokio::main]
fn main()  {
    all_encryption();
    all_hashing()
}