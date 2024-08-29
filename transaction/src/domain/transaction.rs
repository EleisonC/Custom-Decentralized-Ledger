use chrono::{DateTime, Utc};

use super::Email;


#[derive(Clone, Debug)]
pub struct Transaction {
    pub  sender: Email,
    pub recipient: Email,
    pub amount: u64,
    pub txid: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>
}

impl Transaction {
    pub fn new(sender: Email, recipient: Email, amount: u64) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            txid: None,
            timestamp: Utc::now(),
            signature: None,
        }
    }
}