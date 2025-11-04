use pyo3::exceptions::{PyException, PyRuntimeError, PyValueError};
use pyo3::{create_exception, prelude::*};

// Create custom exception types
create_exception!(lame, LameError, PyException);
create_exception!(lame, InitializationError, LameError);
create_exception!(lame, InvalidParameterError, LameError);
create_exception!(lame, EncodingError, LameError);
create_exception!(lame, BufferTooSmallError, LameError);

/// Convert Rust LameError to Python exception
pub fn to_py_err(err: lame_sys::LameError) -> PyErr {
    match err {
        lame_sys::LameError::InitializationFailed => {
            PyErr::new::<PyRuntimeError, _>("LAME initialization failed")
        }
        lame_sys::LameError::InvalidParameter(msg) => {
            PyErr::new::<PyValueError, _>(format!("Invalid parameter: {}", msg))
        }
        lame_sys::LameError::EncodingFailed(code) => {
            PyErr::new::<PyRuntimeError, _>(format!("Encoding failed with code: {}", code))
        }
        lame_sys::LameError::BufferTooSmall { required, provided } => {
            PyErr::new::<PyRuntimeError, _>(format!(
                "Buffer too small: required {} bytes, provided {} bytes",
                required, provided
            ))
        }
        lame_sys::LameError::OutOfMemory => {
            PyErr::new::<PyRuntimeError, _>("Out of memory")
        }
        lame_sys::LameError::InvalidInput(msg) => {
            PyErr::new::<PyValueError, _>(format!("Invalid input: {}", msg))
        }
        lame_sys::LameError::InternalError(msg) => {
            PyErr::new::<PyRuntimeError, _>(format!("Internal error: {}", msg))
        }
        lame_sys::LameError::Utf8Error(e) => {
            PyErr::new::<PyValueError, _>(format!("UTF-8 error: {}", e))
        }
        lame_sys::LameError::NullPointer => {
            PyErr::new::<PyRuntimeError, _>("Null pointer error")
        }
    }
}

/// Register exception classes with Python module
pub fn register_exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("LameError", m.py().get_type_bound::<LameError>())?;
    m.add(
        "InitializationError",
        m.py().get_type_bound::<InitializationError>(),
    )?;
    m.add(
        "InvalidParameterError",
        m.py().get_type_bound::<InvalidParameterError>(),
    )?;
    m.add("EncodingError", m.py().get_type_bound::<EncodingError>())?;
    m.add(
        "BufferTooSmallError",
        m.py().get_type_bound::<BufferTooSmallError>(),
    )?;
    Ok(())
}
