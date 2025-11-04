#!/usr/bin/env python3
"""
Test NumPy interface performance
"""

import time
import numpy as np
import lame

def test_numpy_interface():
    """Test encode_mono_numpy with NumPy array"""
    print("Testing NumPy interface...")

    # Create encoder
    builder = lame.LameEncoder.builder()
    builder.sample_rate(44100)
    builder.channels(1)
    builder.bitrate(128)
    builder.quality(lame.Quality.Good)
    encoder = builder.build()

    # Generate test data as NumPy array
    pcm_np = np.array([0] * 1152, dtype=np.int16)

    # Warm up
    for _ in range(10):
        encoder.encode_mono_numpy(pcm_np)

    # Benchmark
    iterations = 1000
    start = time.perf_counter()
    for _ in range(iterations):
        mp3_data = encoder.encode_mono_numpy(pcm_np)
    end = time.perf_counter()

    avg_time_us = (end - start) / iterations * 1_000_000
    print(f"NumPy interface average: {avg_time_us:.2f} µs")

    return avg_time_us

def test_bytes_interface():
    """Test encode_mono with bytes (for comparison)"""
    print("Testing bytes interface...")

    # Create encoder
    builder = lame.LameEncoder.builder()
    builder.sample_rate(44100)
    builder.channels(1)
    builder.bitrate(128)
    builder.quality(lame.Quality.Good)
    encoder = builder.build()

    # Generate test data as bytes
    pcm_np = np.array([0] * 1152, dtype=np.int16)
    pcm_bytes = pcm_np.tobytes()

    # Warm up
    for _ in range(10):
        encoder.encode_mono(pcm_bytes)

    # Benchmark
    iterations = 1000
    start = time.perf_counter()
    for _ in range(iterations):
        mp3_data = encoder.encode_mono(pcm_bytes)
    end = time.perf_counter()

    avg_time_us = (end - start) / iterations * 1_000_000
    print(f"Bytes interface average: {avg_time_us:.2f} µs")

    return avg_time_us

if __name__ == "__main__":
    print("=" * 70)
    print("NumPy Interface Performance Test")
    print("=" * 70)
    print()

    numpy_time = test_numpy_interface()
    print()
    bytes_time = test_bytes_interface()

    print()
    print("=" * 70)
    print("Results:")
    print("=" * 70)
    print(f"NumPy interface: {numpy_time:.2f} µs")
    print(f"Bytes interface: {bytes_time:.2f} µs")

    if numpy_time < bytes_time:
        speedup = bytes_time / numpy_time
        print(f"NumPy is {speedup:.2f}x faster!")
    else:
        slowdown = numpy_time / bytes_time
        print(f"NumPy is {slowdown:.2f}x slower")

    print()
    print("✅ NumPy interface is working correctly!")
