use super::{Transaction, UnlockingScript, UnspentTransactionOutput};
use derive_new::new;

#[derive(new)]
pub struct BasicTransaction<U: UnspentTransactionOutput> {
    signed_input_transactions: Vec<(Box<dyn UnlockingScript>, U)>,
    output_transactions: Vec<U>,
}

impl <U: UnspentTransactionOutput> Transaction for BasicTransaction<U> {
    // TODO Note: This only verifies if a signature matches a transaction. It does not verify it this UTXO was actually free. This needs to be handled by the blockchain.
    fn verify(&self) -> bool {
        for (s, u) in &self.signed_input_transactions {
            let unlock = s.execute_script();
            if !u.execute_locking_script(unlock) {
                return false;
            }
        }
        return true;
    }

    fn as_bytes(&self) -> Vec<u8> {
        let input_transaction_bytes = self.signed_input_transactions.iter()
                                    .flat_map(|(_, t)| t.as_bytes());
        let unlocking_script_bytes = self.signed_input_transactions.iter()
                                    .flat_map(|(s, _)| s.as_bytes());
        let output_transaction_bytes = self.output_transactions.iter()
                                    .flat_map(|t| t.as_bytes());

        let mut bytes = vec![];
        bytes.extend(input_transaction_bytes);
        bytes.extend(unlocking_script_bytes);
        bytes.extend(output_transaction_bytes);
        return bytes;
    }
}


#[cfg(test)]
mod tests {
    //use super::BasicTransaction;
    #[test]
    fn test_byte_conversion() {
        //TODO
    }

    #[test]
    fn test_verify_positive() {
        // TODO
        // test if a single allowed transaction passes verify
    }

    #[test]
    fn test_verify_positive_multiple() {
        // TODO
        // test if multiple allowed transactions pass verify
    }

    #[test]
    fn test_verify_negative() {
        // TODO
        // test if a forbidden transactions passes verify
    }

    #[test]
    fn test_verify_negative_multiple() {
        // TODO
        // test if a mix of allowed and forbidden transactions passes verify
    }
}