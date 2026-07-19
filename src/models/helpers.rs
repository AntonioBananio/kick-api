//! Helper functions for flexible deserialization.

use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

/// Десериализует поле как u64, принимая как число, так и строку
pub fn deserialize_u64_or_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Number(n) => n.as_u64().ok_or_else(|| {
            serde::de::Error::custom("expected u64 or string")
        }),
        Value::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected u64 or string")),
    }
}

/// Десериализует поле как u32, принимая как число, так и строку
pub fn deserialize_u32_or_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Number(n) => n.as_u64().map(|v| v as u32).ok_or_else(|| {
            serde::de::Error::custom("expected u32 or string")
        }),
        Value::String(s) => s.parse::<u32>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected u32 or string")),
    }
}

/// Десериализует поле как Option<u64>, принимая null, число или строку
pub fn deserialize_option_u64_or_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::Number(n) => Ok(n.as_u64()),
        Value::String(s) => Ok(s.parse::<u64>().ok()),
        _ => Err(serde::de::Error::custom("expected null, u64 or string")),
    }
}

/// Десериализует поле как String, принимая как строку, так и число
pub fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}

/// Десериализует поле как Option<String>, принимая null, строку или число
pub fn deserialize_option_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) => Ok(Some(s)),
        Value::Number(n) => Ok(Some(n.to_string())),
        _ => Err(serde::de::Error::custom("expected null, string or number")),
    }
}