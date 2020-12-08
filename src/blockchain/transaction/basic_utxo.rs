use super::{LockingScript, UnspentTransactionOutput};

pub struct BasicUTXO {
    amount: f32,
    locking_script: Box<dyn LockingScript>,
    source_addr: Vec<u8>,
    destination_addr: Vec<u8>,
}

impl BasicUTXO {
    fn create_transaction(
        amount: f32,
        locking_script: Box<dyn LockingScript>,
        destination_addr: Vec<u8>,
        source_addr: Vec<u8>,
    ) -> Result<Self, &'static str> {
        if amount < 0.0 {
            return Err("The amount has to be a non-negative float.");
        } else {
            Ok(Self {
                amount,
                locking_script,
                source_addr,
                destination_addr,
            })
        }
    }
}

impl UnspentTransactionOutput for BasicUTXO {
    fn get_amount(&self) -> f32 {
        return self.amount;
    }

    fn execute_locking_script(&self, input: Vec<u8>) -> bool {
        return self.locking_script.execute_script(input);
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(&self.amount.to_ne_bytes());
        bytes.extend(self.locking_script.as_bytes());
        return bytes;
    }

    fn get_source_addr(&self) -> &[u8] {
        return &self.source_addr;
    }

    fn get_destination_addr(&self) -> &[u8] {
        return &self.destination_addr;
    }
}
