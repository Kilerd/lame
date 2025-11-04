use pyo3::prelude::*;

/// Encoding quality level
///
/// Higher quality means slower encoding but better audio quality.
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    /// Best quality (slowest)
    Best = 0,
    /// High quality
    High = 2,
    /// Good quality
    Good = 4,
    /// Standard quality (recommended default)
    Standard = 5,
    /// Fast encoding
    Fast = 7,
    /// Fastest encoding (lowest quality)
    Fastest = 9,
}

impl From<Quality> for lame_sys::Quality {
    fn from(q: Quality) -> Self {
        match q {
            Quality::Best => lame_sys::Quality::Best,
            Quality::High => lame_sys::Quality::High,
            Quality::Good => lame_sys::Quality::Good,
            Quality::Standard => lame_sys::Quality::Standard,
            Quality::Fast => lame_sys::Quality::Fast,
            Quality::Fastest => lame_sys::Quality::Fastest,
        }
    }
}

#[pymethods]
impl Quality {
    fn __repr__(&self) -> String {
        format!("Quality.{:?}", self)
    }
}

/// VBR (Variable Bit Rate) mode
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VbrMode {
    /// Constant Bit Rate (no VBR)
    Off = 0,
    /// Variable Bit Rate
    Vbr = 4,
    /// Average Bit Rate
    Abr = 3,
}

impl From<VbrMode> for lame_sys::VbrMode {
    fn from(v: VbrMode) -> Self {
        match v {
            VbrMode::Off => lame_sys::VbrMode::Off,
            VbrMode::Vbr => lame_sys::VbrMode::Vbr,
            VbrMode::Abr => lame_sys::VbrMode::Abr,
        }
    }
}

#[pymethods]
impl VbrMode {
    fn __repr__(&self) -> String {
        format!("VbrMode.{:?}", self)
    }
}
