use async_trait::async_trait;
use aws_smithy_client::SdkError;
use bytes::Bytes;

use super::KinesisResponse;
/// An AWS Kinesis record type primarily to store the underlying aws crates' actual record `T`, and
/// to abstract the encoded length calculation.
pub trait Record {
    type T;

    /// Create a new instance of this record.
    fn new(payload_bytes: &Bytes, partition_key: &str) -> Self;

    /// Returns the encoded length of the record.
    fn encoded_length(&self) -> usize;

    /// Moves the contained record to the caller.
    fn get(self) -> Self::T;
}

/// Capable of sending records.
#[async_trait]
pub trait SendRecord {
    type T;
    type E;

    /// Sends the records.
    async fn send(
        &self,
        records: Vec<Self::T>,
        stream_name: String,
    ) -> Result<KinesisResponse, SdkError<Self::E>>;
}
