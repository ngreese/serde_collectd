//! Module defining collectd binary format.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Enum representing the different parts of a packet.
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Parts {
    Host = 0x0000,
    Time = 0x0001,
    TimeHires = 0x0008,
    Plugin = 0x0002,
    PluginInstance = 0x0003,
    Type = 0x0004,
    TypeInstance = 0x0005,
    Values = 0x0006,
    Interval = 0x0007,
    IntervalHires = 0x0009,
}

/// Enum representing the different types of values.
#[derive(Debug)]
pub enum ValueKind {
    Counter(u64),
    Gauge(f64),
    Derive(i64),
    Absolute(u64),
}

/// Struct representing a value from collectd.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Value {
    time: Option<u64>,
    interval: Option<u64>,
    host: Option<String>,
    plugin: Option<String>,
    plugin_instance: Option<String>,
    r#type: Option<String>,
    type_instance: Option<String>,
    values: Vec<Value>,
}
impl Value {
    /// Function to parse a collectd packet.
    pub fn from_bytes(mut buf: Bytesg) -> Result<Self, ()> {

        Ok(Value::default())
    }
}
