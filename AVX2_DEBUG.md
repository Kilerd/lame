# AVX2 调试指南

本文档帮助你检查和验证 AVX2 优化是否在 LAME 中正确启用。

## 快速检查

在项目根目录运行：

```bash
./check_avx2.sh
```

这个脚本会检查：
1. CPU 是否支持 AVX2
2. 编译时是否检测到 AVX2 头文件
3. 编译后的库中是否包含 AVX2 指令
4. AVX2 源文件是否被编译

## 手动检查步骤

### 1. 检查 CPU 支持

在 Linux 上：
```bash
# 方法 1: 使用 lscpu
lscpu | grep -i avx

# 方法 2: 使用 /proc/cpuinfo
cat /proc/cpuinfo | grep avx2

# 方法 3: 使用 cpuid (需要安装 cpuid 工具)
cpuid | grep AVX2
```

如果看到 `avx2` 标志，说明 CPU 支持 AVX2。

### 2. 检查编译配置

查看 autotools 生成的 config.h：

```bash
find target -name "config.h" | xargs grep -E "HAVE_IMMINTRIN_H|HAVE_XMMINTRIN_H"
```

应该看到：
```c
#define HAVE_IMMINTRIN_H 1   // AVX2 支持
#define HAVE_XMMINTRIN_H 1   // SSE 支持
```

如果没有定义 `HAVE_IMMINTRIN_H`，说明编译时没有检测到 AVX2 头文件。

### 3. 检查编译输出

重新编译并查看详细输出：

```bash
cargo clean
RUST_LOG=debug cargo build --release 2>&1 | tee build.log

# 搜索 AVX2 相关信息
grep -i "avx\|immintrin" build.log
```

### 4. 检查目标文件

查看是否生成了 AVX2 目标文件：

```bash
find target -name "*avx2*.o" -o -name "*avx2*.lo"
```

应该找到 `avx2_quantize_sub.o` 或类似文件。

### 5. 检查汇编指令

使用 objdump 检查是否包含 AVX2 指令：

```bash
find target -name "libmp3lame.a" | xargs objdump -d | grep -E "vfmadd|vperm|vbroadcast|vgather" | head -20
```

AVX2 指令特征：
- 以 `v` 开头（如 `vfmadd`, `vmulps`, `vaddps`）
- 使用 `ymm` 寄存器（如 `ymm0`, `ymm1`，256位）
- SSE 使用 `xmm` 寄存器（128位）

### 6. 运行时检查

修改代码添加调试输出。编辑 `lame/libmp3lame/lame.c`:

在 CPU 特性检测后添加：
```c
/* Detect AVX/AVX2/FMA support */
gfc->CPU_features.AVX = has_AVX();
gfc->CPU_features.AVX2 = has_AVX2();
gfc->CPU_features.FMA = has_FMA();
gfc->CPU_features.AVX512F = has_AVX512F();

/* 添加这几行 */
fprintf(stderr, "DEBUG: AVX=%d, AVX2=%d, FMA=%d, AVX512F=%d\n",
    gfc->CPU_features.AVX,
    gfc->CPU_features.AVX2,
    gfc->CPU_features.FMA,
    gfc->CPU_features.AVX512F);
```

然后重新编译并运行 benchmark：
```bash
cargo build --release
cargo bench 2>&1 | grep -i "DEBUG\|CPU features"
```

## 常见问题

### Q1: CPU 支持 AVX2 但编译时未检测到

**原因**: 编译器可能没有正确配置

**解决方案**:
```bash
# 设置 CFLAGS 明确启用 AVX2
export CFLAGS="-mavx2 -mfma"
cargo clean
cargo build --release
```

### Q2: 编译检测到 AVX2 但运行时未使用

**原因**: 可能是函数派发逻辑问题

**调试步骤**:
1. 检查 `lame/libmp3lame/quantize.c` 的 `init_xrpow_core_init()` 函数
2. 确认条件编译正确：
```c
#if defined(HAVE_IMMINTRIN_H)
    if (gfc->CPU_features.AVX2)
        gfc->init_xrpow_core = init_xrpow_core_avx2;
#endif
```

