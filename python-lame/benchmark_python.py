#!/usr/bin/env python3
"""
Python LAME Encoder Benchmark - 对比性能测试

对比两个 Python LAME 绑定的性能：
1. lame (本项目 - PyO3 + Rust 实现)
2. lameenc (现有的 Python 包)

安装依赖:
    pip install lameenc numpy

运行:
    python benchmark_python.py
"""

import time
import math
import sys
import numpy as np

# 尝试导入两个库
try:
    import lame as lame_pyo3

    HAS_LAME_PYO3 = True
except ImportError:
    HAS_LAME_PYO3 = False
    print("警告: 无法导入 lame 模块 (PyO3 实现)")

try:
    import lameenc

    HAS_LAMEENC = True
except ImportError:
    HAS_LAMEENC = False
    print("警告: 无法导入 lameenc 模块")

if not HAS_LAME_PYO3 and not HAS_LAMEENC:
    print("\n错误: 没有可用的 LAME 库！")
    print("请安装至少一个: lame (本项目) 或 lameenc")
    sys.exit(1)


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
        min_time = np.min(times_us)
        max_time = np.max(times_us)
        median = np.median(times_us)
        return mean, std, min_time, max_time, median

    def report(self):
        mean, std, min_time, max_time, median = self.analyze()
        if mean < 1000:  # 小于1ms，显示微秒
            print(f"{self.name}")
            print(f"  平均: {mean:.2f} µs")
            print(f"  中位: {median:.2f} µs")
            print(f"  标准差: ±{std:.2f} µs")
            print(f"  范围: [{min_time:.2f}, {max_time:.2f}] µs")
        else:  # 大于等于1ms，显示毫秒
            print(f"{self.name}")
            print(f"  平均: {mean / 1000:.2f} ms")
            print(f"  中位: {median / 1000:.2f} ms")
            print(f"  标准差: ±{std / 1000:.2f} ms")
            print(f"  范围: [{min_time / 1000:.2f}, {max_time / 1000:.2f}] ms")
        print()


def generate_pcm_data(num_samples):
    """
    生成单声道 440 Hz 正弦波 PCM 数据

    Args:
        num_samples: 样本数量

    Returns:
        numpy array of int16 和 list of int
    """
    sample_rate = 44100.0
    frequency = 440.0

    pcm_np = np.zeros(num_samples, dtype=np.int16)

    for i in range(num_samples):
        t = i / sample_rate
        sample = math.sin(2.0 * math.pi * frequency * t)
        pcm_np[i] = int(sample * 16384.0)

    pcm_list = pcm_np.tolist()
    pcm_bytes = pcm_np.tobytes()  # For PyO3 bindings that use bytes

    return pcm_np, pcm_list, pcm_bytes


def benchmark_lame_pyo3_single_frame(num_iterations=100):
    """使用 lame (PyO3) 进行单帧编码测试"""
    if not HAS_LAME_PYO3:
        return None

    # 生成测试数据
    _, pcm_list, pcm_bytes = generate_pcm_data(1152)

    stats = BenchmarkStats("lame (PyO3) - 单帧编码")

    # 预热
    for _ in range(10):
        builder = lame_pyo3.LameEncoder.builder()
        builder.sample_rate(44100)
        builder.channels(1)
        builder.bitrate(128)
        builder.quality(lame_pyo3.Quality.Standard)
        encoder = builder.build()
        encoder.encode_mono(pcm_bytes)

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for i in range(num_iterations):
        # 创建编码器
        builder = lame_pyo3.LameEncoder.builder()
        builder.sample_rate(44100)
        builder.channels(1)
        builder.bitrate(128)
        builder.quality(lame_pyo3.Quality.Standard)
        encoder = builder.build()

        # 计时开始
        start = time.perf_counter()

        # 编码
        mp3_data = encoder.encode_mono(pcm_bytes)

        # 计时结束
        end = time.perf_counter()

        stats.add_time(end - start)

        if (i + 1) % 20 == 0:
            print(f"  完成 {i + 1}/{num_iterations} 次迭代")

    stats.report()
    return stats


