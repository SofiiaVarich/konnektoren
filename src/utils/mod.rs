use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

#[cfg(feature = "verifiable-credentials")]
mod verifiable_credential;
#[cfg(feature = "verifiable-credentials")]
pub use verifiable_credential::*;

#[cfg(feature = "certificate-image")]
mod certificate_image;
#[cfg(feature = "certificate-image")]
pub use certificate_image::{create_certificate, create_certificate_data_url};

pub mod translation;

pub fn keypair_from_static_str() -> (SigningKey, VerifyingKey) {
    let mut hasher = Sha256::new();
    hasher.update(env!("SIGNATURE_PRIVATE_KEY"));
    let result = hasher.finalize();

    let seed: [u8; 32] = result[..]
        .try_into()
        .expect("Hash output size does not match ed25519 seed size");

    let signing_key = SigningKey::from_bytes(&seed);
    let verify_key = signing_key.verifying_key();

    (signing_key, verify_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Verifier;
    use ed25519_dalek::Signer;

    #[test]
    fn test_keypair_from_static_str() {
        let (signing_key, verifying_key) = keypair_from_static_str();

        let message = b"Hello, World!";
        let signature = signing_key.sign(message);
        assert!(verifying_key.verify(message, &signature).is_ok());
    }
}
