# Python LAME Encoder

High-performance Python bindings for the LAME MP3 encoder, built with PyO3 and Rust.

## Features

- 🚀 **High Performance**: Near-native speed with zero-copy data transfer
- 🔒 **Thread-Safe**: Automatic GIL release during encoding for true concurrency
- 🎵 **Full API**: Support for mono/stereo encoding, VBR/CBR modes, and quality presets
- 🏷️ **ID3 Tags**: Complete ID3v1 and ID3v2 tag support
- 📦 **Zero Dependencies**: Statically linked LAME library, no runtime dependencies
- 🐍 **Python 3.8+**: Compatible with Python 3.8 through 3.12+

## Installation

### From Source (requires Rust and maturin)

```bash
cd python-lame
pip install maturin
maturin develop --release
```

### Building Wheel

```bash
maturin build --release
pip install target/wheels/lame-*.whl
```

## Quick Start

```python
import lame

# Create encoder
encoder = (
    lame.LameEncoder.builder()
    .sample_rate(44100)
    .channels(1)
    .bitrate(128)
    .quality(lame.Quality.Standard)
    .build()
)

# Encode PCM data
pcm_samples = [0] * 1152  # Your PCM data here
mp3_data = encoder.encode_mono(pcm_samples)

# Flush remaining data
final_data = encoder.flush()

# Write to file
with open("output.mp3", "wb") as f:
    f.write(mp3_data)
    f.write(final_data)
```

## Examples

### Stereo Encoding with ID3 Tags

```python
import lame

# Create stereo encoder
encoder = (
    lame.LameEncoder.builder()
    .sample_rate(44100)
    .channels(2)
    .bitrate(192)
    .quality(lame.Quality.High)
    .build()
)

# Set ID3 tags
encoder.id3_tag() \
    .title("My Song") \
    .artist("My Artist") \
    .album("My Album") \
    .year("2024") \
    .genre("Rock") \
    .track(1) \
    .apply()

# Encode stereo data
left_channel = [0] * 1152
right_channel = [0] * 1152
mp3_data = encoder.encode(left_channel, right_channel)
```

### VBR Encoding

```python
import lame

encoder = (
    lame.LameEncoder.builder()
    .sample_rate(44100)
    .channels(2)
    .vbr_mode(lame.VbrMode.Vbr)
    .vbr_quality(4)  # 0=best, 9=worst
    .build()
)

# Encode interleaved stereo (L, R, L, R, ...)
interleaved_pcm = [0] * (1152 * 2)
mp3_data = encoder.encode_interleaved(interleaved_pcm)
```

### Batch Encoding

```python
import lame

encoder = (
    lame.LameEncoder.builder()
    .sample_rate(44100)
    .channels(1)
    .bitrate(128)
    .build()
)

# Process audio in chunks
mp3_chunks = []
chunk_size = 1152

for chunk in audio_chunks:
    mp3_data = encoder.encode_mono(chunk)
    mp3_chunks.append(mp3_data)

# Don't forget to flush!
mp3_chunks.append(encoder.flush())

# Combine all chunks
complete_mp3 = b''.join(mp3_chunks)
```

## API Reference

### LameEncoder

Main encoder class for MP3 encoding.

**Methods:**
- `builder()` → `EncoderBuilder`: Create a new encoder builder
- `encode(left, right)` → `bytes`: Encode stereo PCM data
- `encode_mono(pcm)` → `bytes`: Encode mono PCM data
- `encode_interleaved(pcm)` → `bytes`: Encode interleaved stereo PCM
- `flush()` → `bytes`: Flush remaining data from encoder
- `id3_tag()` → `Id3Tag`: Create ID3 tag builder

### EncoderBuilder

Builder for configuring encoder parameters.

**Methods:**
- `sample_rate(rate: int)` → `Self`: Set sample rate (e.g., 44100, 48000)
- `channels(n: int)` → `Self`: Set channels (1=mono, 2=stereo)
- `bitrate(kbps: int)` → `Self`: Set bitrate in kbps (e.g., 128, 192, 320)
- `quality(q: Quality)` → `Self`: Set encoding quality
- `vbr_mode(mode: VbrMode)` → `Self`: Set VBR mode
- `vbr_quality(q: int)` → `Self`: Set VBR quality (0-9)
- `build()` → `LameEncoder`: Build the encoder

### Quality

Encoding quality presets.

- `Quality.Best` (0): Highest quality, slowest
- `Quality.High` (2): High quality
- `Quality.Good` (4): Good quality
- `Quality.Standard` (5): Standard quality (recommended)
- `Quality.Fast` (7): Fast encoding
- `Quality.Fastest` (9): Fastest encoding, lowest quality

### VbrMode

Variable bitrate modes.

- `VbrMode.Off` (0): Constant bitrate (CBR)
- `VbrMode.Vbr` (4): Variable bitrate
- `VbrMode.Abr` (3): Average bitrate

### Id3Tag

ID3 tag builder for MP3 metadata.

**Methods:**
- `title(s: str)` → `Self`: Set title
- `artist(s: str)` → `Self`: Set artist
- `album(s: str)` → `Self`: Set album
- `year(s: str)` → `Self`: Set year
- `comment(s: str)` → `Self`: Set comment
- `track(n: int)` → `Self`: Set track number
- `genre(s: str)` → `Self`: Set genre
- `album_artist(s: str)` → `Self`: Set album artist
- `apply()`: Apply tags to encoder

### Utility Functions

- `get_version()` → `str`: Get LAME version string
- `get_url()` → `str`: Get LAME project URL

## Performance

The encoder automatically releases Python's GIL during encoding operations, allowing:
- True concurrent encoding in multiple threads
- Non-blocking operation for async applications
- Near-native performance (~300x realtime for standard quality)

## License

LGPL-2.0 (same as LAME)

## Contributing

Issues and pull requests welcome at the main repository.

## See Also

- [lame-sys](../lame-sys): The underlying Rust bindings
- [LAME Project](http://lame.sourceforge.net/): The LAME MP3 encoder
