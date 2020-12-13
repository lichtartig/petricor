mod basic_blockchain;
mod block;
mod transaction;

use block::CryptoBlock;
use transaction::UnspentTransactionOutput;

pub use basic_blockchain::BasicBlockchain;

pub trait Blockchain<B: CryptoBlock, U: UnspentTransactionOutput> {
    fn append_block(&mut self, block: B) -> Result<&str, &str>;
    fn get_unspent_transaction_outputs(&self) -> Vec<U>;
    fn get_height(&self) -> usize;
}