pub mod block;
pub mod cryptography;
pub mod pub_key;
pub mod signature;
pub mod transaction;

use block::CryptoBlock;
use transaction::UnspentTransactionOutput;

pub trait Blockchain<B: CryptoBlock, U: UnspentTransactionOutput> {
    fn append_block(&mut self, block: B) -> Result<&str, &str>;
    fn get_unspent_transaction_outputs(&self) -> Vec<U>;
    fn get_height(&self) -> usize;
}