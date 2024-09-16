
#[derive(Debug)]
pub enum TransactionAPIErrors {
    InvalidInformation,
    TransactionNotFound,
    UnexpectedError,
    InvalidIndex,
    FailedToSignTransaction,
    SigningError
}