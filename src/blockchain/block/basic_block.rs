use crate::cryptography::HashFunction;
use crate::blockchain::transaction::Transaction;
use super::CryptoBlock;

#[derive(Debug)]
pub struct BasicBlock<H:HashFunction, T: Transaction> {
    transactions: Vec<T>,
    previous_block_hash: Vec<u8>,
    this_block_hash: Vec<u8>,
    hash_function: H,
}

impl <H: HashFunction, T: Transaction> BasicBlock<H, T> {
    fn new(transactions: Vec<T>, previous_hash: Vec<u8>, hash_function: H) -> Self {
        let bytes = Self::inputs_to_bytes(&transactions, &previous_hash);
        let new_hash = hash_function.compute_hash(&bytes);

        Self {
            transactions: transactions,
            previous_block_hash: previous_hash,
            this_block_hash: new_hash,
            hash_function: hash_function,
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

    fn has_valid_hash(&self) -> bool {
        let bytes = Self::inputs_to_bytes(&self.transactions, &self.previous_block_hash);
        let hash = self.hash_function.compute_hash(&bytes);
        return hash == self.this_block_hash;
    }
}

impl <H: HashFunction, T: Transaction> CryptoBlock for BasicBlock<H, T> {
    fn verify(&self) -> bool {
        let transaction_validity = self.transactions.iter().all(|t| t.verify());
        return transaction_validity && self.has_valid_hash();
    }

    fn get_this_block_hash(&self) -> &[u8] {
        &self.this_block_hash
    }

    fn get_previous_block_hash(&self) -> &[u8] {
        &self.previous_block_hash
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