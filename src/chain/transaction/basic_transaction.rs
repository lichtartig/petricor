use super::{Transaction, UnlockingScript, UnspentTransactionOutput};

type PubKey = u64;
type Sign = u64;

pub struct BasicTransaction<U: UnspentTransactionOutput> {
    input_transactions: Vec<U>,
    // TODO: Address or public key?
    output_public_key: PubKey,
    signature: Sign,
}

impl <U: UnspentTransactionOutput> BasicTransaction<U> {
    fn new(input_transactions: Vec<U>, signed_output_transactions: Vec<(Box<dyn UnlockingScript>, U)>) -> Self {
        unimplemented!();
    }
}

impl <U: UnspentTransactionOutput> Transaction for BasicTransaction<U> {
    fn verify(&self) -> bool {
        // TODO!
        // check if signature allows for spending of input transactions
        unimplemented!();
    }

    fn as_bytes(&self) -> Vec<u8> {
        /*let mut bytes = vec![];
        let transaction_bytes = self.input_transactions.iter()
                                            .flat_map(|t| t.as_bytes());

        bytes.extend(transaction_bytes);
        bytes.extend(&self.output_address.to_ne_bytes());
        bytes.extend(&self.signature.to_ne_bytes());

        return bytes;*/
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    //use super::BasicTransaction;
    #[test]
    fn test_byte_conversion() {
        // TODO
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