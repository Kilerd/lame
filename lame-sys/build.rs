use std::env;
use std::path::PathBuf;

fn main() {
    // 获取 LAME 源代码路径
    let lame_dir = PathBuf::from("lame");
    let include_dir = lame_dir.join("include");

    // 1. 使用 autotools 构建 LAME
    println!("cargo:rerun-if-changed=lame/");

    // 使用最简化配置（完全模仿竞品 mp3lame-sys）
    // 测试假设：手动添加的优化标志可能反而降低性能

    let dst = autotools::Config::new(&lame_dir)
        .disable_shared()
        .enable_static()
        .env(
            "CFLAGS",
            "-O3 -ffunction-sections -fdata-sections -fPIC -m64
        -Wno-error=incompatible-pointer-types -march=native -ffast-math -fomit-frame-pointer",
        )
        .disable("rpath", None)
        .disable("frontend", None)
        .disable("decoder", None)
        .disable("gtktest", None)
        .with("pic", None)
        .fast_build(true)
        .build();

    // 链接生成的静态库
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=mp3lame");

    // 链接数学库
    println!("cargo:rustc-link-lib=m");

    // 2. 使用 bindgen 生成 Rust FFI 绑定
    let bindings = bindgen::Builder::default()
        // 输入头文件
        .header(include_dir.join("lame.h").to_str().unwrap())
        // 添加 clang 参数（include 路径）
        .clang_arg(format!("-I{}", lame_dir.display()))
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}/include", dst.display()))
        // 只生成需要的函数（编码 + ID3）
        .allowlist_function("lame_.*")
        .allowlist_function("id3tag_.*")
        .allowlist_function("get_lame_.*")
        .allowlist_function("hip_.*") // 解码器函数（可选）
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
