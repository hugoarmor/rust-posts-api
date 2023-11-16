use std::env;

use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use serde::{Deserialize, Serialize};

pub struct CryptoService {
    encryption_primary_key: String,
    encryption_deterministic_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encrypted {
    iv: String,
    data: String,
}

impl Into<Encrypted> for String {
    fn into(self) -> Encrypted {
        Encrypted {
            iv: env::var("ENCRYPTION_DETERMINISTIC_KEY")
                .expect("Key not found in environment"),
            data: self,
        }
    }
}

impl CryptoService {

    pub fn new() -> Self {
        let encryption_primary_key = env::var("ENCRYPTION_PRIMARY_KEY")
            .expect("Key not found in environment");
        if encryption_primary_key.len() != 32 {
            panic!("Key must be 32 bytes for AES-256, got {}", encryption_primary_key.len());
        }

        let encryption_deterministic_key = env::var("ENCRYPTION_DETERMINISTIC_KEY")
        .expect("Key not found in environment");

        Self {
            encryption_primary_key,
            encryption_deterministic_key
        }
    }

    pub fn encrypt(&self, data: &[u8], deterministic: bool) -> Encrypted {
        let cipher = Cipher::aes_256_cbc();

        let iv = match deterministic {
            true => self.encryption_deterministic_key.as_bytes().to_vec(),
            false => self.generate_random_iv(),
        };

        let mut crypter = match Crypter::new(cipher, Mode::Encrypt, &self.encryption_primary_key.as_bytes(), Some(&iv)) {
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
            iv: base64::encode_block(&iv.to_vec()),
            data: base64::encode_block(&encrypted),
        }
    }

    pub fn decrypt(&self, encrypted: &Encrypted) -> Vec<u8> {
        let cipher = Cipher::aes_256_cbc();

        let iv = base64::decode_block(&encrypted.iv).expect("Failed to decode IV");
        let data = base64::decode_block(&encrypted.data).expect("Failed to decode data");

        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &self.encryption_primary_key.as_bytes(), Some(&iv)).unwrap();

        let mut decrypted = vec![0; data.len() + cipher.block_size()];

        let count = crypter.update(&data, &mut decrypted).unwrap();

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
