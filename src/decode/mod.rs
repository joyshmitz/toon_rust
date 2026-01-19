pub mod decoders;
pub mod event_builder;
pub mod expand;
pub mod parser;
pub mod scanner;
pub mod validation;

use crate::decode::decoders as decoder_impl;
use crate::decode::event_builder::{build_node_from_events, node_to_json};
use crate::decode::expand::expand_paths_safe;
use crate::options::{DecodeOptions, DecodeStreamOptions, ExpandPathsMode, resolve_decode_options};
use crate::{JsonStreamEvent, JsonValue};

#[must_use]
pub fn decode(input: &str, options: Option<DecodeOptions>) -> JsonValue {
    let lines = input
        .split('\n')
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>();
    decode_from_lines(lines, options)
}

#[must_use]
/// Decode TOON lines into a JSON value.
///
/// # Panics
///
/// Panics if decoding fails due to malformed input or strict-mode validation errors.
pub fn decode_from_lines(
    lines: impl IntoIterator<Item = String>,
    options: Option<DecodeOptions>,
) -> JsonValue {
    let resolved = resolve_decode_options(options);
    let events = decoder_impl::decode_stream_sync(
        lines,
        Some(DecodeStreamOptions {
            indent: Some(resolved.indent),
            strict: Some(resolved.strict),
        }),
    )
    .unwrap_or_else(|err| panic!("{err}"));

    let mut node = build_node_from_events(events).unwrap_or_else(|err| panic!("{err}"));

    if resolved.expand_paths == ExpandPathsMode::Safe {
        node = expand_paths_safe(node, resolved.strict).unwrap_or_else(|err| panic!("{err}"));
    }

    node_to_json(node)
}

#[must_use]
/// Decode TOON lines into a stream of events.
///
/// # Panics
///
/// Panics if decoding fails due to malformed input or strict-mode validation errors.
pub fn decode_stream_sync(
    lines: impl IntoIterator<Item = String>,
    options: Option<DecodeStreamOptions>,
) -> Vec<JsonStreamEvent> {
    decoder_impl::decode_stream_sync(lines, options).unwrap_or_else(|err| panic!("{err}"))
}

#[must_use]
/// Decode TOON lines into a stream of events asynchronously.
///
/// # Panics
///
/// Panics if decoding fails due to malformed input or strict-mode validation errors.
pub async fn decode_stream(
    lines: impl IntoIterator<Item = String>,
    options: Option<DecodeStreamOptions>,
) -> Vec<JsonStreamEvent> {
    decoder_impl::decode_stream_sync(lines, options).unwrap_or_else(|err| panic!("{err}"))
}
