//! Module defining collectd binary format.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Enum representing the different parts of a packet.
///
/// Reference https://github.com/logstash-plugins/logstash-codec-collectd
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
enum TypeMap {
    Host = 0x0000,
    Time = 0x0001,
    PluginType = 0x0002,
    PluginInstance = 0x0003,
    CollectdType = 0x0004,
    TypeInstance = 0x0005,
    Values = 0x0006,
    Interval = 0x0007,
    Timestamp = 0x0008,
    IntervalInstance = 0x0009,
    Message = 0x0100,
    Severity = 0x0101,
    Signature = 0x0200,
    Encryption = 0x0210,
}

/// Enum representing the different types of values.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ValueKind {
    Counter(u64),
    Gauge(f64),
    Derive(i64),
    Absolute(u64),
}

/// Enum representing the Value type in the packet.
///
/// Reference https://github.com/logstash-plugins/logstash-codec-collectd
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(u16)]
enum ValueMap {
    Counter = 0x000,
    Gauge = 0x001,
    Derive = 0x002,
    Absolute = 0x003,
}

/// Enum representing either a single value or multiple values
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
enum Values {
    Value(ValueKind),
    Values(Vec<ValueKind>),
}

impl Default for Values {
    fn default() -> Self {
        Values::Values(Vec::new())
    }
}

/// Struct representing a value from collectd.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Value {
    time: Option<u64>,
    interval: Option<u64>,
    host: Option<String>,
    plugin: Option<String>,
    plugin_instance: Option<String>,
    r#type: Option<String>,
    type_instance: Option<String>,
    values: Values,
}
impl Value {
    /// Function to parse a collectd packet.
    pub fn from_bytes(mut buf: Bytes) -> Result<Self, ()> {
        Ok(Value::default())
    }
}
