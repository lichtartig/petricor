use super::HashFunction;
use crypto_hashes::digest::Digest;
use crypto_hashes::sha3::Sha3_256;
use derive_new::new;

#[derive(new, Debug)]
pub struct SHA256 {
}

impl HashFunction for SHA256 {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha3_256::default();
        hasher.update(&bytes);
        return hasher.finalize().as_slice().to_ascii_lowercase();
    }
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, SHA256};
    #[test]
    fn test_hash() {
        let hasher = SHA256::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![237, 36, 121, 248, 105, 128, 216, 102, 205, 18, 100, 127, 36,
                                        16, 121, 172, 22, 121, 172, 48, 120, 100, 99, 212, 2, 34, 251,
                                        126, 22, 57, 97, 108];
        assert_eq!(hashed_input, test_result);
    }
}