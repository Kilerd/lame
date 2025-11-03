# Python LAME Encoder Benchmark

这是一个等价于 Rust benchmark 的 Python 脚本，使用 `lameenc` 库来测试 Python LAME 绑定的性能。

## 安装依赖

```bash
pip install lameenc numpy
```

或使用 pip3：

```bash
pip3 install lameenc numpy
```

## 运行 Benchmark

```bash
python3 benchmark_python.py
```

或直接执行：

```bash
./benchmark_python.py
```

## 测试场景

与 Rust benchmark 完全相同：

### 场景 1: 单帧编码
- 测试数据：1152 samples (~26 ms 音频)
- 迭代次数：100 次
- 目的：测试单次编码调用的性能

### 场景 2: 完整编码流程
- 测试数据：1000 frames (1,152,000 samples, ~26 秒音频)
- 迭代次数：100 次
- 目的：测试完整编码流程，包括初始化和刷新

## 测试配置

所有测试使用相同的配置：

- **采样率**: 44100 Hz
- **声道数**: 1 (单声道)
- **比特率**: 192 kbps
- **质量级别**: 4 (Good)
- **测试数据**: 440 Hz 正弦波 (50% 音量)

## 输出示例

```
============================================================
Python LAME Encoder Benchmark
使用 lameenc 库
============================================================

测试配置:
  - 采样率: 44100 Hz
  - 声道数: 1 (单声道)
  - 比特率: 192 kbps
  - 质量级别: 4 (Good)
  - 测试数据: 440 Hz 正弦波

============================================================
场景 1: 单帧编码 (1152 samples)
============================================================
运行 100 次迭代...
lameenc/single_frame_mono_q4
  平均时间: 45.23 µs
  标准差:   ±2.15 µs

============================================================
场景 2: 完整编码流程 (1000 frames = ~26.1 秒音频)
============================================================
运行 100 次迭代...
lameenc/complete_1000_frames_mono_q4
  平均时间: 52.34 ms
  标准差:   ±1.23 ms

实时编码能力: 498.52x
  （可以在 1 秒内编码 498.52 秒的音频）

============================================================
基准测试总结
============================================================

单帧编码:
  平均: 45.23 µs ± 2.15 µs

完整流程 (1000 frames):
  平均: 52.34 ms ± 1.23 ms

============================================================
测试完成！
============================================================
```

## 与 Rust Benchmark 对比

运行完 Python benchmark 后，可以与 Rust benchmark 结果对比：

```bash
# Rust benchmark (在项目根目录)
cargo bench --bench encoder_comparison

# Python benchmark
python3 benchmark_python.py
```

### 当前参考结果 (macOS Apple Silicon)

| 实现 | 单帧编码 | 完整流程 (1000 frames) |
|------|----------|----------------------|
| **lame-sys (Rust)** | ~50 µs | ~57 ms |
| **mp3lame-encoder (Rust)** | ~39 µs | ~43 ms |
| **lameenc (Python)** | ? | ? |

## 关于 lameenc 库

`lameenc` 是 LAME MP3 编码器的 Python 绑定。它通过 C 扩展直接调用 LAME 库。

- GitHub: https://github.com/chrippa/python-lameenc
- PyPI: https://pypi.org/project/lameenc/

## 注意事项

1. **Python 解释器开销**: Python 的性能会受到解释器和 GIL (Global Interpreter Lock) 的影响
2. **数据转换开销**: Python 和 C 之间的数据转换会增加额外开销
3. **内存管理**: Python 的垃圾回收可能影响性能测量

尽管如此，由于 `lameenc` 使用 C 扩展，实际的编码工作在 native 代码中完成，所以性能应该接近 native 实现。

## 性能优化建议

如果 Python benchmark 性能不理想，可以尝试：

1. **使用 PyPy**: PyPy 是一个更快的 Python 解释器
2. **批量编码**: 一次编码更多数据以减少调用开销
3. **使用 Cython**: 将关键代码编译为 C 扩展

## 故障排除

### lameenc 安装失败

如果 `pip install lameenc` 失败，可能需要安装 LAME 开发库：

**Ubuntu/Debian:**
```bash
sudo apt-get install libmp3lame-dev
pip install lameenc
```

**macOS (Homebrew):**
```bash
brew install lame
pip install lameenc
```

**CentOS/RHEL:**
```bash
sudo yum install lame-devel
pip install lameenc
```

### 找不到 LAME 库

如果运行时报错找不到 LAME 库，确保系统已安装 LAME：

```bash
# 检查 LAME 是否安装
which lame
lame --version
```
