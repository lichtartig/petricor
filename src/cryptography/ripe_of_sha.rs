use super::{HashFunction, RIPEMD160, SHA256};
use derive_new::new;

#[derive(new, Debug)]
pub struct RipeOfSHA {
}

impl HashFunction for RipeOfSHA {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let sha256 = SHA256::new();
        let ripemd160 = RIPEMD160::new();
        let hashed_once = sha256.compute_hash(bytes);
        return ripemd160.compute_hash(&hashed_once);
    }

    fn get_hash_byte_size(&self) -> usize {
        let ripemd160 = RIPEMD160::new();
        return ripemd160.get_hash_byte_size();
    }
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, RipeOfSHA};
    #[test]
    fn test_hash() {
        let hasher = RipeOfSHA::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![138, 60, 79, 4, 135, 182, 51, 61, 149, 79, 212, 7, 169, 135,
                                        255, 192, 176, 5, 180, 238];
        assert_eq!(hashed_input, test_result);
    }

    #[test]
    fn test_hash_size() {
        let hasher = RipeOfSHA::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        assert_eq!(hashed_input.len(), hasher.get_hash_byte_size());
    }
}