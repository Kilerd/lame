use crate::error::{LameError, Result};
use crate::ffi;
use std::ptr;

/// LAME 编码质量级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    /// 最高质量（最慢）
    Best = 0,
    /// 接近最高质量
    High = 2,
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
/// let mut encoder = LameEncoder::builder()
///     .sample_rate(44100)
///     .channels(2)
///     .quality(Quality::Standard)
///     .bitrate(192)
///     .build()
///     .unwrap();
///
/// // 编码 PCM 数据
/// let pcm_left = vec![0i16; 1152];
/// let pcm_right = vec![0i16; 1152];
/// let mut mp3_buffer = vec![0u8; 8192];
///
/// let bytes_written = encoder.encode(&pcm_left, &pcm_right, &mut mp3_buffer).unwrap();
/// ```
pub struct LameEncoder {
    /// 指向 LAME global flags 的指针
    gfp: *mut ffi::lame_global_flags,
}

impl std::fmt::Debug for LameEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LameEncoder")
            .field("gfp", &self.gfp)
            .finish()
    }
}

impl LameEncoder {
    /// 创建编码器构建器
    pub fn builder() -> EncoderBuilder {
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
                self.gfp,
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
    pub fn encode_interleaved(
        &mut self,
        pcm_interleaved: &[i16],
        mp3_buffer: &mut [u8],
    ) -> Result<usize> {
        let num_samples = pcm_interleaved.len() / 2;

        unsafe {
            let result = ffi::lame_encode_buffer_interleaved(
                self.gfp,
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
    pub fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize> {
        unsafe {
            let result =
                ffi::lame_encode_flush(self.gfp, mp3_buffer.as_mut_ptr(), mp3_buffer.len() as i32);

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
        self.gfp
    }
}

impl Drop for LameEncoder {
    fn drop(&mut self) {
        unsafe {
            ffi::lame_close(self.gfp);
        }
    }
}

// LameEncoder 不是 Send/Sync，因为 LAME C 库不是线程安全的
// 如果需要多线程编码，应该为每个线程创建独立的编码器

/// 编码器构建器
///
/// 使用 Builder 模式配置并创建 LAME 编码器。
pub struct EncoderBuilder {
    sample_rate: Option<i32>,
    channels: Option<i32>,
    bitrate: Option<i32>,
    quality: Option<Quality>,
    vbr_mode: Option<VbrMode>,
    vbr_quality: Option<i32>,
}

impl EncoderBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            sample_rate: None,
            channels: None,
            bitrate: None,
            quality: None,
            vbr_mode: None,
            vbr_quality: None,
        }
    }

    /// 设置采样率（Hz）
    ///
    /// 常见值：8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000
    pub fn sample_rate(mut self, rate: i32) -> Self {
        self.sample_rate = Some(rate);
        self
    }

    /// 设置声道数（1 = 单声道, 2 = 立体声）
    pub fn channels(mut self, channels: i32) -> Self {
        self.channels = Some(channels);
        self
    }

    /// 设置比特率（kbps）
    ///
    /// 常见值：32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320
    pub fn bitrate(mut self, bitrate: i32) -> Self {
        self.bitrate = Some(bitrate);
        self
    }

    /// 设置编码质量
    pub fn quality(mut self, quality: Quality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// 设置 VBR 模式
    pub fn vbr_mode(mut self, mode: VbrMode) -> Self {
        self.vbr_mode = Some(mode);
        self
    }

    /// 设置 VBR 质量（0-9，0 = 最高质量）
    pub fn vbr_quality(mut self, quality: i32) -> Self {
        self.vbr_quality = Some(quality);
        self
    }

    /// 构建编码器
    pub fn build(self) -> Result<LameEncoder> {
        unsafe {
            // 初始化 LAME
            let gfp = ffi::lame_init();
            if gfp.is_null() {
                return Err(LameError::InitializationFailed);
            }

            // 设置采样率
            if let Some(rate) = self.sample_rate {
                if ffi::lame_set_in_samplerate(gfp, rate) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("sample_rate".to_string()));
                }
                ffi::lame_set_out_samplerate(gfp, rate);
            }

            // 设置声道数
            if let Some(channels) = self.channels {
                if ffi::lame_set_num_channels(gfp, channels) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("channels".to_string()));
                }
            }

            // 设置比特率
            if let Some(bitrate) = self.bitrate {
                if ffi::lame_set_brate(gfp, bitrate) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("bitrate".to_string()));
                }
            }

            // 设置质量
            if let Some(quality) = self.quality {
                if ffi::lame_set_quality(gfp, quality as i32) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("quality".to_string()));
                }
            }

            // 设置 VBR 模式
            if let Some(vbr_mode) = self.vbr_mode {
                if ffi::lame_set_VBR(gfp, vbr_mode as u32) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("vbr_mode".to_string()));
                }
            }

            // 设置 VBR 质量
            if let Some(vbr_quality) = self.vbr_quality {
                if ffi::lame_set_VBR_q(gfp, vbr_quality) < 0 {
                    ffi::lame_close(gfp);
                    return Err(LameError::InvalidParameter("vbr_quality".to_string()));
                }
            }

            // 初始化参数
            if ffi::lame_init_params(gfp) < 0 {
                ffi::lame_close(gfp);
                return Err(LameError::InitializationFailed);
            }

            Ok(LameEncoder { gfp })
        }
    }
}

impl Default for EncoderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
