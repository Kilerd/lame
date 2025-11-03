use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem::MaybeUninit;

// 生成测试 PCM 数据（440 Hz 正弦波）
fn generate_pcm_data(num_samples: usize) -> (Vec<i16>, Vec<i16>) {
    let sample_rate = 44100.0;
    let frequency = 440.0;

    let mut left = Vec::with_capacity(num_samples);
    let mut right = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
        let value = (sample * 16384.0) as i16;

        left.push(value);
        right.push(value);
    }

    (left, right)
}

// ============================================================================
// 场景 1: 单帧编码（1152 samples）
// ============================================================================

fn bench_lame_sys_single_frame(c: &mut Criterion) {
    let (pcm_left, pcm_right) = generate_pcm_data(1152);
    let mut mp3_buffer = vec![0u8; 8192];

    c.bench_function("lame-sys/single_frame", |b| {
        let mut encoder = lame_sys::LameEncoder::builder()
            .sample_rate(44100)
            .channels(2)
            .bitrate(192)
            .quality(lame_sys::Quality::Standard)
            .build()
            .unwrap();

        b.iter(|| {
            encoder
                .encode(
                    black_box(&pcm_left),
                    black_box(&pcm_right),
                    black_box(&mut mp3_buffer),
                )
                .unwrap()
        });
    });
}

fn bench_mp3lame_encoder_single_frame(c: &mut Criterion) {
    let (pcm_left, pcm_right) = generate_pcm_data(1152);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 8192];

    c.bench_function("mp3lame-encoder/single_frame", |b| {
        let mut encoder = mp3lame_encoder::Builder::new().unwrap();
        encoder.set_num_channels(2).unwrap();
        encoder.set_sample_rate(44100).unwrap();
        encoder
            .set_brate(mp3lame_encoder::Bitrate::Kbps192)
            .unwrap();
        encoder.set_quality(mp3lame_encoder::Quality::Good).unwrap();
        let mut encoder = encoder.build().unwrap();

        b.iter(|| {
            let input = mp3lame_encoder::DualPcm {
                left: black_box(&pcm_left),
                right: black_box(&pcm_right),
            };
            encoder.encode(input, black_box(&mut mp3_buffer)).unwrap()
        });
    });
}

// ============================================================================
// 场景 2: 完整编码流程（1000 frames = ~26 秒）
// ============================================================================

fn bench_lame_sys_complete(c: &mut Criterion) {
    let frame_size = 1152;
    let num_frames = 1000;
    let (pcm_left, pcm_right) = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer = vec![0u8; 8192];

    c.bench_function("lame-sys/complete_1000_frames", |b| {
        b.iter(|| {
            let mut encoder = lame_sys::LameEncoder::builder()
                .sample_rate(44100)
                .channels(1)
                .bitrate(192)
                .quality(lame_sys::Quality::LowStandard)
                .build()
                .unwrap();

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                let bytes = encoder
                    .encode(
                        black_box(&pcm_left[start..end]),
                        black_box(&pcm_right[start..end]),
                        black_box(&mut mp3_buffer),
                    )
                    .unwrap();

                total_bytes += bytes;
            }

            let flush_bytes = encoder.flush(black_box(&mut mp3_buffer)).unwrap();
            total_bytes + flush_bytes
        });
    });
}

fn bench_mp3lame_encoder_complete(c: &mut Criterion) {
    let frame_size = 1152;
    let num_frames = 1000;
    let (pcm_left, pcm_right) = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 8192];

    c.bench_function("mp3lame-encoder/complete_1000_frames", |b| {
        b.iter(|| {
            let mut encoder = mp3lame_encoder::Builder::new().unwrap();
            encoder.set_num_channels(2).unwrap();
            encoder.set_sample_rate(44100).unwrap();
            encoder
                .set_brate(mp3lame_encoder::Bitrate::Kbps192)
                .unwrap();
            encoder.set_quality(mp3lame_encoder::Quality::Good).unwrap();
            let mut encoder = encoder.build().unwrap();

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                let input = mp3lame_encoder::DualPcm {
                    left: black_box(&pcm_left[start..end]),
                    right: black_box(&pcm_right[start..end]),
                };

                let bytes = encoder.encode(input, black_box(&mut mp3_buffer)).unwrap();
                total_bytes += bytes;
            }

            let flush_bytes = encoder
                .flush::<mp3lame_encoder::FlushNoGap>(black_box(&mut mp3_buffer))
                .unwrap();
            total_bytes + flush_bytes
        });
    });
}

criterion_group!(
    benches,
    bench_lame_sys_single_frame,
    bench_mp3lame_encoder_single_frame,
    bench_lame_sys_complete,
    bench_mp3lame_encoder_complete,
);

criterion_main!(benches);
