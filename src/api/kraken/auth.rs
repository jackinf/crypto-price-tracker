use crate::prelude::*;
use base64::Engine;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::header::HeaderValue;
use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};
use std::env;
use std::time::{Duration, SystemTime};

const BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

pub struct Auth {
    private_key_bytes: [u8; 66],
}

impl Auth {
    pub fn new(private_key: String) -> Result<Self> {
        let mut private_key_bytes = [0; 66];
        BASE64_ENGINE.decode_slice(private_key, &mut private_key_bytes)?;

        Ok(Self { private_key_bytes })
    }
}

impl Auth {
    pub fn sign<D: Serialize>(&self, data: D, path: &str, nonce: u64) -> Result<String> {
        let encoded_data = serde_urlencoded::to_string(&data)?;
        let sha2_result = Sha256::digest(format!("{}{}", nonce, encoded_data));

        let mac = Hmac::<Sha512>::new_from_slice(&self.private_key_bytes)?
            .chain_update(path.as_bytes())
            .chain_update(&sha2_result)
            .finalize()
            .into_bytes();

        Ok(BASE64_ENGINE.encode(mac))
    }

    #[must_use]
    pub fn nonce(&self) -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_micros() as u64
    }
}
