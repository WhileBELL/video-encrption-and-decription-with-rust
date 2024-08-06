use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit}; // Import Aead and KeyInit
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
// Encryption successful! Key: [18, 155, 95, 191, 157, 115, 251, 227, 30, 34, 137, 133, 197, 93, 106, 181, 74, 113, 143, 231, 124, 254, 21, 105, 234, 125, 61, 244, 61, 238, 121, 130], 
// Nonce: [190, 89, 1, 229, 41, 166, 250, 50, 46, 35, 99, 192]
    // open ecrption files
    let mut encrypted_file = File::open("encrypted_video.enc")?;
    let mut buffer = Vec::new();
    encrypted_file.read_to_end(&mut buffer)?;

    // separate nonce and ciphertext
    let (nonce_bytes, ciphertext) = buffer.split_at(12); // nonce size is 12 byte

    // input a decrption old key
    let key_bytes = [18, 155, 95, 191, 157, 115, 251, 227, 30, 34, 137, 133, 197, 93, 106, 181, 74, 113, 143, 231, 124, 254, 21, 105, 234, 125, 61, 244, 61, 238, 121, 130];

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(nonce_bytes);

    // make a decrption and decrption data
    let cipher = Aes256Gcm::new(key);
    let decrypted_data = cipher.decrypt(nonce, ciphertext).expect("Decryption failed!");

    // save a decrption files
    let mut output_file = File::create("decrypted_video.mkv")?;
    output_file.write_all(&decrypted_data)?;

    println!("Decryption successful!");
    Ok(())
}
