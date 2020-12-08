use super::HashFunction;
use crypto_hashes::digest::Digest;
use crypto_hashes::sha3::Sha3_256;
use derive_new::new;

// TODO Why is this 32 and not 256!?
const HASH_BYTE_SIZE: usize = 32;

#[derive(new, Debug)]
pub struct SHA256 {
}

impl HashFunction for SHA256 {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha3_256::default();
        hasher.update(&bytes);
        return hasher.finalize().as_slice().to_vec();
    }

    fn get_hash_byte_size(&self) -> usize {
        return HASH_BYTE_SIZE
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
        let test_result: Vec<u8> = vec![237, 36, 121, 248, 73, 128, 216, 70, 205, 18, 68, 127, 36,
                                        16, 89, 172, 22, 121, 172, 48, 88, 68, 67, 212, 2, 34, 251,
                                        126, 22, 57, 65, 76];
        assert_eq!(hashed_input, test_result);
    }

    #[test]
    fn test_hash_size() {
        let hasher = SHA256::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        assert_eq!(hashed_input.len(), hasher.get_hash_byte_size());
    }
}