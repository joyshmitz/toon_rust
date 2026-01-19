#![forbid(unsafe_code)]

pub mod cli;
pub mod decode;
pub mod encode;
pub mod error;
pub mod options;
pub mod shared;

pub use decode::{decode, decode_from_lines, decode_stream, decode_stream_sync};
pub use encode::{encode, encode_lines};
pub use options::{
    DecodeOptions, DecodeStreamOptions, EncodeOptions, EncodeReplacer, ResolvedDecodeOptions,
    ResolvedEncodeOptions,
};

pub type JsonPrimitive = StringOrNumberOrBoolOrNull;
pub type JsonObject = Vec<(String, JsonValue)>;
pub type JsonArray = Vec<JsonValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Primitive(JsonPrimitive),
    Array(JsonArray),
    Object(JsonObject),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonStreamEvent {
    StartObject,
    EndObject,
    StartArray { length: usize },
    EndArray,
    Key { key: String, was_quoted: bool },
    Primitive { value: JsonPrimitive },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringOrNumberOrBoolOrNull {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl StringOrNumberOrBoolOrNull {
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        if !value.is_finite() {
            return Self::Null;
        }
        if value == 0.0 {
            return Self::Number(0.0);
        }
        Self::Number(value)
    }
}

impl From<StringOrNumberOrBoolOrNull> for JsonValue {
    fn from(value: StringOrNumberOrBoolOrNull) -> Self {
        Self::Primitive(value)
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        Self::Primitive(StringOrNumberOrBoolOrNull::String(value))
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        Self::Primitive(StringOrNumberOrBoolOrNull::String(value.to_string()))
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        Self::Primitive(StringOrNumberOrBoolOrNull::Bool(value))
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        Self::Primitive(StringOrNumberOrBoolOrNull::from_f64(value))
    }
}

#[allow(clippy::cast_precision_loss)]
impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        Self::Primitive(StringOrNumberOrBoolOrNull::Number(value as f64))
    }
}

#[allow(clippy::use_self)]
impl From<Vec<JsonValue>> for JsonValue {
    fn from(value: Vec<JsonValue>) -> Self {
        Self::Array(value)
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        Self::Object(value)
    }
}

impl From<serde_json::Value> for JsonValue {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Primitive(StringOrNumberOrBoolOrNull::Null),
            serde_json::Value::Bool(value) => {
                Self::Primitive(StringOrNumberOrBoolOrNull::Bool(value))
            }
            serde_json::Value::Number(value) => {
                let number = value
                    .as_f64()
                    .unwrap_or_else(|| value.to_string().parse::<f64>().unwrap_or(f64::NAN));
                Self::Primitive(StringOrNumberOrBoolOrNull::from_f64(number))
            }
            serde_json::Value::String(value) => {
                Self::Primitive(StringOrNumberOrBoolOrNull::String(value))
            }
            serde_json::Value::Array(values) => {
                Self::Array(values.into_iter().map(Self::from).collect())
            }
            serde_json::Value::Object(map) => {
                let mut entries = Vec::with_capacity(map.len());
                for (key, value) in map {
                    entries.push((key, Self::from(value)));
                }
                Self::Object(entries)
            }
        }
    }
}
