#!/usr/bin/env python3
"""
Compare different quality settings and their encoding speeds
"""

import lame
import time
import math


def generate_test_audio(duration=1.0, sample_rate=44100):
    """Generate test audio with multiple frequencies"""
    num_samples = int(duration * sample_rate)
    samples = []

    # Mix of frequencies: 220Hz, 440Hz, 880Hz
    frequencies = [220, 440, 880]
    amplitudes = [0.3, 0.5, 0.2]

    for i in range(num_samples):
        t = i / sample_rate
        value = 0
        for freq, amp in zip(frequencies, amplitudes):
            value += amp * math.sin(2 * math.pi * freq * t)
        samples.append(int(32767 * value / len(frequencies)))

    return samples


def encode_with_quality(quality, pcm_data):
    """Encode audio with specified quality"""
    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .quality(quality)
        .build()
    )

    start_time = time.time()

    # Encode in chunks
    mp3_chunks = []
    chunk_size = 1152

    for i in range(0, len(pcm_data), chunk_size):
        chunk = pcm_data[i:i + chunk_size]
        if len(chunk) < chunk_size:
            chunk.extend([0] * (chunk_size - len(chunk)))

        mp3_data = encoder.encode_mono(chunk)
        mp3_chunks.append(mp3_data)

    # Flush
    final_data = encoder.flush()
    mp3_chunks.append(final_data)

    elapsed = time.time() - start_time
    total_size = sum(len(chunk) for chunk in mp3_chunks)

    return b''.join(mp3_chunks), elapsed, total_size


def main():
    print("LAME Quality Comparison")
    print("=" * 60)
    print()

    # Generate test audio (10 seconds)
    print("Generating 10 seconds of test audio...")
    pcm_data = generate_test_audio(duration=10.0)
    print(f"PCM samples: {len(pcm_data)}")
    print()

    # Test different quality settings
    qualities = [
        (lame.Quality.Best, "Best"),
        (lame.Quality.High, "High"),
        (lame.Quality.Good, "Good"),
        (lame.Quality.Standard, "Standard"),
        (lame.Quality.Fast, "Fast"),
        (lame.Quality.Fastest, "Fastest"),
    ]

    results = []

    for quality, name in qualities:
        print(f"Encoding with {name} quality...")
        mp3_data, elapsed, size = encode_with_quality(quality, pcm_data)
        results.append((name, elapsed, size, mp3_data))
        print(f"  Time: {elapsed:.4f}s, Size: {size} bytes")

    print()
    print("Summary")
    print("=" * 60)
    print(f"{'Quality':<12} {'Time (ms)':<12} {'Size (bytes)':<15} {'Speed'}")
    print("-" * 60)

    baseline_time = results[0][1]  # Best quality as baseline

    for name, elapsed, size, _ in results:
        speed_ratio = baseline_time / elapsed
        print(f"{name:<12} {elapsed*1000:<12.2f} {size:<15} {speed_ratio:.2f}x")

    # Save one example
    output_file = "quality_test_standard.mp3"
    standard_result = next(r for r in results if r[0] == "Standard")
    with open(output_file, "wb") as f:
        f.write(standard_result[3])

    print()
    print(f"Sample output saved to: {output_file}")


if __name__ == "__main__":
    main()
