use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem::MaybeUninit;

// 生成测试 PCM 数据（单声道 440 Hz 正弦波）
fn generate_pcm_data(num_samples: usize) -> Vec<i16> {
    let sample_rate = 44100.0;
    let frequency = 440.0;

    let mut pcm = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
        let value = (sample * 16384.0) as i16;

        pcm.push(value);
    }

    pcm
}

// ============================================================================
// 场景 1: 单帧编码（1152 samples）- 单声道，Quality = 5
// ============================================================================

fn bench_lame_sys_single_frame(c: &mut Criterion) {
    let pcm = generate_pcm_data(1152);
    let mut mp3_buffer = vec![0u8; 8192];

    c.bench_function("lame-sys/single_frame_mono_q4", |b| {
        let mut encoder = lame_sys::LameEncoder::builder()
            .expect("Failed to create builder")
            .sample_rate(44100)
            .expect("Failed to set sample rate")
            .channels(1) // 单声道
            .expect("Failed to set channels")
            .bitrate(192)
            .expect("Failed to set bitrate")
            .quality(lame_sys::Quality::Standard) // Quality = 5
            .expect("Failed to set quality")
            .build()
            .expect("Failed to build encoder");

        b.iter(|| {
            // 使用正确的单声道编码方法
            encoder
                .encode_mono(black_box(&pcm), black_box(&mut mp3_buffer))
                .expect("Failed to encode")
        });
    });
}

// ============================================================================
// 场景 2: 完整编码流程（1000 frames = ~26 秒）- 单声道，Quality = 5
// ============================================================================

fn bench_lame_sys_complete(c: &mut Criterion) {
    let frame_size = 1152;
    let num_frames = 1000;
    let pcm = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer = vec![0u8; 624 * 1024];

    c.bench_function("lame-sys/complete_1000_frames_mono_q4", |b| {
        b.iter(|| {
            let mut encoder = lame_sys::LameEncoder::builder()
                .expect("Failed to create builder")
                .sample_rate(44100)
                .expect("Failed to set sample rate")
                .channels(1) // 单声道
                .expect("Failed to set channels")
                .bitrate(192)
                .expect("Failed to set bitrate")
                .quality(lame_sys::Quality::Standard)
                .expect("Failed to set quality")
                .build()
                .expect("Failed to build encoder");

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                let bytes = encoder
                    .encode_mono(black_box(&pcm[start..end]), black_box(&mut mp3_buffer))
                    .expect("Failed to encode");

                total_bytes += bytes;
            }

            let flush_bytes = encoder
                .flush(black_box(&mut mp3_buffer))
                .expect("Failed to flush");
            total_bytes + flush_bytes
        });
    });
}

// ============================================================================
// 竞品对比：mp3lame-encoder（相同测试场景）
// ============================================================================

fn bench_competitor_single_frame(c: &mut Criterion) {
    let pcm = generate_pcm_data(1152);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 8192];

    c.bench_function("competitor/single_frame_mono_q4", |b| {
        let mut builder = mp3lame_encoder::Builder::new().expect("Failed to create encoder");
        builder
            .set_sample_rate(44100)
            .expect("Failed to set sample rate");
        builder.set_num_channels(1).expect("Failed to set channels");
        builder
            .set_brate(mp3lame_encoder::Bitrate::Kbps192)
            .expect("Failed to set bitrate");
        builder
            .set_quality(mp3lame_encoder::Quality::Good)
            .expect("Failed to set quality"); // Quality = 5
        let mut encoder = builder.build().expect("Failed to build encoder");

        b.iter(|| {
            // 修复：单声道应该使用 MonoPcm，而不是 InterleavedPcm
            let input = mp3lame_encoder::MonoPcm(&pcm);
            encoder
                .encode(black_box(input), black_box(&mut mp3_buffer[..]))
                .expect("Failed to encode")
        });
    });
}

fn bench_competitor_complete(c: &mut Criterion) {
    let frame_size = 1152;
    let num_frames = 1000;
    let pcm = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 624 * 1024];

    c.bench_function("competitor/complete_1000_frames_mono_q4", |b| {
        b.iter(|| {
            let mut builder = mp3lame_encoder::Builder::new().expect("Failed to create encoder");
            builder
                .set_sample_rate(44100)
                .expect("Failed to set sample rate");
            builder.set_num_channels(1).expect("Failed to set channels");
            builder
                .set_brate(mp3lame_encoder::Bitrate::Kbps192)
                .expect("Failed to set bitrate");
            builder
                .set_quality(mp3lame_encoder::Quality::Good)
                .expect("Failed to set quality"); // Quality = 5
            let mut encoder = builder.build().expect("Failed to build encoder");

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                // 修复：单声道应该使用 MonoPcm，而不是 InterleavedPcm
                let input = mp3lame_encoder::MonoPcm(&pcm[start..end]);
                let bytes = encoder
                    .encode(black_box(input), black_box(&mut mp3_buffer[..]))
                    .expect("Failed to encode");

                total_bytes += bytes;
            }

            let flush_bytes = encoder
                .flush::<mp3lame_encoder::FlushNoGap>(black_box(&mut mp3_buffer[..]))
                .expect("Failed to flush");
            total_bytes + flush_bytes
        });
    });
}

criterion_group!(
    benches,
    bench_lame_sys_single_frame,
    bench_lame_sys_complete,
    bench_competitor_single_frame,
    bench_competitor_complete,
);

criterion_main!(benches);
