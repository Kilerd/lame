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
// 场景 1: 单帧编码（1152 samples）- 单声道，Quality = 4
// ============================================================================

fn bench_lame_sys_single_frame(c: &mut Criterion) {
    let pcm = generate_pcm_data(1152);
    let mut mp3_buffer = vec![0u8; 8192];

    c.bench_function("lame-sys/single_frame_mono_q4", |b| {
        let mut encoder = lame_sys::LameEncoder::builder()
            .sample_rate(44100)
            .channels(1) // 单声道
            .bitrate(192)
            .quality(lame_sys::Quality::Good) // Quality = 4
            .build()
            .unwrap();

        b.iter(|| {
            encoder
                .encode_mono(black_box(&pcm), black_box(&mut mp3_buffer))
                .unwrap()
        });
    });
}

fn bench_mp3lame_encoder_single_frame(c: &mut Criterion) {
    let pcm = generate_pcm_data(1152);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 8192];

    c.bench_function("mp3lame-encoder/single_frame_mono_q4", |b| {
        let mut encoder = mp3lame_encoder::Builder::new().unwrap();
        encoder.set_num_channels(1).unwrap(); // 单声道
        encoder.set_sample_rate(44100).unwrap();
        encoder
            .set_brate(mp3lame_encoder::Bitrate::Kbps192)
            .unwrap();
        encoder.set_quality(mp3lame_encoder::Quality::Good).unwrap(); // Quality = 4
        let mut encoder = encoder.build().unwrap();

        b.iter(|| {
            let input = mp3lame_encoder::MonoPcm(black_box(&pcm));
            encoder.encode(input, black_box(&mut mp3_buffer)).unwrap()
        });
    });
}

// ============================================================================
// 场景 2: 完整编码流程（1000 frames = ~26 秒）- 单声道，Quality = 4
// ============================================================================

fn bench_lame_sys_complete(c: &mut Criterion) {
    let frame_size = 1152;
    let num_frames = 1000;
    let pcm = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer = vec![0u8; 8192];

    c.bench_function("lame-sys/complete_1000_frames_mono_q4", |b| {
        b.iter(|| {
            let mut encoder = lame_sys::LameEncoder::builder()
                .sample_rate(44100)
                .channels(1) // 单声道
                .bitrate(192)
                .quality(lame_sys::Quality::Good) // Quality = 4
                .build()
                .unwrap();

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                let bytes = encoder
                    .encode_mono(black_box(&pcm[start..end]), black_box(&mut mp3_buffer))
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
    let pcm = generate_pcm_data(frame_size * num_frames);
    let mut mp3_buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 8192];

    c.bench_function("mp3lame-encoder/complete_1000_frames_mono_q4", |b| {
        b.iter(|| {
            let mut encoder = mp3lame_encoder::Builder::new().unwrap();
            encoder.set_num_channels(1).unwrap(); // 单声道
            encoder.set_sample_rate(44100).unwrap();
            encoder
                .set_brate(mp3lame_encoder::Bitrate::Kbps192)
                .unwrap();
            encoder.set_quality(mp3lame_encoder::Quality::Good).unwrap(); // Quality = 4
            let mut encoder = encoder.build().unwrap();

            let mut total_bytes = 0;
            for i in 0..num_frames {
                let start = i * frame_size;
                let end = start + frame_size;

                let input = mp3lame_encoder::MonoPcm(black_box(&pcm[start..end]));

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
