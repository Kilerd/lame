# lame-sys

[![License: LGPL-2.0](https://img.shields.io/badge/License-LGPL%202.0-blue.svg)](https://www.gnu.org/licenses/lgpl-2.0)

Rust bindings for the LAME MP3 encoder with safe wrapper API.

LAME (LAME Ain't an MP3 Encoder) is a high-quality MP3 encoding library. This crate provides both low-level FFI bindings and high-level safe Rust API for LAME.

## Features

- ✅ Safe Rust API wrapper
- ✅ Stereo and mono encoding support
- ✅ CBR (Constant Bitrate), VBR (Variable Bitrate), and ABR (Average Bitrate) modes
- ✅ ID3v1 and ID3v2 tag support
- ✅ Static linking (no runtime dependencies)
- ✅ RAII-based automatic resource management
- ✅ Cross-platform (Linux, macOS, Windows)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lame-sys = "0.1.0"
```

## Quick Start

### Basic Encoding

```rust
use lame_sys::{LameEncoder, Quality};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create encoder
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)           // 44.1 kHz
        .channels(2)                  // Stereo
        .quality(Quality::Standard)   // Standard quality
        .bitrate(192)                 // 192 kbps
        .build()?;

    // Prepare PCM data
    let pcm_left = vec![0i16; 1152];    // Left channel
    let pcm_right = vec![0i16; 1152];   // Right channel
    let mut mp3_buffer = vec![0u8; 8192];

    // Encode
    let bytes_written = encoder.encode(&pcm_left, &pcm_right, &mut mp3_buffer)?;

    // Flush remaining data
    let final_bytes = encoder.flush(&mut mp3_buffer)?;

    Ok(())
}
```

### Adding ID3 Tags

```rust
use lame_sys::{LameEncoder, Id3Tag};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .build()?;

    // Set ID3 tags
    Id3Tag::new(&mut encoder)
        .title("My Song")?
        .artist("My Band")?
        .album("My Album")?
        .year("2024")?
        .comment("Encoded with LAME")?
        .apply()?;

    // ... encode audio ...

    Ok(())
}
```

### VBR Encoding

```rust
use lame_sys::{LameEncoder, VbrMode};

let mut encoder = LameEncoder::builder()
    .sample_rate(44100)
    .channels(2)
    .vbr_mode(VbrMode::Vbr)
    .vbr_quality(2)  // High quality (0-9, 0=best)
    .build()?;
```

## Examples

Run the included example:

```bash
cargo run --example simple_encode
```

This will generate a 1-second MP3 file with a 440 Hz sine wave.

## API Documentation

### Encoder Quality Levels

- `Quality::Best` - Highest quality (slowest)
- `Quality::High` - Near best quality
- `Quality::Standard` - Standard quality (recommended)
- `Quality::Fast` - Fast encoding
- `Quality::Fastest` - Fastest speed (lowest quality)

### VBR Modes

- `VbrMode::Off` - CBR (Constant Bitrate)
- `VbrMode::Vbr` - VBR (Variable Bitrate)
- `VbrMode::Abr` - ABR (Average Bitrate)

## Testing

Run the test suite:

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test encode_test

# All tests including doc tests
cargo test

# Build and run example
cargo run --example simple_encode
```

All 20 tests pass successfully:
- ✅ 8 unit tests
- ✅ 8 integration tests
- ✅ 4 documentation tests

## Building

This crate compiles LAME from source. Build dependencies:

- C compiler (gcc, clang, or MSVC)
- Rust 2021 edition or later

```bash
cargo build
cargo build --release
```

## LAME Version

This crate bundles **LAME 3.101 (SVN r6531)** and statically links it into your application.

Check version at runtime:

```rust
use lame_sys::get_lame_version;

println!("LAME version: {}", get_lame_version());
```

## License

This crate uses **LGPL-2.0** license, consistent with the LAME library.

## Architecture

```
lame-sys/
├── build.rs              # Build script (compiles C code + generates bindings)
├── src/
│   ├── lib.rs           # Public API exports
│   ├── ffi.rs           # Auto-generated FFI bindings (via bindgen)
│   ├── encoder.rs       # Safe LameEncoder wrapper
│   ├── id3.rs           # ID3 tag support
│   └── error.rs         # Error types
├── tests/
│   └── encode_test.rs   # Integration tests
├── examples/
│   └── simple_encode.rs # Usage example
└── lame/                # LAME source code (bundled)
```

## Implementation Details

### Static Linking

All LAME code is compiled into a static library (`libmp3lame.a`) and linked into your Rust binary. No runtime dependencies needed.

### Thread Safety

Each `LameEncoder` instance is NOT thread-safe. For multi-threaded encoding, create a separate encoder instance per thread.

### Memory Management

The `LameEncoder` uses RAII pattern with automatic cleanup via `Drop` trait:

```rust
{
    let encoder = LameEncoder::builder().build()?;
    // Use encoder...
} // Automatically calls lame_close() here
```

## Comparison with mp3lame-sys

This crate is inspired by [mp3lame-sys](https://github.com/DoumanAsh/mp3lame-sys) but with key differences:

| Feature | lame-sys | mp3lame-sys |
|---------|----------|-------------|
| Safe Rust API | ✅ Included | ❌ Unsafe only |
| ID3 Tags | ✅ Supported | ⚠️ Limited |
| Build System | `cc` crate | `configure` + `make` |
| LAME Version | 3.101 (SVN r6531) | 3.100 |
| Documentation | ✅ Comprehensive | ⚠️ Basic |

## Contributing

Issues and pull requests are welcome!

## References

- [LAME Official Site](https://lame.sourceforge.io/)
- [LAME Documentation](https://lame.sourceforge.io/using.php)
- [MP3 Format Overview](https://en.wikipedia.org/wiki/MP3)

## Acknowledgments

- LAME development team for the excellent MP3 encoder
- [mp3lame-sys](https://github.com/DoumanAsh/mp3lame-sys) for implementation reference
