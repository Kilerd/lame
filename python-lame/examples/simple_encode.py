#!/usr/bin/env python3
"""
Simple example of encoding PCM to MP3 using the LAME encoder
"""

import lame
import math


def generate_sine_wave(frequency, duration, sample_rate=44100):
    """Generate a sine wave PCM signal"""
    num_samples = int(duration * sample_rate)
    samples = []
    for i in range(num_samples):
        t = i / sample_rate
        value = int(32767 * 0.5 * math.sin(2 * math.pi * frequency * t))
        samples.append(value)
    return samples


def main():
    print(f"LAME version: {lame.get_version()}")
    print(f"LAME URL: {lame.get_url()}")
    print()

    # Create encoder with standard settings
    print("Creating encoder...")
    encoder = (
        lame.LameEncoder.builder()
        .sample_rate(44100)
        .channels(1)
        .bitrate(128)
        .quality(lame.Quality.Standard)
        .build()
    )

    # Set ID3 tags
    print("Setting ID3 tags...")
    encoder.id3_tag() \
        .title("Test Tone") \
        .artist("Python LAME Example") \
        .album("Examples") \
        .year("2024") \
        .comment("Generated sine wave at 440 Hz") \
        .track(1) \
        .genre("Electronic") \
        .apply()

    # Generate a 440 Hz sine wave (A4 note) for 3 seconds
    print("Generating test signal...")
    pcm_data = generate_sine_wave(440, 3.0)

    # Encode in chunks
    print("Encoding...")
    mp3_chunks = []
    chunk_size = 1152  # Standard MP3 frame size

    for i in range(0, len(pcm_data), chunk_size):
        chunk = pcm_data[i:i + chunk_size]
        # Pad last chunk if needed
        if len(chunk) < chunk_size:
            chunk.extend([0] * (chunk_size - len(chunk)))

        mp3_data = encoder.encode_mono(chunk)
        mp3_chunks.append(mp3_data)

    # Flush encoder
    print("Flushing encoder...")
    final_data = encoder.flush()
    mp3_chunks.append(final_data)

    # Combine all chunks
    complete_mp3 = b''.join(mp3_chunks)

    # Write to file
    output_file = "output.mp3"
    with open(output_file, "wb") as f:
        f.write(complete_mp3)

    print(f"\nEncoded MP3 written to: {output_file}")
    print(f"Output size: {len(complete_mp3)} bytes")
    print(f"Duration: ~3 seconds")
    print("\nYou can play it with: ffplay output.mp3")


if __name__ == "__main__":
    main()
