use crate::error::{LameError, Result};
use crate::ffi;
use std::ffi::CString;

/// ID3 标签构建器
///
/// 用于设置 MP3 文件的 ID3 标签（元数据）。
///
/// # 示例
///
/// ```no_run
/// use lame_sys::{LameEncoder, Id3Tag};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut encoder = LameEncoder::builder()
///     .sample_rate(44100)
///     .channels(2)
///     .build()?;
///
/// // 设置 ID3 标签
/// Id3Tag::new(&mut encoder)
///     .title("My Song")?
///     .artist("My Band")?
///     .album("My Album")?
///     .year("2024")?
///     .comment("Encoded with LAME")?
///     .apply()?;
/// # Ok(())
/// # }
/// ```
pub struct Id3Tag<'a> {
    gfp: *mut ffi::lame_global_flags,
    _marker: std::marker::PhantomData<&'a mut crate::encoder::LameEncoder>,
}

impl<'a> Id3Tag<'a> {
    /// 创建新的 ID3 标签构建器
    ///
    /// # 参数
    ///
    /// * `encoder` - LAME 编码器的可变引用
    pub fn new(encoder: &'a mut crate::encoder::LameEncoder) -> Self {
        unsafe {
            let gfp = encoder.as_ptr();
            // 初始化 ID3 标签
            ffi::id3tag_init(gfp);

            Self {
                gfp,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// 设置标题
    pub fn title(self, title: &str) -> Result<Self> {
        let c_title = CString::new(title)?;
        unsafe {
            ffi::id3tag_set_title(self.gfp, c_title.as_ptr());
        }
        Ok(self)
    }

    /// 设置艺术家
    pub fn artist(self, artist: &str) -> Result<Self> {
        let c_artist = CString::new(artist)?;
        unsafe {
            ffi::id3tag_set_artist(self.gfp, c_artist.as_ptr());
        }
        Ok(self)
    }

    /// 设置专辑
    pub fn album(self, album: &str) -> Result<Self> {
        let c_album = CString::new(album)?;
        unsafe {
            ffi::id3tag_set_album(self.gfp, c_album.as_ptr());
        }
        Ok(self)
    }

    /// 设置年份
    pub fn year(self, year: &str) -> Result<Self> {
        let c_year = CString::new(year)?;
        unsafe {
            ffi::id3tag_set_year(self.gfp, c_year.as_ptr());
        }
        Ok(self)
    }

    /// 设置注释
    pub fn comment(self, comment: &str) -> Result<Self> {
        let c_comment = CString::new(comment)?;
        unsafe {
            ffi::id3tag_set_comment(self.gfp, c_comment.as_ptr());
        }
        Ok(self)
    }

    /// 设置曲目编号
    pub fn track(self, track: u32) -> Self {
        let track_str = format!("{}", track);
        if let Ok(c_track) = CString::new(track_str) {
            unsafe {
                ffi::id3tag_set_track(self.gfp, c_track.as_ptr());
            }
        }
        self
    }

    /// 设置流派（Genre）
    ///
    /// 可以是流派名称或 ID3v1 流派编号（0-255）
    pub fn genre(self, genre: &str) -> Result<Self> {
        let c_genre = CString::new(genre)?;
        unsafe {
            ffi::id3tag_set_genre(self.gfp, c_genre.as_ptr());
        }
        Ok(self)
    }

    /// 设置专辑艺术家
    pub fn album_artist(self, album_artist: &str) -> Result<Self> {
        let c_album_artist = CString::new(album_artist)?;
        unsafe {
            ffi::id3tag_set_albumart(self.gfp, c_album_artist.as_ptr(), 0);
        }
        Ok(self)
    }

    /// 完成 ID3 标签设置
    ///
    /// 应用所有设置的标签信息。
    pub fn apply(self) -> Result<()> {
        // ID3 标签会在编码时自动写入
        // 这里只是一个标记方法，表示标签设置完成
        Ok(())
    }
}

/// ID3v1 流派列表（部分常用流派）
#[allow(dead_code)]
pub mod genres {
    pub const BLUES: u8 = 0;
    pub const CLASSIC_ROCK: u8 = 1;
    pub const COUNTRY: u8 = 2;
    pub const DANCE: u8 = 3;
    pub const DISCO: u8 = 4;
    pub const FUNK: u8 = 5;
    pub const GRUNGE: u8 = 6;
    pub const HIP_HOP: u8 = 7;
    pub const JAZZ: u8 = 8;
    pub const METAL: u8 = 9;
    pub const NEW_AGE: u8 = 10;
    pub const OLDIES: u8 = 11;
    pub const OTHER: u8 = 12;
    pub const POP: u8 = 13;
    pub const RNB: u8 = 14;
    pub const RAP: u8 = 15;
    pub const REGGAE: u8 = 16;
    pub const ROCK: u8 = 17;
    pub const TECHNO: u8 = 18;
    pub const INDUSTRIAL: u8 = 19;
    pub const ALTERNATIVE: u8 = 20;
    pub const SKA: u8 = 21;
    pub const DEATH_METAL: u8 = 22;
    pub const PRANKS: u8 = 23;
    pub const SOUNDTRACK: u8 = 24;
    pub const EURO_TECHNO: u8 = 25;
    pub const AMBIENT: u8 = 26;
    pub const TRIP_HOP: u8 = 27;
    pub const VOCAL: u8 = 28;
    pub const JAZZ_FUNK: u8 = 29;
    pub const FUSION: u8 = 30;
    pub const TRANCE: u8 = 31;
    pub const CLASSICAL: u8 = 32;
    pub const INSTRUMENTAL: u8 = 33;
    pub const ACID: u8 = 34;
    pub const HOUSE: u8 = 35;
    pub const GAME: u8 = 36;
    pub const SOUND_CLIP: u8 = 37;
    pub const GOSPEL: u8 = 38;
    pub const NOISE: u8 = 39;
    pub const ALTERNATIVE_ROCK: u8 = 40;
    pub const BASS: u8 = 41;
    pub const SOUL: u8 = 42;
    pub const PUNK: u8 = 43;
    pub const SPACE: u8 = 44;
    pub const MEDITATIVE: u8 = 45;
    pub const INSTRUMENTAL_POP: u8 = 46;
    pub const INSTRUMENTAL_ROCK: u8 = 47;
    pub const ETHNIC: u8 = 48;
    pub const GOTHIC: u8 = 49;
    pub const DARKWAVE: u8 = 50;
}
