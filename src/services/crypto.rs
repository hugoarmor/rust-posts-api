use std::env;

use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use serde::{Deserialize, Serialize};

pub struct CryptoService {
    simetric_secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encrypted {
    iv: Vec<u8>,
    data: Vec<u8>,
}

impl CryptoService {

    pub fn new() -> Self {
        let simetric_secret_key = env::var("SYMETRIC_SECRET_KEY")
            .expect("Key not found in environment")
            .into_bytes();
        if simetric_secret_key.len() != 32 {
            panic!("Key must be 32 bytes for AES-256, got {}", simetric_secret_key.len());
        }

        Self {
            simetric_secret_key,
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Encrypted {
        let cipher = Cipher::aes_256_cbc();

        let iv = self.generate_random_iv();

        let mut crypter = match Crypter::new(cipher, Mode::Encrypt, &self.simetric_secret_key, Some(&iv)) {
            Ok(c) => c,
            Err(e) => panic!("Failed to create Crypter: {}", e),
        };

        let mut encrypted = vec![0; data.len() + cipher.block_size()];
        let count = crypter
            .update(data, &mut encrypted)
            .expect("Failed to encrypt");
        let rest = crypter
            .finalize(&mut encrypted[count..])
            .expect("Failed to finalize encryption");

        encrypted.truncate(count + rest);

        Encrypted {
            iv: iv.to_vec(),
            data: encrypted,
        }
    }

    pub fn decrypt(&self, encrypted: &Encrypted) -> Vec<u8> {
        let cipher = Cipher::aes_256_cbc();

        let key = env::var("SYMETRIC_SECRET_KEY").unwrap().into_bytes();
        let iv = encrypted.iv.as_slice();
        let data = encrypted.data.as_slice();

        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(iv)).unwrap();

        let mut decrypted = vec![0; data.len() + cipher.block_size()];

        let count = crypter.update(data, &mut decrypted).unwrap();

        let rest = crypter.finalize(&mut decrypted[count..]).unwrap();

        decrypted.truncate(count + rest);

        decrypted
    }

    pub fn validate_token(&self, token: &str, encrypted_token: Encrypted) -> bool {
        let decrypted_token = self.decrypt(&encrypted_token);
        let decrypted_token = String::from_utf8(decrypted_token).unwrap();

        token == decrypted_token
    }

    pub fn generate_random_iv(&self) -> Vec<u8> {
        let mut iv = [0u8; 16];
        rand_bytes(&mut iv).expect("Failed to generate IV");

        iv.to_vec()
    }

    pub fn generate_random_token(&self) -> String {
        let mut token = [0u8; 32];
        rand_bytes(&mut token).expect("Failed to generate token");
        base64::encode_block(&token)
    }
}
