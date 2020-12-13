use super::{Transaction, UnlockingScript, UnspentTransactionOutput};
use derive_new::new;

#[derive(new)]
pub struct BasicTransaction<U: UnspentTransactionOutput> {
    signed_input_transactions: Vec<(Box<dyn UnlockingScript>, U)>,
    output_transactions: Vec<U>,
}

impl <U: UnspentTransactionOutput> Transaction for BasicTransaction<U> {
    fn verify(&self) -> bool {
        for (s, u) in &self.signed_input_transactions {
            let unlock = s.execute_script();
            if !u.execute_locking_script(unlock) {
                return false;
            }
        }
        return true;
    }

    fn as_bytes(&self) -> Vec<u8> {
        let input_transaction_bytes = self.signed_input_transactions.iter()
                                    .flat_map(|(_, t)| t.as_bytes());
        let unlocking_script_bytes = self.signed_input_transactions.iter()
                                    .flat_map(|(s, _)| s.as_bytes());
        let output_transaction_bytes = self.output_transactions.iter()
                                    .flat_map(|t| t.as_bytes());

        let mut bytes = vec![];
        bytes.extend(input_transaction_bytes);
        bytes.extend(unlocking_script_bytes);
        bytes.extend(output_transaction_bytes);
        return bytes;
    }

    fn get_spent_utxos(&self) -> &Vec<Box<dyn UnspentTransactionOutput>> {
        // TODO
        unimplemented!();
    }
}