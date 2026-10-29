use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use rand::RngCore;

pub fn encrypt_field(plaintext: &str) -> anyhow::Result<String> {
    let key_b64 = std::env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY 32 bytes base64");
    let key_bytes = B64.decode(key_b64)?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = cipher.encrypt(nonce, plaintext.as_bytes()).map_err(|_| anyhow::anyhow!("encrypt failed"))?;
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ct);
    Ok(B64.encode(combined))
}

pub fn decrypt_field(ciphertext_b64: &str) -> anyhow::Result<String> {
    let key_b64 = std::env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY");
    let key_bytes = B64.decode(key_b64)?;
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)?;
    let combined = B64.decode(ciphertext_b64)?;
    let (nonce_bytes, ct) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let pt = cipher.decrypt(nonce, ct).map_err(|_| anyhow::anyhow!("decrypt failed"))?;
    Ok(String::from_utf8(pt)?)
}
