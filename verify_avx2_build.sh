#!/bin/bash
# 验证 AVX2 函数是否被正确编译和链接

echo "=== AVX2 Build Verification Script ==="
echo ""

# 1. 检查 vector 源文件是否存在
echo "1. Checking AVX2 source files..."
if [ -f "lame/libmp3lame/vector/avx2_quantize_sub.c" ]; then
    echo "   ✓ avx2_quantize_sub.c exists"
else
    echo "   ✗ avx2_quantize_sub.c NOT FOUND!"
    exit 1
fi

# 2. 清理并重新构建
echo ""
echo "2. Cleaning previous build..."
cargo clean

echo ""
echo "3. Building with verbose output..."
cargo build --release 2>&1 | tee build_verbose.log

# 4. 检查是否生成了 AVX2 目标文件
echo ""
echo "4. Checking for AVX2 object files..."
AVX2_OBJS=$(find target -name "*avx2*.o" -o -name "*avx2*.lo" 2>/dev/null)
if [ -n "$AVX2_OBJS" ]; then
    echo "   ✓ Found AVX2 object files:"
    echo "$AVX2_OBJS" | sed 's/^/     /'
else
    echo "   ✗ No AVX2 object files found!"
fi

# 5. 检查静态库中是否包含 AVX2 符号
echo ""
echo "5. Checking for AVX2 symbols in library..."
LIBMP3LAME=$(find target -name "libmp3lame.a" 2>/dev/null | head -1)
if [ -n "$LIBMP3LAME" ]; then
    echo "   Library: $LIBMP3LAME"
    if nm "$LIBMP3LAME" 2>/dev/null | grep -E "fht_AVX2|init_xrpow_core_avx2" > /dev/null; then
        echo "   ✓ AVX2 symbols found:"
        nm "$LIBMP3LAME" | grep -E "fht_AVX2|init_xrpow_core_avx2" | sed 's/^/     /'
    else
        echo "   ✗ AVX2 symbols NOT found in library!"
        echo "   Available symbols:"
        nm "$LIBMP3LAME" | grep -i "fht\|xrpow" | head -10 | sed 's/^/     /'
    fi
else
    echo "   ✗ libmp3lame.a not found!"
fi

# 6. 检查 lame-sys 编译单元
echo ""
echo "6. Checking lame-sys compilation..."
LAME_SYS_LIB=$(find target -name "liblame_sys*.rlib" 2>/dev/null | head -1)
if [ -n "$LAME_SYS_LIB" ]; then
    echo "   Library: $LAME_SYS_LIB"
    if nm "$LAME_SYS_LIB" 2>/dev/null | grep -E "fht_AVX2|init_xrpow_core_avx2" > /dev/null; then
        echo "   ✓ AVX2 symbols found in lame-sys"
    else
        echo "   ⚠ AVX2 symbols not found in lame-sys (may be in dependency)"
    fi
fi

# 7. 检查构建日志中的关键信息
echo ""
echo "7. Checking build log for vector library..."
if grep -q "WITH_VECTOR" build_verbose.log 2>/dev/null; then
    echo "   Vector configuration:"
    grep "WITH_VECTOR\|WITH_XMM" build_verbose.log | sed 's/^/     /'
fi

if grep -q "avx2_quantize_sub" build_verbose.log 2>/dev/null; then
    echo "   ✓ avx2_quantize_sub.c was compiled"
else
    echo "   ✗ avx2_quantize_sub.c was NOT compiled!"
fi

echo ""
echo "=== Verification Complete ==="
echo ""
echo "If build succeeded, you can test with:"
echo "  cargo test --release"
echo "  cargo bench"
