# GitHub Actions 构建和发布说明

## 自动构建流程

本项目配置了 GitHub Actions，可以自动编译 Linux x86_64 平台的 `libmp3lame.so` 共享库。

## 触发方式

### 方式 1: 创建 Git Tag（自动发布）

当你推送以 `v` 开头的 tag 时，会自动触发构建并创建 GitHub Release：

```bash
# 1. 确保本地代码已提交
git add .
git commit -m "your commit message"

# 2. 创建版本标签
git tag v3.101.0

# 3. 推送代码和标签
git push origin main
git push origin v3.101.0
```

**自动操作：**
- ✅ 自动编译 `libmp3lame.so`
- ✅ 自动打包构建产物
- ✅ 自动创建 GitHub Release
- ✅ 自动上传文件到 Release

### 方式 2: 手动触发（测试构建）

在不创建 Release 的情况下测试构建：

1. 访问 GitHub 仓库的 **Actions** 标签页
2. 选择 "Build and Release LAME" workflow
3. 点击右上角 **"Run workflow"** 按钮
4. 选择分支（通常是 main）
5. 点击 **"Run workflow"** 确认

**手动触发时：**
- ✅ 编译构建
- ✅ 上传 Artifacts（保留 90 天）
- ❌ 不创建 GitHub Release

## 构建配置

### 平台和架构
- **操作系统**: Ubuntu 22.04
- **架构**: x86_64 (amd64)

### 编译选项
```bash
./configure \
  --prefix=/usr \
  --disable-decoder \
  --disable-analyzer-hooks \
  --disable-static \
  --enable-shared \
  --enable-nasm
```

### 特性
- ✅ 仅编码功能（不包含解码器）
- ✅ 禁用分析器钩子
- ✅ 仅构建共享库
- ✅ 启用 NASM 汇编优化
- ✅ Strip 调试符号

## 构建产物

每次成功构建会生成以下文件：

### 文件结构
```
libmp3lame-{version}-linux-x86_64.tar.gz
└── libmp3lame-{version}-linux-x86_64/
    ├── README.md                    # 使用说明
    ├── lib/
    │   ├── libmp3lame.so.0.0.0     # 实际库文件
    │   ├── libmp3lame.so.0         # soname 符号链接
    │   └── libmp3lame.so           # 开发符号链接
    └── include/
        └── lame.h                   # 公共 API 头文件
```

### 下载位置

- **Tag 触发**: 在仓库的 [Releases](../../releases) 页面下载
- **手动触发**: 在 Actions 运行详情页面的 Artifacts 部分下载

## 安装使用

### 1. 下载并解压

```bash
# 从 GitHub Releases 下载
wget https://github.com/your-username/your-repo/releases/download/v3.101.0/libmp3lame-3.101.0-linux-x86_64.tar.gz

# 解压
tar -xzf libmp3lame-3.101.0-linux-x86_64.tar.gz
cd libmp3lame-3.101.0-linux-x86_64
```

### 2. 安装到系统

```bash
# 安装库文件和头文件
sudo cp lib/libmp3lame.so* /usr/local/lib/
sudo cp include/lame.h /usr/local/include/

# 更新系统库缓存
sudo ldconfig
```

### 3. 验证安装

```bash
# 检查库是否正确安装
ldconfig -p | grep libmp3lame

# 应该看到类似输出：
# libmp3lame.so.0 (libc6,x86-64) => /usr/local/lib/libmp3lame.so.0
```

### 4. 编译你的程序

```bash
# 编译时链接 libmp3lame
gcc your_program.c -o your_program -lmp3lame -lm

# 或使用 pkg-config（如果配置了 .pc 文件）
gcc your_program.c -o your_program $(pkg-config --cflags --libs libmp3lame)
```

## 示例代码

```c
#include <stdio.h>
#include <lame.h>

int main() {
    lame_t lame = lame_init();

    // 配置编码器
    lame_set_num_channels(lame, 2);
    lame_set_in_samplerate(lame, 44100);
    lame_set_brate(lame, 128);
    lame_set_quality(lame, 2);

    if (lame_init_params(lame) < 0) {
        fprintf(stderr, "Failed to initialize LAME\n");
        return 1;
    }

    printf("LAME initialized successfully!\n");
    printf("Version: %s\n", get_lame_version());

    // ... 编码代码 ...

    lame_close(lame);
    return 0;
}
```

## 故障排除

### 找不到共享库

如果运行程序时出现 "error while loading shared libraries: libmp3lame.so.0"：

```bash
# 方法 1: 更新 ldconfig
sudo ldconfig

# 方法 2: 设置 LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# 方法 3: 添加到系统配置
echo "/usr/local/lib" | sudo tee /etc/ld.so.conf.d/local.conf
sudo ldconfig
```

### 查看 GitHub Actions 日志

如果构建失败：

1. 访问仓库的 **Actions** 标签页
2. 点击失败的 workflow 运行
3. 展开失败的步骤查看详细日志
4. 检查编译错误或配置问题

## 版本管理建议

使用语义化版本号（Semantic Versioning）：

- `v3.101.0` - 稳定发布版本
- `v3.101.1` - Bug 修复版本
- `v3.102.0` - 新功能版本

## 注意事项

1. **只推送稳定的 tag**: 因为 tag 会自动创建 Release，请确保代码已经测试通过
2. **Tag 不可修改**: 一旦推送的 tag 创建了 Release，避免删除或修改 tag
3. **测试先用手动触发**: 在创建正式 tag 之前，可以先手动触发测试构建
4. **保留原始产物**: GitHub Artifacts 会保留 90 天，Release 文件永久保留

## 许可证

构建产物遵循 LGPL v2 许可证，与 LAME 源代码保持一致。
