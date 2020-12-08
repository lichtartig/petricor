mod sha256;
mod ripemd160;
mod double_sha256;
mod ripe_of_sha;

pub use sha256::SHA256;
pub use ripemd160::RIPEMD160;
pub use double_sha256::DoubleSHA256;
pub use ripe_of_sha::RipeOfSHA;

pub trait HashFunction {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8>;
}

pub trait VerifySignature {
    fn verify_signature(&self, pub_key: &[u8], payload: &[u8]) -> bool;
}

pub trait MakeSignature {
    fn sign_payload(&self, priv_key: &[u8], payload: &[u8]) -> Vec<u8>;
    fn get_signature_byte_size(&self) -> usize;
}