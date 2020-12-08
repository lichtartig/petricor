use derive_new::new;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use super::{GeneratePubKey, MakeSignature, VerifySignature};

const REQUIRED_PRIV_KEY_BYTE_SIZE: usize = 256;
const SIGNATURE_BYTE_SIZE: usize = 64;

#[derive(new)]
pub struct ECsecp256k1 {
}

impl GeneratePubKey for ECsecp256k1 {
    fn generate_pub_key(&self, priv_key: &Vec<u8>) -> Result<Vec<u8>, String> {
        let secp = Secp256k1::new();
        let priv_key_converted = SecretKey::from_slice(&priv_key);
        match priv_key_converted {
            Ok(pk) => {
                let pub_key = PublicKey::from_secret_key(&secp, &pk);
                Ok(pub_key.serialize().to_vec())
            },
            Err(s) => Err(s.to_string()),
        }
    }

    fn get_required_priv_key_byte_size(&self) -> usize {
        return REQUIRED_PRIV_KEY_BYTE_SIZE;
    }
}

impl MakeSignature for ECsecp256k1 {
    fn sign_payload(&self, priv_key: &[u8], payload: &[u8]) -> Result<Vec<u8>, String> {
        let secp = Secp256k1::new();
        let payload_converted = Message::from_slice(payload);
        let priv_key_converted = SecretKey::from_slice(priv_key);
        match (payload_converted, priv_key_converted) {
            (Ok(plc), Ok(pkc)) => {
                let signature = secp.sign(&plc, &pkc);
                Ok(signature.serialize_compact().to_vec())
            },
            (Err(s), _) | (_, Err(s)) => Err(s.to_string()),
        }
    }

    fn get_signature_byte_size(&self) -> usize {
        return SIGNATURE_BYTE_SIZE;
    }
}

impl VerifySignature for ECsecp256k1 {
    fn verify_signature(&self, pub_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool, String> {
        let secp = Secp256k1::new();
        let payload_converted = Message::from_slice(payload);
        let signature_converted = Signature::from_compact(signature);
        let pub_key_converted = PublicKey::from_slice(pub_key);

        match (payload_converted, signature_converted, pub_key_converted) {
            (Ok(plc), Ok(sc), Ok(pkc)) => {
                Ok(secp.verify(&plc, &sc, &pkc).is_ok())
            },
            (Err(s), _, _) | (_, Err(s), _) | (_, _, Err(s)) => Err(s.to_string()),
        }

    }
}


#[cfg(test)]
mod tests {
    use super::{ECsecp256k1, GeneratePubKey, MakeSignature, VerifySignature};
    use crate::cryptography::elliptic_curve_secp256k1::{SIGNATURE_BYTE_SIZE, REQUIRED_PRIV_KEY_BYTE_SIZE};
    use crate::cryptography::{HashFunction, SHA256};

    #[test]
    fn test_generate_pub_key() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();
        let target_pub_key: Vec<u8> = vec![3, 98, 131, 234, 244, 222, 173, 156, 183, 167, 172, 40,
                                           159, 4, 109, 151, 26, 173, 3, 126, 194, 216, 188, 47, 99,
                                           142, 90, 144, 72, 129, 54, 211, 156];
        assert_eq!(pub_key, target_pub_key);
    }

    #[test]
    fn test_get_required_priv_key_byte_size() {
        let ec = ECsecp256k1::new();
        assert_eq!(ec.get_required_priv_key_byte_size(), REQUIRED_PRIV_KEY_BYTE_SIZE);
    }

    #[test]
    fn test_get_signature_byte_size() {
        let ec = ECsecp256k1::new();
        assert_eq!(ec.get_signature_byte_size(), SIGNATURE_BYTE_SIZE);
    }

    #[test]
    fn test_sign_payload() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let test_signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        let target_signature: Vec<u8> = vec![83, 123, 183, 78, 17, 120, 136, 153, 92, 178, 93, 253,
                                             30, 143, 103, 231, 209, 0, 21, 249, 243, 167, 183, 213,
                                             132, 41, 14, 224, 96, 156, 205, 182, 45, 202, 76, 25,
                                             119, 74, 72, 140, 0, 27, 120, 109, 125, 124, 75, 120,
                                             115, 10, 219, 184, 64, 81, 57, 24, 163, 56, 33, 233,
                                             138, 228, 46, 211];
        assert_eq!(test_signature, target_signature);
    }

    #[test]
    fn test_signature_byte_size() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let test_signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        let target_signature: Vec<u8> = vec![175, 163, 157, 7, 191, 242, 230, 235, 163, 61, 160, 207,
                                             228, 92, 187, 137, 106, 115, 198, 10, 186, 154, 30, 69,
                                             7, 226, 60, 57, 43, 174, 125, 251, 12, 187, 206, 229,
                                             245, 19, 255, 184, 234, 117, 73, 160, 231, 236, 241,
                                             174, 153, 249, 48, 175, 22, 32, 165, 203, 119, 109, 251,
                                             70, 208, 43, 55, 171];
        assert_eq!(target_signature.len(), ec.get_signature_byte_size());
    }

    #[test]
    fn test_get_verify_signature() {
        let hasher = SHA256::new();
        let test_priv_key = hasher.compute_hash(&vec![0, 5, 3, 7, 8]);
        let payload = hasher.compute_hash(&vec![1, 3, 5, 7, 9, 11, 13]);

        let ec = ECsecp256k1::new();
        let pub_key = ec.generate_pub_key(&test_priv_key).unwrap();
        let test_signature = ec.sign_payload(&test_priv_key, &payload).unwrap();

        assert_eq!(ec.verify_signature(&pub_key, &payload, &test_signature).unwrap(), true);
    }
}