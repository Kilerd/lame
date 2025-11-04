use crate::error::{LameError, Result};
use crate::ffi;
use std::ptr::{self, NonNull};

/// LAME 编码质量级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    /// 最高质量（最慢）
    Best = 0,
    /// 接近最高质量
    High = 2,
    /// 良好质量
    Good = 4,
    /// 标准质量（推荐）
    Standard = 5,
    /// 快速编码
    Fast = 7,
    /// 最快速度（质量最低）
    Fastest = 9,
}

/// VBR（可变比特率）模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VbrMode {
    /// 关闭 VBR（使用 CBR）
    Off = 0,
    /// VBR 模式
    Vbr = 4,
    /// ABR（平均比特率）模式
    Abr = 3,
}

/// LAME MP3 编码器
///
/// 这是对 LAME C API 的安全封装，使用 RAII 模式自动管理资源。
///
/// # 示例
///
/// ```no_run
/// use lame_sys::{LameEncoder, Quality};
///
/// let mut encoder = LameEncoder::builder()?
///     .sample_rate(44100)?
///     .channels(2)?
///     .quality(Quality::Standard)?
///     .bitrate(192)?
///     .build()?;
///
/// // 编码 PCM 数据
/// let pcm_left = vec![0i16; 1152];
/// let pcm_right = vec![0i16; 1152];
/// let mut mp3_buffer = vec![0u8; 8192];
///
/// let bytes_written = encoder.encode(&pcm_left, &pcm_right, &mut mp3_buffer)?;
/// # Ok::<(), lame_sys::LameError>(())
/// ```
pub struct LameEncoder {
    /// 指向 LAME global flags 的非空指针（优化友好）
    gfp: NonNull<ffi::lame_global_flags>,
}

impl std::fmt::Debug for LameEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LameEncoder")
            .field("gfp", &self.gfp.as_ptr())
            .finish()
    }
}

impl LameEncoder {
    /// 创建编码器构建器
    ///
    /// 如果无法初始化 LAME，返回错误。
    pub fn builder() -> Result<EncoderBuilder> {
        EncoderBuilder::new()
    }

    /// 编码立体声 PCM 数据到 MP3
    ///
    /// # 参数
    ///
    /// * `pcm_left` - 左声道 PCM 样本（16-bit）
    /// * `pcm_right` - 右声道 PCM 样本（16-bit）
    /// * `mp3_buffer` - 输出 MP3 数据的缓冲区
    ///
    /// # 返回
    ///
    /// 返回写入 `mp3_buffer` 的字节数
    #[inline(always)]
    pub fn encode(
        &mut self,
        pcm_left: &[i16],
        pcm_right: &[i16],
        mp3_buffer: &mut [u8],
    ) -> Result<usize> {
        if pcm_left.len() != pcm_right.len() {
            return Err(LameError::InvalidInput(
                "Left and right channel lengths must match".to_string(),
            ));
        }

        let num_samples = pcm_left.len();

        unsafe {
            let result = ffi::lame_encode_buffer(
                self.gfp.as_ptr(),
                pcm_left.as_ptr(),
                pcm_right.as_ptr(),
                num_samples as i32,
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as i32,
            );

            if result < 0 {
                Err(LameError::EncodingFailed(result))
            } else {
                Ok(result as usize)
            }
        }
    }

    /// 编码交错立体声 PCM 数据到 MP3
    ///
    /// # 参数
    ///
    /// * `pcm_interleaved` - 交错的立体声 PCM 样本（L, R, L, R, ...）
    /// * `mp3_buffer` - 输出 MP3 数据的缓冲区
    ///
    /// # 返回
    ///
    /// 返回写入 `mp3_buffer` 的字节数
    #[inline(always)]
    pub fn encode_interleaved(
        &mut self,
        pcm_interleaved: &[i16],
        mp3_buffer: &mut [u8],
    ) -> Result<usize> {
        let num_samples = pcm_interleaved.len() / 2;

        unsafe {
            let result = ffi::lame_encode_buffer_interleaved(
                self.gfp.as_ptr(),
                pcm_interleaved.as_ptr() as *mut i16,
                num_samples as i32,
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as i32,
            );

            if result < 0 {
                Err(LameError::EncodingFailed(result))
            } else {
                Ok(result as usize)
            }
        }
    }

    /// 编码单声道 PCM 数据到 MP3
    ///
    /// # 参数
    ///
    /// * `pcm` - 单声道 PCM 样本（16-bit）
    /// * `mp3_buffer` - 输出 MP3 数据的缓冲区
    ///
    /// # 返回
    ///
    /// 返回写入 `mp3_buffer` 的字节数
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use lame_sys::LameEncoder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut encoder = LameEncoder::builder()?
    ///     .sample_rate(44100)?
    ///     .channels(1)?  // 单声道
    ///     .bitrate(128)?
    ///     .build()?;
    ///
    /// let pcm = vec![0i16; 1152];
    /// let mut mp3_buffer = vec![0u8; 8192];
    ///
    /// let bytes_written = encoder.encode_mono(&pcm, &mut mp3_buffer)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn encode_mono(&mut self, pcm: &[i16], mp3_buffer: &mut [u8]) -> Result<usize> {
        unsafe {
            let result = ffi::lame_encode_buffer(
                self.gfp.as_ptr(),
                pcm.as_ptr(),
                ptr::null(), // 单声道传递 null 指针
                pcm.len() as i32,
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as i32,
            );

            if result < 0 {
                Err(LameError::EncodingFailed(result))
            } else {
                Ok(result as usize)
            }
        }
    }

    /// 刷新编码器缓冲区
    ///
    /// 在编码完所有数据后调用此方法，获取最后的 MP3 帧。
    ///
    /// # 参数
    ///
    /// * `mp3_buffer` - 输出缓冲区
    ///
    /// # 返回
    ///
    /// 返回写入的字节数
    #[inline(always)]
    pub fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize> {
        unsafe {
            let result = ffi::lame_encode_flush(
                self.gfp.as_ptr(),
                mp3_buffer.as_mut_ptr(),
                mp3_buffer.len() as i32,
            );

            if result < 0 {
                Err(LameError::EncodingFailed(result))
            } else {
                Ok(result as usize)
            }
        }
    }

    /// 获取原始的 LAME global flags 指针（用于高级操作）
    ///
    /// # 安全性
    ///
    /// 调用者必须确保不会释放返回的指针，也不能在编码器销毁后使用。
    pub unsafe fn as_ptr(&self) -> *mut ffi::lame_global_flags {
        self.gfp.as_ptr()
    }
}

