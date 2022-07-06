mod context;

pub use context::Context;
pub(crate) use context::GLOBAL_CONTEXT;

use crate::plugins::InterpFunction;

pub type LogErrorHandlerFunction = fn(&mut Context, ErrorCode, String);

pub fn default_log_error_handler_function(_context: &mut Context, _code: ErrorCode, _text: String) {
}

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
