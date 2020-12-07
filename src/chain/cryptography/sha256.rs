use crate::chain::cryptography::HashFunction;
use crypto_hash;
use derive_new::new;

#[derive(new, Debug)]
pub struct SHA256 {
}

impl HashFunction for SHA256 {
    fn compute_hash(&self, bytes: &Vec<u8>) -> String {
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}


#[cfg(test)]
mod tests {
    use super::{HashFunction, SHA256};
    #[test]
    fn test_hash() {
        let hash_fct = SHA256::new();
        let test_data = hash_fct.compute_hash(&vec![1,2,3,4,5]);
        let test_hash = "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0";
        assert_eq!(test_data, test_hash);
    }
}