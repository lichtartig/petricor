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
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, DoubleSHA256};
    #[test]
    fn test_hash() {
        let hasher = DoubleSHA256::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![49, 107, 121, 239, 240, 29, 41, 97, 237, 152, 11, 147, 23, 1,
                                        115, 146, 194, 117, 206, 95, 157, 198, 31, 244, 42, 183, 192,
                                        214, 106, 20, 112, 24];
        assert_eq!(hashed_input, test_result);
    }
}