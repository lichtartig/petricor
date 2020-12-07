mod sha256;
pub use sha256::SHA256;

pub trait HashFunction {
    fn compute_hash(&self, bytes: &Vec<u8>) -> String;
}