def benchmark_lameenc_single_frame(num_iterations=100):
    """使用 lameenc 进行单帧编码测试"""
    if not HAS_LAMEENC:
        return None

    # 生成测试数据
    pcm_np, _, _ = generate_pcm_data(1152)

    stats = BenchmarkStats("lameenc - 单帧编码")

    # 预热
    for _ in range(10):
        encoder = lameenc.Encoder()
        encoder.set_bit_rate(128)
        encoder.set_in_sample_rate(44100)
        encoder.set_channels(1)
        encoder.set_quality(5)  # Quality = 5 (Standard)
        encoder.silence()
        encoder.encode(pcm_np.tobytes())

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for i in range(num_iterations):
        # 创建编码器
        encoder = lameenc.Encoder()
        encoder.set_bit_rate(128)
        encoder.set_in_sample_rate(44100)
        encoder.set_channels(1)
        encoder.set_quality(5)
        encoder.silence()

        # 计时开始
        start = time.perf_counter()

        # 编码
        mp3_data = encoder.encode(pcm_np.tobytes())

        # 计时结束
        end = time.perf_counter()

        stats.add_time(end - start)

        if (i + 1) % 20 == 0:
            print(f"  完成 {i + 1}/{num_iterations} 次迭代")

    stats.report()
    return stats


def benchmark_lame_pyo3_complete_flow(num_frames=1000, num_iterations=100):
    """使用 lame (PyO3) 进行完整编码流程测试"""
    if not HAS_LAME_PYO3:
        return None

    frame_size = 1152
    total_samples = frame_size * num_frames

    # 生成测试数据
    _, pcm_list, pcm_bytes = generate_pcm_data(total_samples)

    stats = BenchmarkStats(f"lame (PyO3) - 完整流程 ({num_frames} frames)")

    # 预热
    builder = lame_pyo3.LameEncoder.builder()
    builder.sample_rate(44100)
    builder.channels(1)
    builder.bitrate(128)
    builder.quality(lame_pyo3.Quality.Standard)
    encoder = builder.build()

    for i in range(10):
        start_idx = (i % 10) * frame_size * 2  # *2 because bytes (i16 = 2 bytes)
        end_idx = start_idx + frame_size * 2
        encoder.encode_mono(pcm_bytes[start_idx:end_idx])

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for iteration in range(num_iterations):
        # 创建编码器
        builder = lame_pyo3.LameEncoder.builder()
        builder.sample_rate(44100)
        builder.channels(1)
        builder.bitrate(128)
        builder.quality(lame_pyo3.Quality.Standard)
        encoder = builder.build()

        # 计时开始
        start = time.perf_counter()

        # 编码所有帧
        total_bytes = 0
        for i in range(num_frames):
            start_idx = i * frame_size * 2  # *2 because bytes (i16 = 2 bytes)
            end_idx = start_idx + frame_size * 2

            mp3_data = encoder.encode_mono(pcm_bytes[start_idx:end_idx])
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
    mean_time, _, _, _, _ = stats.analyze()
    audio_duration = num_frames * frame_size / 44100.0  # 音频时长（秒）
    encoding_time = mean_time / 1_000_000.0  # 编码时间（秒）
    realtime_factor = audio_duration / encoding_time

    print(f"实时编码能力: {realtime_factor:.2f}x")
    print(f"  （可以在 1 秒内编码 {realtime_factor:.2f} 秒的音频）")
    print()

    return stats


def benchmark_lameenc_complete_flow(num_frames=1000, num_iterations=100):
    """使用 lameenc 进行完整编码流程测试"""
    if not HAS_LAMEENC:
        return None

    frame_size = 1152
    total_samples = frame_size * num_frames

    # 生成测试数据
    pcm_np, _, _ = generate_pcm_data(total_samples)

    stats = BenchmarkStats(f"lameenc - 完整流程 ({num_frames} frames)")

    # 预热
    encoder = lameenc.Encoder()
    encoder.set_bit_rate(128)
    encoder.set_in_sample_rate(44100)
    encoder.set_channels(1)
    encoder.set_quality(5)
    encoder.silence()

    for i in range(10):
        start_idx = (i % 10) * frame_size
        end_idx = start_idx + frame_size
        encoder.encode(pcm_np[start_idx:end_idx].tobytes())

    # 正式测试
    print(f"运行 {num_iterations} 次迭代...")

    for iteration in range(num_iterations):
        # 创建编码器
        encoder = lameenc.Encoder()
        encoder.set_bit_rate(128)
        encoder.set_in_sample_rate(44100)
        encoder.set_channels(1)
        encoder.set_quality(5)
        encoder.silence()

        # 计时开始
        start = time.perf_counter()

        # 编码所有帧
        total_bytes = 0
        for i in range(num_frames):
            start_idx = i * frame_size
            end_idx = start_idx + frame_size

            mp3_data = encoder.encode(pcm_np[start_idx:end_idx].tobytes())
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
    mean_time, _, _, _, _ = stats.analyze()
    audio_duration = num_frames * frame_size / 44100.0  # 音频时长（秒）
    encoding_time = mean_time / 1_000_000.0  # 编码时间（秒）
    realtime_factor = audio_duration / encoding_time

    print(f"实时编码能力: {realtime_factor:.2f}x")
    print(f"  （可以在 1 秒内编码 {realtime_factor:.2f} 秒的音频）")
    print()

    return stats


