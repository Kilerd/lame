use crate::encoder::LameEncoder;
use crate::error::to_py_err;
use pyo3::prelude::*;
use std::marker::PhantomData;

/// ID3 tag builder for MP3 metadata
///
/// # Example
///
/// ```python
/// encoder = LameEncoder.builder()
/// encoder.sample_rate(44100)
/// encoder.channels(2)
/// encoder = encoder.build()
///
/// tag = encoder.id3_tag()
/// tag.title("My Song")
/// tag.artist("My Artist")
/// tag.album("My Album")
/// tag.year("2024")
/// tag.apply()
///
/// # Now encode...
/// ```
#[pyclass(unsendable)]
pub struct Id3Tag {
    inner: Option<lame_sys::Id3Tag<'static>>,
    _phantom: PhantomData<*mut LameEncoder>,
}

impl Id3Tag {
    pub(crate) fn new(encoder: &mut LameEncoder) -> Self {
        // SAFETY: We're using PhantomData to ensure the lifetime is managed correctly
        // The Python borrow checker will ensure encoder lives long enough
        let inner = unsafe {
            std::mem::transmute::<lame_sys::Id3Tag<'_>, lame_sys::Id3Tag<'static>>(
                lame_sys::Id3Tag::new(&mut encoder.inner),
            )
        };
        Self {
            inner: Some(inner),
            _phantom: PhantomData,
        }
    }
}

#[pymethods]
impl Id3Tag {
    /// Set the song title
    fn title(&mut self, title: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.title(title).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the artist name
    fn artist(&mut self, artist: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.artist(artist).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the album name
    fn album(&mut self, album: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.album(album).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the year
    fn year(&mut self, year: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.year(year).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set a comment
    fn comment(&mut self, comment: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.comment(comment).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the track number
    fn track(&mut self, track: u32) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.track(track);
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the genre
    fn genre(&mut self, genre: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.genre(genre).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Set the album artist
    fn album_artist(&mut self, album_artist: &str) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        let tag = tag.album_artist(album_artist).map_err(to_py_err)?;
        self.inner = Some(tag);
        Ok(())
    }

    /// Apply the ID3 tags to the encoder
    ///
    /// Must be called before encoding starts.
    fn apply(&mut self) -> PyResult<()> {
        let tag = self.inner.take().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Tag already consumed")
        })?;
        tag.apply().map_err(to_py_err)
    }

    fn __repr__(&self) -> String {
        "Id3Tag()".to_string()
    }
}
