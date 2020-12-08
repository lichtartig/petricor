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
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, RipeOfSHA};
    #[test]
    fn test_hash() {
        let hasher = RipeOfSHA::new();
        let test_input: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let hashed_input = hasher.compute_hash(&test_input);
        let test_result: Vec<u8> = vec![207, 106, 174, 194, 214, 44, 201, 132, 219, 49, 55, 116, 21,
                                        99, 248, 103, 170, 141, 156, 106];
        assert_eq!(hashed_input, test_result);
    }
}