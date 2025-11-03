#!/bin/bash

echo "=========================================="
echo "AVX2 Support Detection Script"
echo "=========================================="
echo ""

# 1. Check CPU support
echo "1. Checking CPU AVX2 support:"
if command -v lscpu &> /dev/null; then
    echo "Using lscpu:"
    lscpu | grep -i avx
elif [ -f /proc/cpuinfo ]; then
    echo "Using /proc/cpuinfo:"
    grep -i avx /proc/cpuinfo | head -1
else
    echo "Cannot detect CPU features on this system"
fi
echo ""

# 2. Check if HAVE_IMMINTRIN_H was defined during build
echo "2. Checking build configuration:"
CONFIG_H=$(find target -name "config.h" -type f 2>/dev/null | head -1)
if [ -n "$CONFIG_H" ]; then
    echo "Found config.h: $CONFIG_H"
    echo "HAVE_IMMINTRIN_H:"
    grep "HAVE_IMMINTRIN_H" "$CONFIG_H" || echo "  Not defined"
    echo "HAVE_XMMINTRIN_H:"
    grep "HAVE_XMMINTRIN_H" "$CONFIG_H" || echo "  Not defined"
    echo "HAVE_NASM:"
    grep "HAVE_NASM" "$CONFIG_H" || echo "  Not defined"
else
    echo "config.h not found in target directory"
    echo "Try building first: cargo build --release"
fi
echo ""

# 3. Check for AVX2 instructions in compiled library
echo "3. Checking for AVX2 instructions in compiled code:"
LIBMP3LAME=$(find target -name "libmp3lame.a" -type f 2>/dev/null | head -1)
if [ -n "$LIBMP3LAME" ]; then
    echo "Found libmp3lame.a: $LIBMP3LAME"
    if command -v objdump &> /dev/null; then
        echo "Searching for AVX2 instructions (vfmadd, vbroadcast, etc.):"
        objdump -d "$LIBMP3LAME" 2>/dev/null | grep -E "vfmadd|vperm|vbroadcast|vgather" | head -5
        if [ $? -eq 0 ]; then
            echo "  ✓ AVX2 instructions found!"
        else
            echo "  ✗ No AVX2 instructions found"
        fi
    else
        echo "objdump not available, cannot check for AVX2 instructions"
    fi
else
    echo "libmp3lame.a not found"
fi
echo ""

# 4. Check for avx2_quantize_sub.o
echo "4. Checking if AVX2 source files were compiled:"
AVX2_OBJ=$(find target -name "*avx2*.o" -o -name "*avx2*.lo" 2>/dev/null)
if [ -n "$AVX2_OBJ" ]; then
    echo "Found AVX2 object files:"
    echo "$AVX2_OBJ"
else
    echo "No AVX2 object files found"
fi
echo ""

echo "=========================================="
echo "Recommendations:"
echo "=========================================="
echo "If AVX2 is not detected:"
echo "1. Ensure your CPU supports AVX2: cat /proc/cpuinfo | grep avx2"
echo "2. Rebuild with: cargo clean && cargo build --release"
echo "3. Check compiler flags in CFLAGS: echo \$CFLAGS"
echo "4. Try adding: CFLAGS='-mavx2 -mfma' cargo build --release"
echo ""
