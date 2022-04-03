#[derive(Copy, Clone)]
pub struct LogErrorHandler {
    pub handler: Option<LogErrorHandlerFunction>,
}

impl LogErrorHandler {
    pub fn new(func: Option<LogErrorHandlerFunction>) -> Self {
        Self { handler: func }
    }
}

pub type LogErrorHandlerFunction = fn(ErrorCode, String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
#[non_exhaustive]
pub enum ErrorCode {
    Undefined = 0,
    File = 1,
    Range = 2,
    Internal = 3,
    Null = 4,
    Read = 5,
    Seek = 6,
    Write = 7,
    UnknownExtension = 8,
    ColorSpaceCheck = 9,
    AlreadyDefined = 10,
    BadSignature = 11,
    CorruptionDetected = 12,
    NotSuitable = 13,
}

pub fn default_log_error_handler_function(_code: ErrorCode, _text: String) {}

pub const DEFAULT_LOG_ERROR_HANDLER: LogErrorHandler = LogErrorHandler {
    handler: Some(default_log_error_handler_function),
};
