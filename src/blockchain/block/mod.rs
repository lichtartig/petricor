mod basic_block;
pub use self::basic_block::BasicBlock;

pub trait CryptoBlock {
    fn verify(&self) -> bool;
}