def compare_stats(stats1, stats2, label1, label2):
    """对比两个统计结果"""
    if stats1 is None or stats2 is None:
        return

    mean1, _, _, _, _ = stats1.analyze()
    mean2, _, _, _, _ = stats2.analyze()

    speedup = mean2 / mean1
    improvement = ((mean2 - mean1) / mean2) * 100

    print(f"\n性能对比: {label1} vs {label2}")
    print(f"  {label1} 平均: {mean1:.2f} µs")
    print(f"  {label2} 平均: {mean2:.2f} µs")
    print(f"  加速比: {speedup:.2f}x")
    print(f"  性能提升: {improvement:+.2f}%")
    print()


def main():
    print("\n" + "=" * 70)
    print("Python LAME Encoder 性能对比测试")
    print("=" * 70)
    print()

    print("可用的库:")
    if HAS_LAME_PYO3:
        print(f"  ✓ lame (PyO3) - 版本: {lame_pyo3.get_version()}")
    else:
        print("  ✗ lame (PyO3) - 未安装")

    if HAS_LAMEENC:
        try:
            print(f"  ✓ lameenc - 版本: {lameenc.__version__}")
        except AttributeError:
            print("  ✓ lameenc - 版本: (无法获取)")
    else:
        print("  ✗ lameenc - 未安装")

    print()
    print("测试配置:")
    print("  - 采样率: 44100 Hz")
    print("  - 声道数: 1 (单声道)")
    print("  - 比特率: 128 kbps")
    print("  - 质量级别: 5 (Standard)")
    print("  - 测试数据: 440 Hz 正弦波")
    print()

    # ==================== 场景 1: 单帧编码 ====================
    print("=" * 70)
    print("场景 1: 单帧编码 (1152 samples = ~26 ms 音频)")
    print("=" * 70)
    print()

    pyo3_single = None
    lameenc_single = None

    if HAS_LAME_PYO3:
        print("测试 lame (PyO3)...")
        pyo3_single = benchmark_lame_pyo3_single_frame(num_iterations=100)

    if HAS_LAMEENC:
        print("测试 lameenc...")
        lameenc_single = benchmark_lameenc_single_frame(num_iterations=100)

    if pyo3_single and lameenc_single:
        compare_stats(pyo3_single, lameenc_single, "lame (PyO3)", "lameenc")

    # ==================== 场景 2: 完整流程 ====================
    print("=" * 70)
    print("场景 2: 完整编码流程 (1000 frames = ~26 秒音频)")
    print("=" * 70)
    print()

    pyo3_complete = None
    lameenc_complete = None

    if HAS_LAME_PYO3:
        print("测试 lame (PyO3)...")
        pyo3_complete = benchmark_lame_pyo3_complete_flow(
            num_frames=1000, num_iterations=100
        )

    if HAS_LAMEENC:
        print("测试 lameenc...")
        lameenc_complete = benchmark_lameenc_complete_flow(
            num_frames=1000, num_iterations=100
        )

    if pyo3_complete and lameenc_complete:
        compare_stats(pyo3_complete, lameenc_complete, "lame (PyO3)", "lameenc")

    # ==================== 总结 ====================
    print("=" * 70)
    print("性能测试总结")
    print("=" * 70)
    print()

    if pyo3_single:
        mean, std, _, _, _ = pyo3_single.analyze()
        print(f"lame (PyO3) - 单帧编码:")
        print(f"  {mean:.2f} µs ± {std:.2f} µs")

    if lameenc_single:
        mean, std, _, _, _ = lameenc_single.analyze()
        print(f"lameenc - 单帧编码:")
        print(f"  {mean:.2f} µs ± {std:.2f} µs")

    print()

    if pyo3_complete:
        mean, std, _, _, _ = pyo3_complete.analyze()
        print(f"lame (PyO3) - 完整流程 (1000 frames):")
        print(f"  {mean / 1000:.2f} ms ± {std / 1000:.2f} ms")

    if lameenc_complete:
        mean, std, _, _, _ = lameenc_complete.analyze()
        print(f"lameenc - 完整流程 (1000 frames):")
        print(f"  {mean / 1000:.2f} ms ± {std / 1000:.2f} ms")

    print()
    print("=" * 70)
    print("测试完成！")
    print("=" * 70)


if __name__ == "__main__":
    main()