impl Drop for LameEncoder {
    fn drop(&mut self) {
        unsafe {
            ffi::lame_close(self.gfp.as_ptr());
        }
    }
}

// LameEncoder 不是 Send/Sync，因为 LAME C 库不是线程安全的
// 如果需要多线程编码，应该为每个线程创建独立的编码器

/// 编码器构建器
///
/// 使用 Builder 模式配置并创建 LAME 编码器。
///
/// 注意：Builder 在创建时就初始化 LAME C 结构体，每个配置方法立即调用底层 FFI。
/// 这种设计消除了额外的内存分配和分支判断，提供更好的性能。
pub struct EncoderBuilder {
    /// 指向 LAME global flags 的非空指针
    inner: NonNull<ffi::lame_global_flags>,
}

impl EncoderBuilder {
    /// 创建新的构建器
    ///
    /// 立即初始化 LAME C 结构体。如果初始化失败，返回错误。
    pub fn new() -> Result<Self> {
        unsafe {
            let gfp = ffi::lame_init();
            if gfp.is_null() {
                return Err(LameError::InitializationFailed);
            }
            Ok(Self {
                inner: NonNull::new_unchecked(gfp),
            })
        }
    }

    /// 获取内部指针（私有辅助方法）
    #[inline(always)]
    fn ptr(&self) -> *mut ffi::lame_global_flags {
        self.inner.as_ptr()
    }

    /// 设置采样率（Hz）
    ///
    /// 常见值：8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000
    #[inline(always)]
    pub fn sample_rate(self, rate: i32) -> Result<Self> {
        unsafe {
            if ffi::lame_set_in_samplerate(self.ptr(), rate) < 0 {
                return Err(LameError::InvalidParameter("sample_rate".to_string()));
            }
            ffi::lame_set_out_samplerate(self.ptr(), rate);
        }
        Ok(self)
    }

    /// 设置声道数（1 = 单声道, 2 = 立体声）
    #[inline(always)]
    pub fn channels(self, channels: i32) -> Result<Self> {
        unsafe {
            if ffi::lame_set_num_channels(self.ptr(), channels) < 0 {
                return Err(LameError::InvalidParameter("channels".to_string()));
            }
        }
        Ok(self)
    }

    /// 设置比特率（kbps）
    ///
    /// 常见值：32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320
    #[inline(always)]
    pub fn bitrate(self, bitrate: i32) -> Result<Self> {
        unsafe {
            if ffi::lame_set_brate(self.ptr(), bitrate) < 0 {
                return Err(LameError::InvalidParameter("bitrate".to_string()));
            }
        }
        Ok(self)
    }

    /// 设置编码质量
    #[inline(always)]
    pub fn quality(self, quality: Quality) -> Result<Self> {
        unsafe {
            if ffi::lame_set_quality(self.ptr(), quality as i32) < 0 {
                return Err(LameError::InvalidParameter("quality".to_string()));
            }
        }
        Ok(self)
    }

    /// 设置 VBR 模式
    #[inline(always)]
    pub fn vbr_mode(self, mode: VbrMode) -> Result<Self> {
        unsafe {
            if ffi::lame_set_VBR(self.ptr(), mode as u32) < 0 {
                return Err(LameError::InvalidParameter("vbr_mode".to_string()));
            }
        }
        Ok(self)
    }

    /// 设置 VBR 质量（0-9，0 = 最高质量）
    #[inline(always)]
    pub fn vbr_quality(self, quality: i32) -> Result<Self> {
        unsafe {
            if ffi::lame_set_VBR_q(self.ptr(), quality) < 0 {
                return Err(LameError::InvalidParameter("vbr_quality".to_string()));
            }
        }
        Ok(self)
    }

    /// 构建编码器
    ///
    /// 完成配置并创建可用的编码器。此方法会调用 `lame_init_params()` 来最终确定所有设置。
    #[inline(always)]
    pub fn build(self) -> Result<LameEncoder> {
        unsafe {
            // 初始化参数（所有配置都已在 setter 中设置完成）
            if ffi::lame_init_params(self.ptr()) < 0 {
                return Err(LameError::InitializationFailed);
            }

            // 转移所有权给 LameEncoder，防止 Drop 释放
            let inner = self.inner;
            std::mem::forget(self);

            Ok(LameEncoder { gfp: inner })
        }
    }
}

impl Drop for EncoderBuilder {
    fn drop(&mut self) {
        // 清理 LAME C 结构体（如果 build() 未被调用）
        unsafe {
            ffi::lame_close(self.ptr());
        }
    }
}
