use grpcio::Error;

/// Representation of long running operations
pub struct Operation {
    pub name: String,
    pub done: bool,
    pub result: OperationResult,
}

/// Type of operation result
pub enum OperationResult {
    Error(Error),
    Response,
}