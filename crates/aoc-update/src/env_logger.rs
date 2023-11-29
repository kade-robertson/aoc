use serde::{de::Visitor, Deserialize};
use std::io::{stdin, IsTerminal};
use tracing_subscriber::prelude::*;

use crate::env::Environment;

#[derive(Debug)]
pub enum LogFormat {
    Full,
    Compact,
    Pretty,
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        if stdin().is_terminal() {
            LogFormat::Full
        } else {
            LogFormat::Json
        }
    }
}

struct LogFormatVisitor;
impl<'de> Visitor<'de> for LogFormatVisitor {
    type Value = LogFormat;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A string representing the log format (full, compact, pretty, json)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.to_lowercase().as_str() {
            "full" => Ok(LogFormat::Full),
            "compact" => Ok(LogFormat::Compact),
            "pretty" => Ok(LogFormat::Pretty),
            "json" => Ok(LogFormat::Json),
            _ => Err(E::custom(format!(
                "Invalid log format: {}. Valid values are full, compact, pretty, json",
                v
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for LogFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(LogFormatVisitor)
    }
}

impl Environment {
    pub fn init_logger(&self) {
        macro_rules! base_logger {
            ($f:expr) => {
                tracing_subscriber::registry()
                    .with(tracing_subscriber::EnvFilter::new(self.log_level.clone()))
                    .with($f)
                    .init()
            };
        }

        match self.log_format {
            LogFormat::Full => base_logger!(tracing_subscriber::fmt::layer()),
            LogFormat::Compact => base_logger!(tracing_subscriber::fmt::layer().compact()),
            LogFormat::Pretty => base_logger!(tracing_subscriber::fmt::layer().pretty()),
            LogFormat::Json => base_logger!(tracing_subscriber::fmt::layer().json()),
        };
    }
}
