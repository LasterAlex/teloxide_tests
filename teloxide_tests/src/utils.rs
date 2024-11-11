use serde_json::Value;
use teloxide::prelude::*;
use teloxide::types::FileMeta;

macro_rules! assert_eqn {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&$actual, &$expected) {
            (actual, expected) => {
                if !(*actual == *expected) {
                    panic!("assertion `actual == expected` failed:
   actual: {actual:?}
 expected: {expected:?}", actual=&*actual, expected=&*expected)

                }
            }
        }
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        match (&$actual, &$expected) {
            (actual, expected) => {
                if !(*actual == *expected) {
                    panic!("assertion `actual == expected` failed: {message}
   actual: {actual:?}
 expected: {expected:?}", message=$($arg)+, actual=&*actual, expected=&*expected)

                }
            }
        }
    };
}

pub(crate) use assert_eqn;

pub fn find_file(value: Value) -> Option<FileMeta> {
    // Recursively searches for file meta
    let mut file_id = None;
    let mut file_unique_id = None;
    let mut file_size = None;
    if let Value::Object(map) = value {
        for (k, v) in map {
            if k == "file_id" {
                file_id = Some(v.as_str().unwrap().to_string());
            } else if k == "file_unique_id" {
                file_unique_id = Some(v.as_str().unwrap().to_string());
            } else if k == "file_size" {
                file_size = Some(v.as_u64().unwrap() as u32);
            } else if let Some(found) = find_file(v) {
                return Some(found);
            }
        }
    }
    if let (Some(id), Some(unique_id)) = (file_id, file_unique_id) {
        return Some(FileMeta {
            id,
            unique_id,
            size: file_size.unwrap_or(0),
        });
    }
    None
}

pub fn find_chat_id(value: Value) -> Option<i64> {
    // Recursively searches for chat id
    if let Value::Object(map) = value {
        for (k, v) in map {
            if k == "chat" {
                return v["id"].as_i64();
            } else if let Some(found) = find_chat_id(v) {
                return Some(found);
            }
        }
    }
    None
}

/// A key that defines the parallelism of updates
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DistributionKey(pub ChatId);

pub(crate) fn default_distribution_function(update: &Update) -> Option<DistributionKey> {
    update.chat().map(|c| c.id).map(DistributionKey)
}
