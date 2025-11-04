use crate::builder::EncoderBuilder;
use crate::error::to_py_err;
use crate::id3::Id3Tag;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// LAME MP3 Encoder
///
/// Encoder that automatically releases the GIL during encoding
/// operations, allowing other Python threads to run concurrently.
///
/// Note: Each encoder instance should only be used from a single Python thread.
///
/// # Example
///
/// ```python
/// encoder = (
///     LameEncoder.builder()
///     .sample_rate(44100)
///     .channels(1)
///     .bitrate(128)
///     .build()
/// )
///
/// pcm = [0] * 1152  # Input PCM samples
/// mp3_data = encoder.encode_mono(pcm)
/// final_data = encoder.flush()
/// ```
#[pyclass(unsendable)]
pub struct LameEncoder {
    pub(crate) inner: lame_sys::LameEncoder,
    // Reusable buffer for MP3 output to avoid repeated allocations
    pub(crate) mp3_buffer: Vec<u8>,
}

#[pymethods]
impl LameEncoder {
    /// Create a new encoder builder
    ///
    /// Returns an EncoderBuilder for configuring encoder parameters.
    #[staticmethod]
    fn builder() -> PyResult<EncoderBuilder> {
        EncoderBuilder::new()
    }

    /// Encode stereo PCM data from bytes (for backward compatibility)
    ///
    /// Args:
    ///     pcm_left: Left channel samples as bytes (i16 little-endian)
    ///     pcm_right: Right channel samples as bytes (i16 little-endian)
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: For best performance, use `encode_numpy()` instead.
    /// Releases the GIL during encoding for better concurrency.
    fn encode<'py>(
        &mut self,
        py: Python<'py>,
        pcm_left: &Bound<'py, PyBytes>,
        pcm_right: &Bound<'py, PyBytes>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // Get read-only byte slices from PyBytes
        let left_bytes = pcm_left.as_bytes();
        let right_bytes = pcm_right.as_bytes();

