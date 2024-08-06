use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit}; // Import Aead and KeyInit
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Read data from video file
    let mut input_file = File::open("test1.mp4")?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Generate a random key and nonce
    let key_bytes: [u8; 32] = rand::thread_rng().gen(); // Explicitly specify the type
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes); // Specify the key type
    let nonce_bytes: [u8; 12] = rand::thread_rng().gen(); // Explicitly specify the type
    let nonce = Nonce::from_slice(&nonce_bytes); // Specify the nonce type

    // Encrypt the data
    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher.encrypt(nonce, buffer.as_ref()).expect("Encryption failed!");

    // Save the encrypted result to a file
    let mut output_file = File::create("encrypted_video.enc")?;
    output_file.write_all(&nonce_bytes)?; // Save nonce to the file
    output_file.write_all(&ciphertext)?;

    println!("Encryption successful! Key: {:?}, Nonce: {:?}", key_bytes, nonce_bytes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use aes_gcm::aead::{Aead, NewAead};

    #[test]
    fn test_encryption_decryption() {
        let key = Key::<Aes256Gcm>::from_slice(b"an example very very secret key.");
        let nonce = Nonce::from_slice(b"unique nonce"); // 96 bit

        let cipher = Aes256Gcm::new(key);
        let plaintext = b"plaintext message";

        // Encryption
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).expect("encryption failure!");

        // Decryption
        let decrypted_text = cipher.decrypt(nonce, ciphertext.as_ref()).expect("decryption failure!");
        assert_eq!(&plaintext[..], &decrypted_text[..]);
    }
}
