use lame_sys::{LameEncoder, Id3Tag, Quality, VbrMode};

#[test]
fn test_basic_encoding() {
    // 创建编码器
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(128)
        .quality(Quality::Standard)
        .build()
        .expect("Failed to create encoder");

    // 创建测试数据（静音）
    let num_samples = 1152; // LAME 推荐的帧大小
    let pcm_left = vec![0i16; num_samples];
    let pcm_right = vec![0i16; num_samples];

    // MP3 缓冲区（推荐大小：1.25 * num_samples + 7200）
    let mp3_buffer_size = (1.25 * num_samples as f64) as usize + 7200;
    let mut mp3_buffer = vec![0u8; mp3_buffer_size];

    // 编码
    let bytes_written = encoder
        .encode(&pcm_left, &pcm_right, &mut mp3_buffer)
        .expect("Encoding failed");

    println!("Encoded {} bytes", bytes_written);

    // 刷新缓冲区
    let final_bytes = encoder
        .flush(&mut mp3_buffer)
        .expect("Flush failed");

    println!("Final flush: {} bytes", final_bytes);
}

#[test]
fn test_interleaved_encoding() {
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(192)
        .build()
        .expect("Failed to create encoder");

    // 创建交错立体声数据
    let num_samples = 1152;
    let mut pcm_interleaved = vec![0i16; num_samples * 2];

    // 生成简单的测试波形
    for i in 0..num_samples {
        let sample = (i as f32 * 0.1).sin();
        let value = (sample * 16384.0) as i16;
        pcm_interleaved[i * 2] = value;     // 左声道
        pcm_interleaved[i * 2 + 1] = value; // 右声道
    }

    let mp3_buffer_size = (1.25 * num_samples as f64) as usize + 7200;
    let mut mp3_buffer = vec![0u8; mp3_buffer_size];

    let bytes_written = encoder
        .encode_interleaved(&pcm_interleaved, &mut mp3_buffer)
        .expect("Interleaved encoding failed");

    assert!(bytes_written > 0);
    println!("Encoded {} bytes (interleaved)", bytes_written);
}

#[test]
fn test_vbr_encoding() {
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .vbr_mode(VbrMode::Vbr)
        .vbr_quality(2) // 高质量
        .build()
        .expect("Failed to create VBR encoder");

    let num_samples = 1152;
    let pcm_left = vec![0i16; num_samples];
    let pcm_right = vec![0i16; num_samples];
    let mut mp3_buffer = vec![0u8; 8192];

    let bytes_written = encoder
        .encode(&pcm_left, &pcm_right, &mut mp3_buffer)
        .expect("VBR encoding failed");

    println!("VBR encoded {} bytes", bytes_written);
}

#[test]
fn test_id3_tags() {
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(128)
        .build()
        .expect("Failed to create encoder");

    // 设置 ID3 标签
    Id3Tag::new(&mut encoder)
        .title("Test Song")
        .expect("Failed to set title")
        .artist("Test Artist")
        .expect("Failed to set artist")
        .album("Test Album")
        .expect("Failed to set album")
        .year("2024")
        .expect("Failed to set year")
        .comment("Integration test")
        .expect("Failed to set comment")
        .track(1)
        .genre("Rock")
        .expect("Failed to set genre")
        .apply()
        .expect("Failed to apply tags");

    // 编码一些数据
    let num_samples = 1152;
    let pcm_left = vec![0i16; num_samples];
    let pcm_right = vec![0i16; num_samples];
    let mut mp3_buffer = vec![0u8; 8192];

    let bytes_written = encoder
        .encode(&pcm_left, &pcm_right, &mut mp3_buffer)
        .expect("Encoding with ID3 tags failed");

    assert!(bytes_written > 0);
    println!("Encoded with ID3 tags: {} bytes", bytes_written);
}

#[test]
fn test_different_sample_rates() {
    let sample_rates = [8000, 16000, 22050, 32000, 44100, 48000];

    for &sample_rate in &sample_rates {
        let mut encoder = LameEncoder::builder()
            .sample_rate(sample_rate)
            .channels(1) // 单声道
            .bitrate(64)
            .build()
            .expect(&format!("Failed to create encoder for {} Hz", sample_rate));

        let num_samples = 1152;
        let pcm = vec![0i16; num_samples];
        let mut mp3_buffer = vec![0u8; 8192];

        let bytes_written = encoder
            .encode(&pcm, &pcm, &mut mp3_buffer)
            .expect(&format!("Encoding failed for {} Hz", sample_rate));

        println!("Sample rate {} Hz: {} bytes", sample_rate, bytes_written);
    }
}

#[test]
fn test_different_qualities() {
    let qualities = [
        Quality::Best,
        Quality::High,
        Quality::Standard,
        Quality::Fast,
        Quality::Fastest,
    ];

    for quality in &qualities {
        let mut encoder = LameEncoder::builder()
            .sample_rate(44100)
            .channels(2)
            .bitrate(128)
            .quality(*quality)
            .build()
            .expect(&format!("Failed to create encoder for quality {:?}", quality));

        let num_samples = 1152;
        let pcm_left = vec![0i16; num_samples];
        let pcm_right = vec![0i16; num_samples];
        let mut mp3_buffer = vec![0u8; 8192];

        let bytes_written = encoder
            .encode(&pcm_left, &pcm_right, &mut mp3_buffer)
            .expect(&format!("Encoding failed for quality {:?}", quality));

        println!("Quality {:?}: {} bytes", quality, bytes_written);
    }
}

