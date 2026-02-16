use sha2::{Sha256, Digest};
use hex;

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}
// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }


pub fn all_hashing(){
    let personal = Person {
        id : 1,
        name : "James".to_string(),
        phone : 1000
    };
    // Hashing
    let mut hasher = Sha256::new();
    hasher.update(personal.phone.to_string().as_bytes());
    let hashed_result = hasher.finalize();
    println!("hashed result: {:?}", hashed_result);

    let hashed_hex = hex::encode(hashed_result);

    println!("Original data: {:?}", personal.name);
    println!("SHA-256 hash: {}", hashed_hex);

    // Verification (by comparing the computed hash to an expected value)
    let expected_hash = "40510175845988f13f6162ed8526f0b09f73384467fa855e1e79b44a56562a58";

    if hashed_hex == expected_hash {
        println!("Verification successful: the data matches the expected hash.");
    } else {
        println!("Verification failed: the data has been altered or does not match.");
    }
}