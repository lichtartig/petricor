mod elliptic_curve_secp256k1;
mod sha256;
mod ripemd160;
mod double_sha256;
mod ripe_of_sha;

pub use elliptic_curve_secp256k1::ECsecp256k1;
pub use sha256::SHA256;
pub use ripemd160::RIPEMD160;
pub use double_sha256::DoubleSHA256;
pub use ripe_of_sha::RipeOfSHA;

pub trait HashFunction {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8>;
    fn get_hash_byte_size(&self) -> usize;
}

pub trait GeneratePubKey {
    fn generate_pub_key(&self, priv_key: &Vec<u8>) -> Result<Vec<u8>, String>;
    fn get_required_priv_key_byte_size(&self) -> usize;
}

pub trait MakeSignature {
    fn sign_payload(&self, priv_key: &[u8], payload: &[u8]) -> Result<Vec<u8>, String>;
    fn get_signature_byte_size(&self) -> usize;
}

pub trait VerifySignature {
    fn verify_signature(&self, pub_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool, String>;
}