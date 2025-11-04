//! Python bindings for the LAME MP3 encoder
//!
//! This module provides Python bindings for LAME, a high-quality MP3 encoder.
//! It wraps the `lame-sys` Rust crate with a Python-friendly API.
//!
//! # Features
//!
//! - Safe, high-level API for MP3 encoding
//! - Automatic GIL release for concurrent encoding
//! - Support for mono and stereo encoding
//! - ID3 tag support
//! - VBR and CBR encoding modes
//! - Quality presets
//!
//! # Example
//!
//! ```python
//! import lame
//!
//! # Create encoder
//! encoder = (
//!     lame.LameEncoder.builder()
//!     .sample_rate(44100)
//!     .channels(1)
//!     .bitrate(128)
//!     .quality(lame.Quality.Standard)
//!     .build()
//! )
//!
//! # Set ID3 tags
//! encoder.id3_tag()
//!     .title("My Song")
//!     .artist("My Artist")
//!     .apply()
//!
//! # Encode PCM data
//! pcm_data = [0] * 1152
//! mp3_data = encoder.encode_mono(pcm_data)
//!
//! # Flush encoder
//! final_data = encoder.flush()
//! ```

mod builder;
mod encoder;
mod enums;
mod error;
mod id3;
mod utils;

use pyo3::prelude::*;

/// LAME MP3 Encoder for Python
///
/// This module provides Python bindings for the LAME MP3 encoder library.
#[pymodule]
fn lame(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add classes
    m.add_class::<encoder::LameEncoder>()?;
    m.add_class::<builder::EncoderBuilder>()?;
    m.add_class::<enums::Quality>()?;
    m.add_class::<enums::VbrMode>()?;
    m.add_class::<id3::Id3Tag>()?;

    // Add exceptions
    error::register_exceptions(m)?;

    // Add utility functions
    m.add_function(wrap_pyfunction!(utils::get_version, m)?)?;
    m.add_function(wrap_pyfunction!(utils::get_url, m)?)?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add(
        "__doc__",
        "Python bindings for LAME MP3 encoder with high-level API",
    )?;

    Ok(())
}
