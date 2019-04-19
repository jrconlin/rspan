use protobuf::well_known_types::Any;
use google::rpc::status;

/// Representation of status of result of operations
pub struct Status {
    pub code: i32,
    pub message: String,
    pub details: Vec<Any>,
}

impl Status {
    /// Construct a `Status` struct from a protobuf
    pub fn from_pb(mut status_pb: status::Status) -> Self {
        Status {
            code: status_pb.get_code(),
            message: status_pb.take_message(),
            details: status_pb.take_details().into_vec()
        }
    }
}