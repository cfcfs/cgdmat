use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes128Gcm, Key, Nonce};
use base64;
use sha2::{Digest, Sha256};
use std::str;

pub struct Cipher {
    key: Vec<u8>,   // 128-bits
    nonce: Vec<u8>, // 96-bits; unique per message
}

impl Cipher {
    pub fn new(password: &str) -> Cipher {
        let mut hasher = Sha256::new();
        hasher.update(password);
        let hash = hasher.finalize();
        //println!("hash: {:?} len:{}", hash, hash.len());
        return Cipher {
            key: hash[..16].to_vec(),
            nonce: hash[16..28].to_vec(),
        };
    }

    pub fn encrypt(&self, data: &str) -> String {
        let nonce = Nonce::from_slice(self.nonce.as_ref());
        let key = Key::from_slice(self.key.as_ref());
        let cipher = Aes128Gcm::new(key);

        let ciphertext = cipher
            .encrypt(nonce, data.as_ref())
            .expect("encryption failure!");

        let encoded = base64::encode(ciphertext);
        return encoded;
    }

    pub fn decrypt(&self, base64_data: &str) -> String {
        let data = base64::decode(base64_data).unwrap();
        let nonce = Nonce::from_slice(self.nonce.as_ref());
        let key = Key::from_slice(self.key.as_ref());
        let cipher = Aes128Gcm::new(key);

        let plaintext = cipher
            .decrypt(nonce, data.as_ref())
            .expect("encryption failure!");

        return String::from(str::from_utf8(&plaintext).unwrap());
    }
}
