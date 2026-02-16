use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};

pub fn all_encryption()  {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    let nonce = Nonce::from_slice(b"password1234");

    let plaintext = b"test";

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .expect("encryption failed");

    println!("Encrypted: {:?}", ciphertext);

    // Decrypt
    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failed");

    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}