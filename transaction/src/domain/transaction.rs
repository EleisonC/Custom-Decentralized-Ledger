use chrono::{DateTime, Utc};


#[derive(Clone, Debug)]
pub struct Transaction {
    pub  sender: String,
    pub recipient: String,
    pub amount: u64,
    pub txid: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
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