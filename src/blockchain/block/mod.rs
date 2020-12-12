mod basic_block;
pub use self::basic_block::BasicBlock;

pub trait CryptoBlock {
    fn verify(&self) -> bool;
    fn get_this_block_hash(&self) -> &[u8];
    fn get_previous_block_hash(&self) -> &[u8];
}