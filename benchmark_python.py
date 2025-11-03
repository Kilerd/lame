#!/usr/bin/env python3
"""
Python LAME Encoder Benchmark
使用 lameenc 库进行性能测试，与 Rust 实现对比

安装依赖:
    pip install lameenc numpy

运行:
    python benchmark_python.py
"""

import time
import math
import numpy as np
import lameenc


# 统计相关
class BenchmarkStats:
    def __init__(self, name):
        self.name = name
        self.times = []

    def add_time(self, duration):
        self.times.append(duration)

    def analyze(self):
        times_us = [t * 1_000_000 for t in self.times]  # 转换为微秒
        mean = np.mean(times_us)
        std = np.std(times_us)
        return mean, std

    def report(self):
        mean, std = self.analyze()
        if mean < 1000:  # 小于1ms，显示微秒
            print(f"{self.name}")
            print(f"  平均时间: {mean:.2f} µs")
            print(f"  标准差:   ±{std:.2f} µs")
        else:  # 大于等于1ms，显示毫秒
            print(f"{self.name}")
            print(f"  平均时间: {mean / 1000:.2f} ms")
            print(f"  标准差:   ±{std / 1000:.2f} ms")
        print()


def generate_pcm_data(num_samples):
    """
    生成单声道 440 Hz 正弦波 PCM 数据

    Args:
        num_samples: 样本数量

    Returns:
        numpy array of int16
    """
    sample_rate = 44100.0
    frequency = 440.0

    pcm = np.zeros(num_samples, dtype=np.int16)

    for i in range(num_samples):
        t = i / sample_rate
        sample = math.sin(2.0 * math.pi * frequency * t)
        pcm[i] = int(sample * 16384.0)

    return pcm


def benchmark_single_frame(num_iterations=100):
    """
    场景 1: 单帧编码（1152 samples = ~26 ms 音频）
    """
    print("=" * 60)
    print("场景 1: 单帧编码 (1152 samples)")
    print("=" * 60)

    # 生成测试数据
    pcm = generate_pcm_data(1152)

    stats = BenchmarkStats("lameenc/single_frame_mono_q4")

    # 预热
    encoder = lameenc.Encoder()
    encoder.set_bit_rate(192)
    encoder.set_in_sample_rate(44100)
    encoder.set_channels(1)
    encoder.set_quality(4)  # Quality = 4 (Good)
    encoder.silence()

    for _ in range(10):
        encoder.encode(pcm.tobytes())

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for i in range(num_iterations):
        # 创建编码器
        encoder = lameenc.Encoder()
        encoder.set_bit_rate(192)
        encoder.set_in_sample_rate(44100)
        encoder.set_channels(1)
        encoder.set_quality(4)
        encoder.silence()

        # 计时开始
        start = time.perf_counter()

        # 编码
        mp3_data = encoder.encode(pcm.tobytes())

        # 计时结束
        end = time.perf_counter()

        stats.add_time(end - start)

        if (i + 1) % 20 == 0:
            print(f"  完成 {i + 1}/{num_iterations} 次迭代")

    stats.report()
    return stats


def benchmark_complete_flow(num_frames=1000, num_iterations=100):
    """
    场景 2: 完整编码流程（1000 frames = ~26 秒音频）
    """
    print("=" * 60)
    print(
        f"场景 2: 完整编码流程 ({num_frames} frames = ~{num_frames * 1152 / 44100:.1f} 秒音频)"
    )
    print("=" * 60)

    frame_size = 1152
    total_samples = frame_size * num_frames

    # 生成测试数据
    pcm = generate_pcm_data(total_samples)

    stats = BenchmarkStats(f"lameenc/complete_{num_frames}_frames_mono_q4")

    # 预热
    encoder = lameenc.Encoder()
    encoder.set_bit_rate(192)
    encoder.set_in_sample_rate(44100)
    encoder.set_channels(1)
    encoder.set_quality(4)
    encoder.silence()

    for i in range(10):
        start_idx = (i % 10) * frame_size
        end_idx = start_idx + frame_size
        encoder.encode(pcm[start_idx:end_idx].tobytes())

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for iteration in range(num_iterations):
        # 创建编码器
        encoder = lameenc.Encoder()
        encoder.set_bit_rate(192)
        encoder.set_in_sample_rate(44100)
        encoder.set_channels(1)
        encoder.set_quality(4)
        encoder.silence()

        # 计时开始
        start = time.perf_counter()

        # 编码所有帧
        total_bytes = 0
        for i in range(num_frames):
            start_idx = i * frame_size
            end_idx = start_idx + frame_size

            mp3_data = encoder.encode(pcm[start_idx:end_idx].tobytes())
            total_bytes += len(mp3_data)

        # 刷新缓冲区
        final_data = encoder.flush()
        total_bytes += len(final_data)

        # 计时结束
        end = time.perf_counter()

        stats.add_time(end - start)

        if (iteration + 1) % 10 == 0:
            print(
                f"  完成 {iteration + 1}/{num_iterations} 次迭代 (编码了 {total_bytes} 字节)"
            )

    stats.report()

    # 计算实时编码能力
    mean_time, _ = stats.analyze()
    audio_duration = num_frames * frame_size / 44100.0  # 音频时长（秒）
    encoding_time = mean_time / 1_000_000.0  # 编码时间（秒）
    realtime_factor = audio_duration / encoding_time

    print(f"实时编码能力: {realtime_factor:.2f}x")
    print(f"  （可以在 1 秒内编码 {realtime_factor:.2f} 秒的音频）")
    print()

    return stats


def main():
    print("\n" + "=" * 60)
    print("Python LAME Encoder Benchmark")
    print("使用 lameenc 库")
    print("=" * 60)
    print()

    print("测试配置:")
    print("  - 采样率: 44100 Hz")
    print("  - 声道数: 1 (单声道)")
    print("  - 比特率: 192 kbps")
    print("  - 质量级别: 4 (Good)")
    print("  - 测试数据: 440 Hz 正弦波")
    print()

    # 检查 lameenc 版本
    try:
        print(f"lameenc 版本: {lameenc.__version__}")
    except AttributeError:
        print("lameenc 版本: (无法获取)")
    print()

    # 运行基准测试
    single_frame_stats = benchmark_single_frame(num_iterations=100)
    complete_flow_stats = benchmark_complete_flow(num_frames=1000, num_iterations=100)

    # 总结
    print("=" * 60)
    print("基准测试总结")
    print("=" * 60)

    single_mean, single_std = single_frame_stats.analyze()
    complete_mean, complete_std = complete_flow_stats.analyze()

    print(f"\n单帧编码:")
    print(f"  平均: {single_mean:.2f} µs ± {single_std:.2f} µs")

    print(f"\n完整流程 (1000 frames):")
    print(f"  平均: {complete_mean / 1000:.2f} ms ± {complete_std / 1000:.2f} ms")

    print("\n" + "=" * 60)
    print("测试完成！")
    print("=" * 60)


if __name__ == "__main__":
    main()
