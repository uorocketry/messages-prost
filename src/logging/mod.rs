// This module will contain logging-related data types and logic.

use crate::messages::{ErrorContext, LogLevel, LogMessage};

pub fn log_info(message: &str) -> LogMessage {
    LogMessage {
        level: LogLevel::Info.into(),
        context: 0, // UNSPECIFIED
        message: message.into(),
    }
}

pub fn log_error(context: ErrorContext, message: &str) -> LogMessage {
    LogMessage {
        level: LogLevel::Error.into(),
        context: context.into(),
        message: message.into(),
    }
}

pub fn log_debug(message: &str) -> LogMessage {
    LogMessage {
        level: LogLevel::Debug.into(),
        context: 0, // UNSPECIFIED
        message: message.into(),
    }
}

pub fn log_warning(message: &str) -> LogMessage {
    LogMessage {
        level: LogLevel::Warn.into(),
        context: 0, // UNSPECIFIED
        message: message.into(),
    }
}
