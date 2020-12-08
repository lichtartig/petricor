use crate::cryptography::HashFunction;
use crate::chain::transaction::Transaction;
use super::CryptoBlock;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct BasicBlock<'a, H:HashFunction, T: Transaction> {
    transactions: Vec<T>,
    previous_hash: Vec<u8>,
    new_hash: Vec<u8>,
    hash_function: PhantomData<&'a H>,
}

impl <H: HashFunction, T: Transaction> BasicBlock<'_, H, T> {
    fn new(transactions: Vec<T>, previous_hash: Vec<u8>, hash_function: &H) -> Self {
        let bytes = Self::inputs_to_bytes(&transactions, &previous_hash);
        let new_hash = hash_function.compute_hash(&bytes);

        Self {
            transactions,
            previous_hash,
            new_hash,
            hash_function: PhantomData,
        }
    }

    fn inputs_to_bytes(transactions: &Vec<T>, previous_hash: &Vec<u8>) -> Vec<u8> {
        let transaction_bytes = transactions.iter()
            .flat_map(|t| t.as_bytes());

        let mut bytes = vec![];
        bytes.extend(transaction_bytes);
        bytes.extend(previous_hash);
        return bytes
    }
}

impl <H: HashFunction, T: Transaction> CryptoBlock for BasicBlock<'_, H, T> {
    fn verify(&self) -> bool {
        // TODO
        // verify all transactions
        // verify cryptography
        unimplemented!();
    }
}


/*#[cfg(test)]
mod tests {
    #[test]
    fn test_inputs_to_bytes() {
        //TODO
    }

    #[test]
    fn test_verify() {
        // TODO
    }
}*/