        // ✅ Use bytemuck for safe type conversion with alignment checking
        let pcm_left_slice: &[i16] = bytemuck::try_cast_slice(left_bytes).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Left channel PCM data must be properly aligned for i16",
            )
        })?;
        let pcm_right_slice: &[i16] = bytemuck::try_cast_slice(right_bytes).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Right channel PCM data must be properly aligned for i16",
            )
        })?;

        // Ensure buffer is large enough (reuse if possible)
        let required_size = pcm_left_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure
        let left_vec = pcm_left_slice.to_vec();
        let right_vec = pcm_right_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        // Release GIL during encoding
        let bytes_written = py.allow_threads(move || {
            // SAFETY: We hold a mutable reference to self, so no other thread can access it
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder
                .encode(&left_vec, &right_vec, mp3_buffer)
                .map_err(to_py_err)
        })?;

        // Return only the written portion as Python bytes
        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Encode interleaved stereo PCM data from bytes (for backward compatibility)
    ///
    /// Args:
    ///     pcm_interleaved: Interleaved samples as bytes (L, R, L, R, ... in i16 little-endian)
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: For best performance, use `encode_interleaved_numpy()` instead.
    /// Releases the GIL during encoding for better concurrency.
    fn encode_interleaved<'py>(
        &mut self,
        py: Python<'py>,
        pcm_interleaved: &Bound<'py, PyBytes>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // Get read-only byte slice from PyBytes
        let pcm_bytes = pcm_interleaved.as_bytes();

        // ✅ Use bytemuck for safe type conversion with alignment checking
        let pcm_slice: &[i16] = bytemuck::try_cast_slice(pcm_bytes).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "PCM data must be properly aligned for i16 (length must be even)",
            )
        })?;

        // Ensure buffer is large enough (reuse if possible)
        let required_size = pcm_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure
        let pcm_vec = pcm_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        let bytes_written = py.allow_threads(move || {
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder
                .encode_interleaved(&pcm_vec, mp3_buffer)
                .map_err(to_py_err)
        })?;

        // Return only the written portion as Python bytes
        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Encode mono PCM data from bytes (for backward compatibility)
    ///
    /// Args:
    ///     pcm: Mono samples as bytes (i16 little-endian format)
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: For best performance, use `encode_mono_numpy()` instead.
    /// This method converts bytes to i16 and clones data for thread safety.
    /// Releases the GIL during encoding for better concurrency.
    fn encode_mono<'py>(
        &mut self,
        py: Python<'py>,
        pcm: &Bound<'py, PyBytes>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // Get read-only byte slice from PyBytes
        let pcm_bytes = pcm.as_bytes();

        // ✅ Use bytemuck for safe type conversion with alignment checking
        let pcm_slice: &[i16] = bytemuck::try_cast_slice(pcm_bytes).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "PCM data must be properly aligned for i16 (length must be even)",
            )
        })?;

        // Ensure buffer is large enough (reuse if possible)
        let required_size = pcm_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure (avoids raw pointer issues)
        let pcm_vec = pcm_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        let bytes_written = py.allow_threads(move || {
            // SAFETY: We hold a mutable reference to self, so no other thread can access it
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder.encode_mono(&pcm_vec, mp3_buffer).map_err(to_py_err)
        })?;

        // Return only the written portion as Python bytes
        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Encode mono PCM data from NumPy array (recommended, zero-copy)
    ///
    /// Args:
    ///     pcm: Mono samples as NumPy array with dtype=np.int16
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: This method is the recommended way to encode audio data.
    /// It provides zero-copy access to NumPy arrays and is completely safe (no unsafe code).
    /// Releases the GIL during encoding for better concurrency.
    ///
    /// Example:
    ///     ```python
    ///     import numpy as np
    ///     pcm = np.array([0, 100, -100, ...], dtype=np.int16)
    ///     mp3_data = encoder.encode_mono_numpy(pcm)
    ///     ```
    fn encode_mono_numpy<'py>(
        &mut self,
        py: Python<'py>,
        pcm: PyReadonlyArray1<'py, i16>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // ✅ Zero-copy access to NumPy array (completely safe, no unsafe needed)
        let pcm_slice = pcm.as_slice()?;

        // Ensure buffer is large enough (reuse if possible)
        let required_size = pcm_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure (avoids raw pointers)
        let pcm_vec = pcm_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        // Release GIL during encoding
        let bytes_written = py.allow_threads(move || {
            // SAFETY: Same pattern as the original implementation
            // We hold a mutable reference to self, so no other thread can access it
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder.encode_mono(&pcm_vec, mp3_buffer).map_err(to_py_err)
        })?;

        // Return only the written portion as Python bytes
        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Encode stereo PCM data from NumPy arrays (recommended, zero-copy)
    ///
    /// Args:
    ///     pcm_left: Left channel samples as NumPy array with dtype=np.int16
    ///     pcm_right: Right channel samples as NumPy array with dtype=np.int16
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: This method is the recommended way to encode stereo audio.
    /// It provides zero-copy access to NumPy arrays and is completely safe.
    fn encode_numpy<'py>(
        &mut self,
        py: Python<'py>,
        pcm_left: PyReadonlyArray1<'py, i16>,
        pcm_right: PyReadonlyArray1<'py, i16>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // ✅ Zero-copy access to NumPy arrays (no unsafe needed)
        let pcm_left_slice = pcm_left.as_slice()?;
        let pcm_right_slice = pcm_right.as_slice()?;

        // Ensure buffer is large enough
        let required_size = pcm_left_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure
        let left_vec = pcm_left_slice.to_vec();
        let right_vec = pcm_right_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        // Release GIL during encoding
        let bytes_written = py.allow_threads(move || {
            // SAFETY: Same pattern as the original implementation
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder
                .encode(&left_vec, &right_vec, mp3_buffer)
                .map_err(to_py_err)
        })?;

        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Encode interleaved stereo PCM data from NumPy array (recommended, zero-copy)
    ///
    /// Args:
    ///     pcm_interleaved: Interleaved samples as NumPy array (L, R, L, R, ...) with dtype=np.int16
    ///
    /// Returns:
    ///     Encoded MP3 data as bytes
    ///
    /// Note: This method is the recommended way for interleaved stereo audio.
    fn encode_interleaved_numpy<'py>(
        &mut self,
        py: Python<'py>,
        pcm_interleaved: PyReadonlyArray1<'py, i16>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // ✅ Zero-copy access to NumPy array (no unsafe needed)
        let pcm_slice = pcm_interleaved.as_slice()?;

        // Ensure buffer is large enough
        let required_size = pcm_slice.len() * 5 / 4 + 7200;
        if self.mp3_buffer.len() < required_size {
            self.mp3_buffer.resize(required_size, 0);
        }

        // Clone data to pass ownership to the closure
        let pcm_vec = pcm_slice.to_vec();

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;
        let buffer_ptr = self.mp3_buffer.as_mut_ptr() as usize;
        let buffer_len = self.mp3_buffer.len();

        // Release GIL during encoding
        let bytes_written = py.allow_threads(move || {
            // SAFETY: Same pattern as the original implementation
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            let mp3_buffer =
                unsafe { std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len) };
            encoder
                .encode_interleaved(&pcm_vec, mp3_buffer)
                .map_err(to_py_err)
        })?;

        Ok(PyBytes::new_bound(py, &self.mp3_buffer[..bytes_written]))
    }

    /// Flush remaining data from encoder
    ///
    /// Should be called after all PCM data has been encoded to ensure
    /// all data is written to the output.
    ///
    /// Returns:
    ///     Final MP3 data as bytes
    ///
    /// Note: Releases the GIL during flushing for better concurrency.
    fn flush<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut mp3_buffer = vec![0u8; 7200];

        let encoder_ptr = &mut self.inner as *mut lame_sys::LameEncoder as usize;

        let bytes_written = py.allow_threads(|| {
            let encoder = unsafe { &mut *(encoder_ptr as *mut lame_sys::LameEncoder) };
            encoder.flush(&mut mp3_buffer).map_err(to_py_err)
        })?;

        mp3_buffer.truncate(bytes_written);
        Ok(PyBytes::new_bound(py, &mp3_buffer))
    }

    /// Create an ID3 tag builder for this encoder
    ///
    /// Returns an Id3Tag builder for setting metadata.
    fn id3_tag(&mut self) -> Id3Tag {
        Id3Tag::new(self)
    }

    fn __repr__(&self) -> String {
        "LameEncoder()".to_string()
    }
}
