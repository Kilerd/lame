//! LAME MP3 编码器的 Rust 绑定
//!
//! 这个 crate 提供了对 LAME MP3 编码器的安全 Rust 封装。
//! LAME 是一个高质量的 MP3 编码器，支持 CBR、VBR 和 ABR 编码模式。
//!
//! # 特性
//!
//! - 安全的 Rust API 封装
//! - 支持立体声和单声道编码
//! - 支持 CBR（恒定比特率）、VBR（可变比特率）和 ABR（平均比特率）模式
//! - ID3v1 和 ID3v2 标签支持
//! - 静态链接 LAME 库，无运行时依赖
//! - RAII 模式自动资源管理
//!
//! # 快速开始
//!
//! ```no_run
//! use lame_sys::{LameEncoder, Quality};
//!
//! // 创建编码器
//! let mut encoder = LameEncoder::builder()
//!     .sample_rate(44100)      // 44.1 kHz
//!     .channels(2)             // 立体声
//!     .quality(Quality::Standard)  // 标准质量
//!     .bitrate(192)            // 192 kbps
//!     .build()
//!     .unwrap();
//!
//! // 准备 PCM 数据
//! let pcm_left = vec![0i16; 1152];   // 左声道
//! let pcm_right = vec![0i16; 1152];  // 右声道
//! let mut mp3_buffer = vec![0u8; 8192];
//!
//! // 编码
//! let bytes_written = encoder.encode(&pcm_left, &pcm_right, &mut mp3_buffer)
//!     .unwrap();
//!
//! // 刷新缓冲区
//! let final_bytes = encoder.flush(&mut mp3_buffer)
//!     .unwrap();
//! ```
//!
//! # ID3 标签
//!
//! ```no_run
//! use lame_sys::{LameEncoder, Id3Tag};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut encoder = LameEncoder::builder()
//!     .sample_rate(44100)
//!     .channels(2)
//!     .build()?;
//!
//! // 设置 ID3 标签
//! Id3Tag::new(&mut encoder)
//!     .title("My Song")?
//!     .artist("My Band")?
//!     .album("My Album")?
//!     .year("2024")?
//!     .comment("Encoded with LAME")?
//!     .apply()?;
//! # Ok(())
//! # }
//! ```
//!
//! # 许可证
//!
//! 本 crate 使用 LGPL-2.0 许可证，与 LAME 库保持一致。

#![warn(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// 包含 bindgen 生成的 FFI 绑定
#[allow(missing_docs)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// 内部模块
pub mod encoder;
pub mod error;
pub mod id3;

// 重新导出公共 API
pub use encoder::{EncoderBuilder, LameEncoder, Quality, VbrMode};
pub use error::{LameError, Result};
pub use id3::{genres, Id3Tag};

/// 获取 LAME 版本字符串
///
/// # 返回
///
/// 返回 LAME 库的版本信息，例如 "3.100"
pub fn get_lame_version() -> String {
    unsafe {
        let version_ptr = ffi::get_lame_version();
        if version_ptr.is_null() {
            return String::from("unknown");
        }

        let c_str = std::ffi::CStr::from_ptr(version_ptr);
        c_str.to_string_lossy().into_owned()
    }
}

/// 获取 LAME 主页 URL
///
/// # 返回
///
/// 返回 LAME 项目的官方网址
pub fn get_lame_url() -> String {
    unsafe {
        let url_ptr = ffi::get_lame_url();
        if url_ptr.is_null() {
            return String::from("https://lame.sourceforge.io/");
        }

        let c_str = std::ffi::CStr::from_ptr(url_ptr);
        c_str.to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lame_version() {
        let version = get_lame_version();
        assert!(!version.is_empty());
        println!("LAME version: {}", version);
    }

    #[test]
    fn test_lame_url() {
        let url = get_lame_url();
        assert!(!url.is_empty());
        println!("LAME URL: {}", url);
    }

    #[test]
    fn test_encoder_creation() {
        let result = LameEncoder::builder()
            .sample_rate(44100)
            .channels(2)
            .bitrate(128)
            .quality(Quality::Standard)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_basic() {
        let mut encoder = LameEncoder::builder()
            .sample_rate(44100)
            .channels(2)
            .bitrate(128)
            .build()
            .unwrap();

        // 创建静音样本
        let samples = vec![0i16; 1152];
        let mut mp3_buffer = vec![0u8; 8192];

        let result = encoder.encode(&samples, &samples, &mut mp3_buffer);
        assert!(result.is_ok());
    }
}
