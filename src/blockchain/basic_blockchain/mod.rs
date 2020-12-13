mod block_node;

use super::{Blockchain, CryptoBlock, UnspentTransactionOutput};
use self::block_node::BlockNode;

pub struct BasicBlockchain<B: CryptoBlock, U: UnspentTransactionOutput + Clone> {
    genesis_block_node: BlockNode<B, U>,
    orphan_blocks: Vec<B>,
}

impl <B: CryptoBlock, U: UnspentTransactionOutput + Clone> BasicBlockchain<B, U> {
    fn new(genesis_block: B) -> Self {
        // TODO
        unimplemented!();
    }

    fn try_to_append_orphan_blocks(&mut self) {
        // TODO
        unimplemented!();
    }
}

impl <B: CryptoBlock, U: UnspentTransactionOutput + Clone> Blockchain<B, U> for BasicBlockchain<B, U> {
    fn append_block(&mut self, block: B) -> Result<&str, &str> {
        // TODO
        unimplemented!();
    }

    fn get_unspent_transaction_outputs(&self) -> Vec<U> {
        // TODO
        unimplemented!();
    }

    fn get_height(&self) -> usize {
        // TODO
        unimplemented!();
    }
}