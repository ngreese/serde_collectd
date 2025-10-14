//! Define error types and behavior.

/// Error kinds for parsing collectd.
pub enum ErrorKind {
    ProtocolError,
    HeaderError,
    EncryptionError,
    NaNError,
}