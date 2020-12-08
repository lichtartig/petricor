use super::HashFunction;
use crypto_hashes::ripemd160::{Ripemd160, Digest};
use derive_new::new;

// TODO Why is this 20 and not 160!?
const HASH_BYTE_SIZE: usize = 20;

#[derive(new, Debug)]
pub struct RIPEMD160 {
}

impl HashFunction for RIPEMD160 {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let mut hasher = Ripemd160::default();
        hasher.input(&bytes);
        return hasher.result().as_slice().to_vec();
    }

    fn get_hash_byte_size(&self) -> usize {
        return HASH_BYTE_SIZE
    }
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, RIPEMD160};
    #[test]
    fn test_hash() {
        let hasher = RIPEMD160::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![129, 122, 190, 119, 235, 183, 234, 21, 154, 247, 186, 125,
                                        225, 235, 191, 3, 79, 230, 202, 254];
        assert_eq!(hashed_input, test_result);
    }

    #[test]
    fn test_hash_size() {
        let hasher = RIPEMD160::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        assert_eq!(hashed_input.len(), hasher.get_hash_byte_size());
    }
}