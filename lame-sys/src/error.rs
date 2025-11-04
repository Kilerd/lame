use std::fmt;
use std::error::Error;

/// LAME 编码器错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LameError {
    /// 初始化失败
    InitializationFailed,

    /// 参数设置失败
    InvalidParameter(String),

    /// 编码失败
    EncodingFailed(i32),

    /// 缓冲区太小
    BufferTooSmall { required: usize, provided: usize },

    /// 内存分配失败
    OutOfMemory,

    /// 无效的输入数据
    InvalidInput(String),

    /// 内部错误
    InternalError(String),

    /// UTF-8 字符串转换错误
    Utf8Error(std::str::Utf8Error),

    /// 空指针错误
    NullPointer,
}

impl fmt::Display for LameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LameError::InitializationFailed => {
                write!(f, "Failed to initialize LAME encoder")
            }
            LameError::InvalidParameter(msg) => {
                write!(f, "Invalid parameter: {}", msg)
            }
            LameError::EncodingFailed(code) => {
                write!(f, "Encoding failed with code: {}", code)
            }
            LameError::BufferTooSmall { required, provided } => {
                write!(
                    f,
                    "Output buffer too small: required {} bytes, provided {} bytes",
                    required, provided
                )
            }
            LameError::OutOfMemory => {
                write!(f, "Out of memory")
            }
            LameError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
            LameError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            }
            LameError::Utf8Error(err) => {
                write!(f, "UTF-8 conversion error: {}", err)
            }
            LameError::NullPointer => {
                write!(f, "Unexpected null pointer")
            }
        }
    }
}

impl Error for LameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LameError::Utf8Error(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::str::Utf8Error> for LameError {
    fn from(err: std::str::Utf8Error) -> Self {
        LameError::Utf8Error(err)
    }
}

impl From<std::ffi::NulError> for LameError {
    fn from(_: std::ffi::NulError) -> Self {
        LameError::InvalidInput("String contains null byte".to_string())
    }
}

/// Result 类型别名
pub type Result<T> = std::result::Result<T, LameError>;
