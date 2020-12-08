use derive_new::new;
use crate::cryptography::VerifySignature;
use super::{LockingScript, UnlockingScript};

#[derive(new)]
pub struct PayToPublicKeyHashLockingScript<V: VerifySignature> {
    pub_key: Vec<u8>,
    payload: Vec<u8>,
    pub_key_byte_size: usize,
    signature_byte_size: usize,
    signature_verifier: V,
}

impl <V: VerifySignature> LockingScript for PayToPublicKeyHashLockingScript<V> {
    fn execute_script(&self, input: Vec<u8>) -> bool {
        if input.len() - 1 != self.pub_key_byte_size + self.signature_byte_size {
            return false;
        }

        let input_signature = &input[..self.signature_byte_size];
        let input_pub_key = &input[self.signature_byte_size..];

        if input_pub_key != self.pub_key {
            return false;
        }

        match self.signature_verifier.verify_signature(&self.pub_key, &self.payload, &input_signature) {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(&self.pub_key);
        bytes.extend(&self.pub_key_byte_size.to_ne_bytes());
        bytes.extend(&self.signature_byte_size.to_ne_bytes());
        return bytes;
    }
}


#[derive(new)]
pub struct PayToPublicKeyHashUnlockingScript {
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl UnlockingScript for PayToPublicKeyHashUnlockingScript {
    fn execute_script(&self) -> Vec<u8> {
        return self.as_bytes();
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.signature);
        bytes.extend(&self.pub_key);
        return bytes;
    }
}


#[cfg(test)]
mod tests {
    use crate::cryptography::{ECsecp256k1, GeneratePubKey, HashFunction, MakeSignature, SHA256, VerifySignature};
    use crate::blockchain::transaction::pay_to_public_key_hash::{LockingScript, UnlockingScript, PayToPublicKeyHashLockingScript, PayToPublicKeyHashUnlockingScript};

    #[test]
    fn test_locking_script_as_bytes() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();

        let hash_size = hasher.get_hash_byte_size();
        let signature_size = ec.get_signature_byte_size();

        let locking_script = PayToPublicKeyHashLockingScript::new(pub_key, payload, hash_size, signature_size, ec);
        let target_bytes: Vec<u8> = vec![3, 98, 131, 234, 244, 222, 173, 156, 183, 167, 172, 40, 159,
                                         4, 109, 151, 26, 173, 3, 126, 194, 216, 188, 47, 99, 142,
                                         90, 144, 72, 129, 54, 211, 156, 32, 0, 0, 0, 0, 0, 0, 0, 64,
                                         0, 0, 0, 0, 0, 0, 0];
        assert_eq!(locking_script.as_bytes(), target_bytes);
    }

    #[test]
    fn test_unlocking_script_as_bytes() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();
        let signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        let unlocking_script = PayToPublicKeyHashUnlockingScript::new(signature, pub_key);
        let target_bytes: Vec<u8> = vec![83, 123, 183, 78, 17, 120, 136, 153, 92, 178, 93, 253, 30,
                                         143, 103, 231, 209, 0, 21, 249, 243, 167, 183, 213, 132, 41,
                                         14, 224, 96, 156, 205, 182, 45, 202, 76, 25, 119, 74, 72,
                                         140, 0, 27, 120, 109, 125, 124, 75, 120, 115, 10, 219, 184,
                                         64, 81, 57, 24, 163, 56, 33, 233, 138, 228, 46, 211, 3, 98,
                                         131, 234, 244, 222, 173, 156, 183, 167, 172, 40, 159, 4,
                                         109, 151, 26, 173, 3, 126, 194, 216, 188, 47, 99, 142, 90,
                                         144, 72, 129, 54, 211, 156];
        assert_eq!(unlocking_script.as_bytes(), target_bytes);
    }

    #[test]
    fn test_unlocking_script() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();
        let signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        let unlocking_script = PayToPublicKeyHashUnlockingScript::new(signature.to_vec(), pub_key.to_vec());
        let script_output = unlocking_script.execute_script();

        let mut target_output = vec![];
        target_output.extend(signature);
        target_output.extend(pub_key);

        assert_eq!(script_output, target_output);
    }

    #[test]
    fn test_locking_script() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();
        let signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        let hash_size = hasher.get_hash_byte_size();
        let signature_size = ec.get_signature_byte_size();

        let locking_script = PayToPublicKeyHashLockingScript::new(pub_key.to_vec(), payload, hash_size, signature_size, ec);
        let unlocking_script = PayToPublicKeyHashUnlockingScript::new(signature, pub_key);

        let unlock_output = unlocking_script.execute_script();
        assert_eq!(locking_script.execute_script(unlock_output), true);
    }
}