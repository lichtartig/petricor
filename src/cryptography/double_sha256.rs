use super::{HashFunction, SHA256};
use derive_new::new;

#[derive(new, Debug)]
pub struct DoubleSHA256 {
}

impl HashFunction for DoubleSHA256 {
    fn compute_hash(&self, bytes: &Vec<u8>) -> Vec<u8> {
        let hasher = SHA256::new();
        let hashed_once = hasher.compute_hash(bytes);
        return hasher.compute_hash(&hashed_once);
    }

    fn get_hash_byte_size(&self) -> usize {
        let sha256 = SHA256::new();
        return sha256.get_hash_byte_size();
    }
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, DoubleSHA256};
    #[test]
    fn test_hash() {
        let hasher = DoubleSHA256::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![85, 90, 172, 47, 107, 189, 167, 107, 236, 20, 110, 127, 252,
                                        151, 114, 210, 81, 3, 97, 30, 221, 131, 218, 163, 250, 87,
                                        23, 100, 165, 209, 86, 7];
        assert_eq!(hashed_input, test_result);
    }

    #[test]
    fn test_hash_size() {
        let hasher = DoubleSHA256::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        assert_eq!(hashed_input.len(), hasher.get_hash_byte_size());
    }
}