#[test]
fn test_multiple_frames() {
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .bitrate(128)
        .build()
        .expect("Failed to create encoder");

    let num_samples = 1152;
    let mut total_bytes = 0;

    // 编码多帧
    for frame in 0..10 {
        let pcm_left = vec![((frame * 100) % 1000) as i16; num_samples];
        let pcm_right = vec![((frame * 100) % 1000) as i16; num_samples];
        let mut mp3_buffer = vec![0u8; 8192];

        let bytes_written = encoder
            .encode(&pcm_left, &pcm_right, &mut mp3_buffer)
            .expect(&format!("Encoding frame {} failed", frame));

        total_bytes += bytes_written;
    }

    // 刷新
    let mut mp3_buffer = vec![0u8; 8192];
    let final_bytes = encoder
        .flush(&mut mp3_buffer)
        .expect("Flush failed");

    total_bytes += final_bytes;

    println!("Total bytes encoded: {}", total_bytes);
    assert!(total_bytes > 0);
}

#[test]
fn test_error_handling() {
    // 测试无效参数
    let result = LameEncoder::builder()
        .sample_rate(0) // 无效采样率
        .channels(2)
        .build();

    // 应该失败（虽然 LAME 可能有默认处理）
    // 这个测试主要是确保 API 不会崩溃
    println!("Invalid sample rate result: {:?}", result);

    // 测试不匹配的声道长度
    if let Ok(mut encoder) = LameEncoder::builder()
        .sample_rate(44100)
        .channels(2)
        .build()
    {
        let pcm_left = vec![0i16; 1152];
        let pcm_right = vec![0i16; 100]; // 不同长度
        let mut mp3_buffer = vec![0u8; 8192];

        let result = encoder.encode(&pcm_left, &pcm_right, &mut mp3_buffer);
        assert!(result.is_err());
        println!("Mismatched channel length error: {:?}", result);
    }
}

#[test]
fn test_mono_encoding() {
    // 创建单声道编码器
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(1) // 单声道
        .bitrate(128)
        .quality(Quality::Standard)
        .build()
        .expect("Failed to create mono encoder");

    // 创建测试数据
    let num_samples = 1152;
    let pcm = vec![0i16; num_samples];

    let mp3_buffer_size = (1.25 * num_samples as f64) as usize + 7200;
    let mut mp3_buffer = vec![0u8; mp3_buffer_size];

    // 使用 encode_mono 方法
    let bytes_written = encoder
        .encode_mono(&pcm, &mut mp3_buffer)
        .expect("Mono encoding failed");

    assert!(bytes_written > 0, "No bytes written for mono encoding");
    println!("Mono encoded {} bytes", bytes_written);

    // 刷新缓冲区
    let final_bytes = encoder
        .flush(&mut mp3_buffer)
        .expect("Mono flush failed");

    println!("Mono final flush: {} bytes", final_bytes);
}

#[test]
fn test_mono_encoding_with_sine_wave() {
    // 创建单声道编码器
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(192)
        .build()
        .expect("Failed to create mono encoder");

    // 生成 440 Hz 正弦波
    let num_samples = 1152;
    let sample_rate = 44100.0;
    let frequency = 440.0;

    let mut pcm = vec![0i16; num_samples];
    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
        pcm[i] = (sample * 16384.0) as i16;
    }

    let mp3_buffer_size = (1.25 * num_samples as f64) as usize + 7200;
    let mut mp3_buffer = vec![0u8; mp3_buffer_size];

    let bytes_written = encoder
        .encode_mono(&pcm, &mut mp3_buffer)
        .expect("Mono sine wave encoding failed");

    assert!(bytes_written > 0);
    println!("Mono sine wave encoded {} bytes", bytes_written);
}

#[test]
fn test_mono_multiple_frames() {
    // 创建单声道编码器
    let mut encoder = LameEncoder::builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .build()
        .expect("Failed to create mono encoder");

    let num_samples = 1152;
    let mut total_bytes = 0;

    // 编码多帧
    for frame in 0..10 {
        let pcm = vec![((frame * 1000) % 32767) as i16; num_samples];
        let mut mp3_buffer = vec![0u8; 8192];

        let bytes_written = encoder
            .encode_mono(&pcm, &mut mp3_buffer)
            .expect(&format!("Mono encoding frame {} failed", frame));

        total_bytes += bytes_written;
        println!("Mono frame {}: {} bytes", frame, bytes_written);
    }

    // 刷新
    let mut mp3_buffer = vec![0u8; 8192];
    let final_bytes = encoder
        .flush(&mut mp3_buffer)
        .expect("Mono flush failed");

    total_bytes += final_bytes;

    println!("Mono total bytes encoded: {}", total_bytes);
    assert!(total_bytes > 0);
}

#[test]
fn test_mono_different_bitrates() {
    let bitrates = [64, 96, 128, 192, 256];

    for &bitrate in &bitrates {
        let mut encoder = LameEncoder::builder()
            .sample_rate(44100)
            .channels(1)
            .bitrate(bitrate)
            .build()
            .expect(&format!("Failed to create mono encoder for {} kbps", bitrate));

        let num_samples = 1152;
        let pcm = vec![0i16; num_samples];
        let mut mp3_buffer = vec![0u8; 8192];

        let bytes_written = encoder
            .encode_mono(&pcm, &mut mp3_buffer)
            .expect(&format!("Mono encoding failed for {} kbps", bitrate));

        println!("Mono bitrate {} kbps: {} bytes", bitrate, bytes_written);
        assert!(bytes_written > 0);
    }
}
