use crate::encoder::LameEncoder;
use crate::enums::{Quality, VbrMode};
use crate::error::to_py_err;
use pyo3::prelude::*;

/// Builder for configuring and creating a LameEncoder
///
/// # Example
///
/// ```python
/// builder = LameEncoder.builder()
/// builder.sample_rate(44100)
/// builder.channels(2)
/// builder.bitrate(128)
/// builder.quality(Quality.Standard)
/// encoder = builder.build()
/// ```
#[pyclass(unsendable)]
pub struct EncoderBuilder {
    inner: Option<lame_sys::EncoderBuilder>,
}

#[pymethods]
impl EncoderBuilder {
    /// Create a new encoder builder with default settings
    #[new]
    pub fn new() -> PyResult<Self> {
        let inner = lame_sys::LameEncoder::builder().map_err(to_py_err)?;
        Ok(Self { inner: Some(inner) })
    }

    /// Set the input sample rate in Hz
    ///
    /// Common values: 44100, 48000, 32000, 22050, 16000
    fn sample_rate(&mut self, rate: i32) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.sample_rate(rate).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Set the number of input channels (1 or 2)
    ///
    /// 1 = mono, 2 = stereo
    fn channels(&mut self, channels: i32) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.channels(channels).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Set the output bitrate in kbps
    ///
    /// Common values: 320, 256, 192, 128, 96, 64
    fn bitrate(&mut self, bitrate: i32) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.bitrate(bitrate).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Set the encoding quality level
    ///
    /// Higher quality means slower encoding but better audio quality.
    fn quality(&mut self, quality: Quality) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.quality(quality.into()).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Set the VBR (Variable Bit Rate) mode
    fn vbr_mode(&mut self, mode: VbrMode) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.vbr_mode(mode.into()).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Set the VBR quality (0=best, 9=worst)
    ///
    /// Only effective when VBR mode is enabled.
    fn vbr_quality(&mut self, quality: i32) -> PyResult<()> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let builder = builder.vbr_quality(quality).map_err(to_py_err)?;
        self.inner = Some(builder);
        Ok(())
    }

    /// Build and initialize the encoder
    ///
    /// Returns a configured LameEncoder ready for encoding.
    fn build(&mut self) -> PyResult<LameEncoder> {
        let builder = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Builder already consumed")
        })?;
        let inner = builder.build().map_err(to_py_err)?;
        Ok(LameEncoder {
            inner,
            mp3_buffer: Vec::new(), // Will grow on first use
        })
    }

    fn __repr__(&self) -> String {
        "EncoderBuilder()".to_string()
    }
}
