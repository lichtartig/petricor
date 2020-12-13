use crate::blockchain::block::CryptoBlock;
use crate::blockchain::transaction::UnspentTransactionOutput;

pub struct BlockNode<B: CryptoBlock, U: UnspentTransactionOutput + Clone> {
    block: B,
    blocks_utxos: Vec<U>,
    children: Vec<BlockNode<B, U>>,
}

impl <B: CryptoBlock, U: UnspentTransactionOutput + Clone> BlockNode<B, U> {
    pub fn new(block: B, parent_utxos: Vec<U>) -> Self {
        let blocks_utxos = Self::get_blocks_utxos(parent_utxos, &block);
        Self {
            block: block,
            blocks_utxos: blocks_utxos,
            children: vec![],
        }
    }

    pub fn append_block(&mut self, block: B) -> Result<&str, &str> {
        match (self.is_child(&block), self.is_valid(&block)) {
            (true, true) => {
                self.children.push(BlockNode::new(block, self.blocks_utxos.to_vec()));
                Ok("Success.")
            },
            (true, false) => Err("The block's UTXO's are invalid."),
            (false, _) => {
                self.append_to_children(block)
            },
        }
    }

    fn get_blocks_utxos(parent_utxos: Vec<U>, block: &B) -> Vec<U> {
        // TODO
        unimplemented!();
    }

    fn is_descendant(&self, block: &B) -> bool {
        self.is_child(block) | self.children.iter().any(|c| c.is_descendant(block))
    }

    fn is_child(&self, block: &B) -> bool {
        self.block.get_this_block_hash() == block.get_previous_block_hash()
    }

    fn is_valid(&self, block: &B) -> bool {
        // TODO
        unimplemented!();
        //let spend_utxos = block.get_spent_utxos();
    }

    fn append_to_children(&mut self, block: B) -> Result<&str, &str> {
        let mut ancestors = self.children.iter_mut().filter(|c| c.is_descendant(&block));
        match ancestors.next() {
            None => Err("Block has no ancestors."),
            Some(a) => a.append_block(block)
        }
    }
}