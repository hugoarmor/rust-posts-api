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
#[serde(tag = "t")]
pub enum Encrypted {
    #[serde(rename = "d")]
    Deterministic { iv: String, data: String },
    #[serde(rename = "n")]
    NonDeterministic { iv: String, data: String },
}

impl Encrypted {
    pub fn get_iv(&self) -> &String {
        match self {
            Encrypted::Deterministic { iv, .. } => iv,
            Encrypted::NonDeterministic { iv, .. } => iv,
        }
    }

    pub fn get_data(&self) -> &String {
        match self {
            Encrypted::Deterministic { data, .. } => data,
            Encrypted::NonDeterministic { data, .. } => data,
        }
    }
}

pub trait IntoEncrypted {
    fn into_encrypted(self, deterministic: bool) -> Encrypted;
}

impl IntoEncrypted for String {
    fn into_encrypted(self, deterministic: bool) -> Encrypted {
        let crypto_service = CryptoService::new();

        match deterministic {
            true => crypto_service.encrypt(self.as_bytes(), true),
            false => crypto_service.encrypt(self.as_bytes(), false),
        }
    }
}

impl CryptoService {
    pub fn new() -> Self {
        let encryption_primary_key =
            env::var("ENCRYPTION_PRIMARY_KEY").expect("PRIMARY_KEY not found in environment");
        if encryption_primary_key.len() != 32 {
            panic!(
                "PRIMARY_KEY must be 32 bytes for AES-256, got {}",
                encryption_primary_key.len()
            );
        }

        let encryption_deterministic_key =
            env::var("ENCRYPTION_DETERMINISTIC_KEY").expect("DETERMINISTIC_KEY not found in environment");

        if encryption_deterministic_key.len() != 16 {
            panic!(
                "DETERMINISTIC_KEY must be 16 bytes for AES-128, got {}",
                encryption_deterministic_key.len()
            );
        }

        Self {
            encryption_primary_key,
            encryption_deterministic_key,
        }
    }

    pub fn encrypt(&self, data: &[u8], deterministic: bool) -> Encrypted {
        let cipher = Cipher::aes_256_cbc();

        let iv = match deterministic {
            true => self.encryption_deterministic_key.as_bytes().to_vec(),
            false => self.generate_random_iv(),
        };

        let mut crypter = match Crypter::new(
            cipher,
            Mode::Encrypt,
            &self.encryption_primary_key.as_bytes(),
            Some(&iv),
        ) {
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

        let iv = base64::encode_block(&iv.to_vec());
        let data = base64::encode_block(&encrypted);

        match deterministic {
            true => Encrypted::Deterministic { iv, data },
            false => Encrypted::NonDeterministic { iv, data },
        }
    }

    pub fn decrypt(&self, encrypted: &Encrypted) -> Vec<u8> {
        let cipher = Cipher::aes_256_cbc();

        let iv = base64::decode_block(encrypted.get_iv()).expect("Failed to decode IV");
        let data = base64::decode_block(encrypted.get_data()).expect("Failed to decode data");

        let mut crypter = Crypter::new(
            cipher,
            Mode::Decrypt,
            &self.encryption_primary_key.as_bytes(),
            Some(&iv),
        )
        .unwrap();

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
