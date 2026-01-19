use toon_rust::cli::json_stream::json_stream_from_events;
use toon_rust::cli::json_stringify::json_stringify_lines;
use toon_rust::{JsonStreamEvent, JsonValue, StringOrNumberOrBoolOrNull};

#[test]
fn json_stringify_lines_matches_serde_for_compact() {
    let value = JsonValue::Object(vec![
        (
            "a".to_string(),
            JsonValue::Primitive(StringOrNumberOrBoolOrNull::Number(1.0)),
        ),
        (
            "b".to_string(),
            JsonValue::Array(vec![
                JsonValue::Primitive(StringOrNumberOrBoolOrNull::Bool(true)),
                JsonValue::Primitive(StringOrNumberOrBoolOrNull::String("x".to_string())),
            ]),
        ),
    ]);

    let chunks = json_stringify_lines(&value, 0);
    let actual = chunks.concat();

    let expected = serde_json::to_string(&serde_value(&value)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_lines_matches_serde_for_pretty() {
    let value = JsonValue::Array(vec![
        JsonValue::Primitive(StringOrNumberOrBoolOrNull::Null),
        JsonValue::Primitive(StringOrNumberOrBoolOrNull::Number(3.5)),
        JsonValue::Object(vec![(
            "key".to_string(),
            JsonValue::Primitive(StringOrNumberOrBoolOrNull::String("value".to_string())),
        )]),
    ]);

    let chunks = json_stringify_lines(&value, 2);
    let actual = chunks.concat();

    let expected = serde_json::to_string_pretty(&serde_value(&value)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn json_stream_from_events_matches_stringify() {
    let events = vec![
        JsonStreamEvent::StartObject,
        JsonStreamEvent::Key {
            key: "a".to_string(),
            was_quoted: false,
        },
        JsonStreamEvent::Primitive {
            value: StringOrNumberOrBoolOrNull::Number(1.0),
        },
        JsonStreamEvent::Key {
            key: "b".to_string(),
            was_quoted: false,
        },
        JsonStreamEvent::StartArray { length: 2 },
        JsonStreamEvent::Primitive {
            value: StringOrNumberOrBoolOrNull::Bool(true),
        },
        JsonStreamEvent::Primitive {
            value: StringOrNumberOrBoolOrNull::String("x".to_string()),
        },
        JsonStreamEvent::EndArray,
        JsonStreamEvent::EndObject,
    ];

    let actual = json_stream_from_events(events, 2).unwrap().concat();

    let value = JsonValue::Object(vec![
        (
            "a".to_string(),
            JsonValue::Primitive(StringOrNumberOrBoolOrNull::Number(1.0)),
        ),
        (
            "b".to_string(),
            JsonValue::Array(vec![
                JsonValue::Primitive(StringOrNumberOrBoolOrNull::Bool(true)),
                JsonValue::Primitive(StringOrNumberOrBoolOrNull::String("x".to_string())),
            ]),
        ),
    ]);
    let expected = json_stringify_lines(&value, 2).concat();

    assert_eq!(actual, expected);
}

#[test]
fn json_stream_from_events_rejects_mismatched_end() {
    let events = vec![JsonStreamEvent::EndObject];
    let err = json_stream_from_events(events, 0).unwrap_err();
    assert!(err.to_string().contains("Mismatched endObject"));
}

fn serde_value(value: &JsonValue) -> serde_json::Value {
    match value {
        JsonValue::Primitive(primitive) => match primitive {
            StringOrNumberOrBoolOrNull::Null => serde_json::Value::Null,
            StringOrNumberOrBoolOrNull::Bool(value) => serde_json::Value::Bool(*value),
            StringOrNumberOrBoolOrNull::Number(value) => serde_json::Number::from_f64(*value)
                .map_or(serde_json::Value::Null, serde_json::Value::Number),
            StringOrNumberOrBoolOrNull::String(value) => serde_json::Value::String(value.clone()),
        },
        JsonValue::Array(values) => {
            serde_json::Value::Array(values.iter().map(serde_value).collect())
        }
        JsonValue::Object(entries) => {
            let mut map = serde_json::Map::new();
            for (key, value) in entries {
                map.insert(key.clone(), serde_value(value));
            }
            serde_json::Value::Object(map)
        }
    }
}
