pub mod encoders;
pub mod folding;
pub mod normalize;
pub mod primitives;
pub mod replacer;

use crate::encode::normalize::normalize_json_value;
use crate::encode::replacer::apply_replacer;
use crate::options::{EncodeOptions, resolve_encode_options};
use crate::{JsonStreamEvent, JsonValue};

pub fn encode(input: impl Into<JsonValue>, options: Option<EncodeOptions>) -> String {
    let lines = encode_lines(input, options);
    lines.join("\n")
}

pub fn encode_lines(input: impl Into<JsonValue>, options: Option<EncodeOptions>) -> Vec<String> {
    let resolved = resolve_encode_options(options);
    let normalized = normalize_json_value(input.into());
    let replaced = if let Some(replacer) = &resolved.replacer {
        apply_replacer(&normalized, replacer)
    } else {
        normalized
    };
    encoders::encode_json_value(&replaced, &resolved)
}

pub fn encode_stream_events(
    _input: impl Into<JsonValue>,
    _options: Option<EncodeOptions>,
) -> Vec<JsonStreamEvent> {
    todo!("encode_stream_events not implemented")
}
