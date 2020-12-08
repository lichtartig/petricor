mod basic_transaction;
mod basic_utxo;
mod pay_to_public_key_hash;

pub use basic_transaction::BasicTransaction;
pub use basic_utxo::BasicUTXO;

trait UnlockingScript {
    fn execute_script(&self) -> Vec<u8>;
    fn as_bytes(&self) -> Vec<u8>;
}

trait LockingScript {
    fn execute_script(&self, input: Vec<u8>) -> bool;
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait UnspentTransactionOutput {
    fn get_amount(&self) -> f32;
    fn execute_locking_script(&self, input: Vec<u8>) -> bool;
    fn as_bytes(&self) -> Vec<u8>;
    fn get_source_addr(&self) -> &[u8];
    fn get_destination_addr(&self) -> &[u8];
}

pub trait Transaction {
    fn verify(&self) -> bool;
    fn as_bytes(&self) -> Vec<u8>;
}