3. 添加调试输出验证：
```c
#if defined(HAVE_IMMINTRIN_H)
    if (gfc->CPU_features.AVX2) {
        fprintf(stderr, "DEBUG: Using AVX2 quantization\n");
        gfc->init_xrpow_core = init_xrpow_core_avx2;
    }
#endif
```

### Q3: 链接错误 - undefined symbol: fht_AVX2 / init_xrpow_core_avx2

**原因**: vector 库没有被正确构建，导致 AVX2 函数未被编译

**症状**:
```
rust-lld: error: undefined symbol: fht_AVX2
rust-lld: error: undefined symbol: init_xrpow_core_avx2
```

**根本原因**:
- `build.rs` 中强制定义了 `HAVE_IMMINTRIN_H=1`，使代码引用 AVX2 函数
- 但 configure 没有检测到 SSE/AVX2 支持，导致 `WITH_XMM=no`
- 结果 `liblamevectorroutines.la` 库没有被构建
- 链接时找不到 AVX2 函数定义

**解决方案**:

已在 `build.rs` 中添加环境变量强制启用 vector 支持：

```rust
// 确保 configure 能正确检测到 SSE/AVX2 支持
std::env::set_var("ac_cv_header_xmmintrin_h", "yes");
std::env::set_var("ac_cv_header_immintrin_h", "yes");
```

**验证修复**:
```bash
# 清理并重新构建
cargo clean
cargo build --release

# 或使用验证脚本
./verify_avx2_build.sh
```

### Q4: 链接错误 - undefined reference to `has_AVX2_nasm`

**原因**: NASM 代码没有正确编译

**解决方案**:
```bash
# 检查 NASM 是否安装
nasm -version

# 如果没有安装
sudo apt-get install nasm  # Ubuntu/Debian
sudo yum install nasm      # CentOS/RHEL

# 重新配置和编译
cd lame
./configure --enable-nasm
make clean && make
```

### Q5: 性能没有提升

**可能原因**:

1. **AVX2 未启用** - 使用上面的方法检查
2. **编译优化级别不够** - 确保使用 `-O3` 或 `--release`
3. **测试数据量太小** - AVX2 在处理大量数据时效果更明显
4. **其他瓶颈** - AVX2 只优化了量化和 FFT，其他部分可能仍是瓶颈

**验证方法**:
```bash
# 使用 perf 分析热点
perf record -g cargo bench
perf report
```

## 预期性能提升

如果 AVX2 正确启用，你应该看到：

- **单帧编码**: 15-25% 提升
- **完整编码流程**: 40-60% 提升（对于长音频）
- **CPU features 输出**: 应显示 "AVX2 (ASM used)"

## 进一步调试

如果以上方法都无法解决问题，可以：

1. **查看 configure.log**:
```bash
cat lame/config.log | grep -A 10 "checking working AVX2"
```

2. **手动测试 AVX2 编译**:
```bash
cat > test_avx2.c << 'EOF'
#include <immintrin.h>
#include <stdio.h>

int main() {
    __m256 a = _mm256_set1_ps(1.0f);
    __m256 b = _mm256_set1_ps(2.0f);
    __m256 c = _mm256_add_ps(a, b);
    float result[8];
    _mm256_storeu_ps(result, c);
    printf("AVX2 test: %f\n", result[0]);
    return 0;
}
EOF

gcc -mavx2 test_avx2.c -o test_avx2
./test_avx2
```

3. **提供完整的诊断信息**:
```bash
# 收集所有信息
{
    echo "=== CPU Info ==="
    cat /proc/cpuinfo | grep "model name\|flags" | head -2

    echo -e "\n=== Compiler Info ==="
    gcc --version | head -1

    echo -e "\n=== Build Config ==="
    find target -name "config.h" | head -1 | xargs grep -E "HAVE_IMMINTRIN_H|HAVE_NASM"

    echo -e "\n=== AVX2 Objects ==="
    find target -name "*avx2*"

    echo -e "\n=== Benchmark Result ==="
    cargo bench 2>&1 | grep -E "time:|change:"
} > avx2_diagnostic.txt

cat avx2_diagnostic.txt
```

## 联系支持

如果问题仍然存在，请提供：
- `avx2_diagnostic.txt` 文件内容
- `./check_avx2.sh` 的完整输出
- CPU 型号和操作系统版本
