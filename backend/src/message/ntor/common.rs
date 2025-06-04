use log::error;
use rand_core::OsRng;
use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};
use x25519_dalek::{PublicKey, StaticSecret};
use crate::message::ntor::utils::vec_to_array32;

pub struct PrivatePublicKeyPair {
    // In the future, type StaticSecret should be reserved for the server's static and the EphemeralSecret reserved for the ephemeral private key.
    // However, as a quirk of the nTOR protocol, we also need to use StaticSecret for the client's ephemeral private key hence why it is adopted here.
    pub(crate) private_key: Option<StaticSecret>,
    pub public_key: PublicKey,
}

pub fn generate_private_public_key_pair() -> PrivatePublicKeyPair {
    let private_key = StaticSecret::random_from_rng(OsRng);
    let public_key = PublicKey::from(&private_key);

    PrivatePublicKeyPair {
        private_key: Some(private_key),
        public_key,
    }
}

pub struct Certificate {
    pub public_key: PublicKey,
    pub server_id: String,
}

// In the paper, the outgoing message is ("ntor", B_id, client_ephemeral_public_key).
pub struct InitSessionMessage {
    pub client_ephemeral_public_key: PublicKey,
}

impl InitSessionMessage {
    pub fn from(bytes: Vec<u8>) -> Self {
        let u8_array = vec_to_array32(bytes);
        InitSessionMessage {
            client_ephemeral_public_key: PublicKey::from(u8_array),
        }
    }
}

// In the paper, the return message is ("ntor", server_ephemeral_public_key, t_b_hash).
pub struct InitSessionResponse {
    pub server_ephemeral_public_key: PublicKey,
    pub t_hash: Vec<u8>
}

pub(crate) fn encrypt(key_bytes: Vec<u8>, mut data: Vec<u8>) -> Result<([u8; 12], Vec<u8>), &'static str> {
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes);

    if let Err(err) = key {
        error!("Error encrypt: {:?}", err);
        return Err("encrypt failed")
    }

    let sealing_key = aead::LessSafeKey::new(key.unwrap());

    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes).unwrap();
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    return match sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut data) {
        Ok(()) => {
            Ok((nonce_bytes, data))
        }
        Err(err) => {
            error!("encrypt failed {:?}", err);
            Err("encrypt failed")
        }
    }
}

pub(crate) fn decrypt(nonce_bytes: [u8; 12], key_bytes: Vec<u8>, mut data: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap();
    let opening_key = aead::LessSafeKey::new(key);
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    let decrypted_data = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut data).unwrap();

    // println!("Decrypted: {:?}", String::from_utf8_lossy(decrypted_data));
    Ok(decrypted_data.to_vec())
}




