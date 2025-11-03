use std::env;
use std::path::PathBuf;

fn main() {
    // 获取 LAME 源代码路径
    let lame_dir = PathBuf::from("lame");
    let libmp3lame_dir = lame_dir.join("libmp3lame");
    let include_dir = lame_dir.join("include");

    // 1. 使用 cc crate 编译 LAME C 源代码
    println!("cargo:rerun-if-changed=lame/");

    let mut build = cc::Build::new();

    // 添加 include 路径
    build
        .include(&lame_dir)           // 用于 config.h
        .include(&include_dir)        // 用于 lame.h
        .include(&libmp3lame_dir);    // 用于内部头文件

    // 定义编译宏
    build
        .define("HAVE_CONFIG_H", None)
        .define("TAKEHIRO_IEEE754_HACK", None);

    // 添加标准库头文件宏
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        build
            .define("HAVE_LIMITS_H", "1")
            .define("HAVE_ERRNO_H", "1")
            .define("HAVE_FCNTL_H", "1")
            .define("STDC_HEADERS", "1")
            .define("HAVE_STDINT_H", "1");
    }

    // 核心源文件列表（20 个）
    let source_files = [
        "VbrTag.c",
        "bitstream.c",
        "encoder.c",
        "fft.c",
        "gain_analysis.c",
        "id3tag.c",
        "lame.c",
        "newmdct.c",
        "presets.c",
        "psymodel.c",
        "quantize.c",
        "quantize_pvt.c",
        "reservoir.c",
        "set_get.c",
        "tables.c",
        "takehiro.c",
        "util.c",
        "vbrquantize.c",
        "version.c",
        "mpglib_interface.c",
    ];

    // 添加所有源文件
    for file in &source_files {
        build.file(libmp3lame_dir.join(file));
    }

    // 设置编译选项
    build
        .warnings(false)  // 禁用警告（LAME 代码有很多旧风格）
        .opt_level(2);    // 优化级别

    // 编译静态库
    build.compile("mp3lame");

    // 链接数学库
    println!("cargo:rustc-link-lib=m");

    // 2. 使用 bindgen 生成 Rust FFI 绑定
    let bindings = bindgen::Builder::default()
        // 输入头文件
        .header(include_dir.join("lame.h").to_str().unwrap())
        // 添加 clang 参数（include 路径）
        .clang_arg(format!("-I{}", lame_dir.display()))
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", libmp3lame_dir.display()))
        // 只生成需要的函数（编码 + ID3）
        .allowlist_function("lame_.*")
        .allowlist_function("id3tag_.*")
        .allowlist_function("get_lame_.*")
        .allowlist_function("hip_.*")  // 解码器函数（可选）
        // 生成的类型
        .allowlist_type("lame_global_flags")
        .allowlist_type("hip_t")
        // 常量和枚举
        .allowlist_var("MPEG_VERSION_.*")
        .allowlist_var("NOT_SET")
        .allowlist_var("MAX_.*")
        .allowlist_var("MIN_.*")
        // 布局测试
        .layout_tests(true)
        // 生成注释
        .generate_comments(true)
        // Rust 特性
        .use_core()
        .derive_default(true)
        .derive_debug(true)
        // 完成构建
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 写入绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
