use pyo3::prelude::*;

/// Get the LAME version string
///
/// Returns:
///     Version string of the LAME library
///
/// # Example
///
/// ```python
/// import lame
/// print(lame.get_version())  # e.g., "3.101"
/// ```
#[pyfunction]
pub fn get_version() -> String {
    lame_sys::get_lame_version()
}

/// Get the LAME project URL
///
/// Returns:
///     URL of the LAME project website
#[pyfunction]
pub fn get_url() -> String {
    lame_sys::get_lame_url()
}
