mod basic_transaction;

pub use basic_transaction::BasicTransaction;

trait UnlockingScript {
    fn execute_unlocking_script(&self) -> [u8];
}

trait LockingScript {
    fn execute_locking_script(&self, input: [u8]) -> bool;
}

pub trait UnspentTransactionOutput {
    fn get_amount(&self) -> f64;
    fn execute_locking_script(&self, input: [u8]) -> bool;
}

pub trait Transaction {
    fn verify(&self) -> bool;
    fn as_bytes(&self) -> Vec<u8>;
}

