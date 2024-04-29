use std::{env, path::PathBuf};
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| {
        println!("cargo:error=You must provide a target OS; either nintendo_ds_arm9 or nintendo_ds_arm7 is supported.");
        "error".to_string()
    });
    let wonderful_path = env::var("WONDERFUL_TOOLCHAIN").unwrap_or(String::from("/opt/wonderful"));
    let blocksds_path =
        env::var("BLOCKSDS").unwrap_or(String::from("/opt/wonderful/thirdparty/blocksds/core"));
    let profile = env::var("PROFILE").unwrap();
   
    println!("cargo:rustc-link-search=native={blocksds_path}/libs/libnds/lib");
    println!("cargo:rustc-link-search=native={blocksds_path}/libs/dswifi/lib");
    println!("cargo:rustc-link-search=native={blocksds_path}/libs/maxmod/lib");

    println!(
        "cargo:rustc-link-search=native={wonderful_path}/toolchain/gcc-arm-none-eabi/lib/gcc/arm-none-eabi/13.2.0/thumb"
    );
   
    match target_os.as_str() {
        "nintendo_ds_arm9" => arm9_main(&wonderful_path, &blocksds_path, &profile),
        "nintendo_ds_arm7" => arm7_main(&wonderful_path, &blocksds_path, &profile),
        "error" => {}
        _ => {
            println!("cargo:error=Target OS is not valid! please use the appropriate nintendo DS target json included with this library!")
        }
    }
}

fn arm7_main(wonderful_path: &str, blocksds_path: &str, profile: &str) {
    println!(
        "cargo:rustc-link-search=native={wonderful_path}/toolchain/gcc-arm-none-eabi/arm-none-eabi/lib/thumb/arm7tdmi"
    );
    println!("cargo:rustc-link-arg=Wl,--start-group");
    println!(
        "cargo:rustc-link-lib=static={}",
        match profile {
            "debug" => "nds7d",
            _ => "nds7",
        }
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match profile {
            "debug" => "dswifi7d",
            _ => "dswifi7",
        }
    );
    println!("cargo:rustc-link-lib=static=mm7");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-arg=Wl,--end-group");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!(
            "-isystem{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include",
            wonderful_path
        ))
        .clang_arg("-DARM7")
        .clang_arg("-D__BLOCKSDS__")
        .clang_arg("-D__NDS__")
        .clang_arg(format!("-I{}/libs/libnds/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/dswifi/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/maxmod/include", blocksds_path))
        .header(format!("{}/libs/libnds/include/nds/interrupts.h", blocksds_path))
        .header(format!("{}/libs/libnds/include/nds.h", blocksds_path))
        .wrap_static_fns(true)
        .wrap_static_fns_path("src/arm7_bindings.c")
        .use_core()
        .trust_clang_mangling(false)
        .prepend_enum_name(false)
        .clang_arg("-mfloat-abi=soft")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(".")
        .wrap_unsafe_ops(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/arm7_bindings.rs")
        .expect("Couldn't write bindings!");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let maxmod_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!(
            "-isystem{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include",
            wonderful_path
        ))
        .clang_arg("-DARM7")
        .clang_arg("-D__BLOCKSDS__")
        .clang_arg("-D__NDS__")
        .clang_arg(format!("-I{}/libs/libnds/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/dswifi/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/maxmod/include", blocksds_path))
        .header(format!("{}/libs/maxmod/include/maxmod7.h", blocksds_path))
        .wrap_static_fns(true)
        .wrap_static_fns_path("src/maxmod7.c")
        .use_core()
        .trust_clang_mangling(false)
        .prepend_enum_name(false)
        .clang_arg("-mfloat-abi=soft")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(".")
        .wrap_unsafe_ops(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    maxmod_bindings
        .write_to_file("src/maxmod7.rs")
        .expect("Couldn't write bindings!");

    let wifi_bindings = bindgen::Builder::default()
    .clang_arg(format!(
        "-isystem{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include",
        wonderful_path
    ))
    .clang_arg("-DARM7")
    .clang_arg("-D__BLOCKSDS__")
    .clang_arg("-D__NDS__")
    .clang_arg(format!("-I{}/libs/libnds/include", blocksds_path))
    .clang_arg(format!("-I{}/libs/dswifi/include", blocksds_path))
    .clang_arg(format!("-I{}/libs/maxmod/include", blocksds_path))
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/libs/dswifi/include/dswifi7.h", blocksds_path))
        .wrap_static_fns(true)
        .wrap_static_fns_path("src/dswifi7.c")
        .use_core()
        .trust_clang_mangling(false)
        .prepend_enum_name(false)
        .clang_arg("-mfloat-abi=soft")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(".")
        .wrap_unsafe_ops(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    wifi_bindings
        .write_to_file("src/dswifi7.rs")
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .object(format!("{}/sys/crts/ds_arm7_crt0.o", &blocksds_path))
        .object(format!("{}/sys/crts/ds_arm7_iwram_crt0.o", &blocksds_path))
        .object(format!("{}/sys/crts/ds_arm7_vram_crt0.o", &blocksds_path))
        .file("src/arm7_bindings.c")
        .file("src/maxmod7.c")
  //      .file("src/dswifi7.c")
        .compiler(format!(
            "{}/toolchain/gcc-arm-none-eabi/bin/arm-none-eabi-gcc",
            wonderful_path
        ))
        .include(env::var("CARGO_MANIFEST_DIR").unwrap())
        .include(format!("{}/libs/libnds/include", blocksds_path))
        .include(format!("{}/libs/dswifi/include", blocksds_path))
        .include(format!("{}/libs/maxmod/include", blocksds_path))
        .include(format!(
            "{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include/",
            wonderful_path
        ))
        .no_default_flags(true)
        .define("ARM7", "1")
        .define("__BLOCKSDS__","1")
        .define("__NDS__","1")
        .flag(&format!(
            "-include{}/wrapper.h",
            env::current_dir().unwrap().display()
        ))
        .flag("-g")
        .flag("-Wall")
        .flag("-O3")
        .flag("-mcpu=arm7tdmi")
        .flag("-mtune=arm7tdmi")
        .flag("-fomit-frame-pointer")
        .flag("-ffast-math")
        .flag("-w")
        .compile("bindings");
}

fn arm9_main(wonderful_path: &str, blocksds_path: &str, profile: &str) {
    println!(
        "cargo:rustc-link-search=native={wonderful_path}/toolchain/gcc-arm-none-eabi/arm-none-eabi/lib/thumb/arm946e-s"
    );
    println!("cargo:rustc-link-arg=Wl,--start-group");
    println!(
        "cargo:rustc-link-lib=static={}",
        match profile {
            "debug" => "nds9d",
            _ => "nds9",
        }
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match profile {
            "debug" => "dswifi9d",
            _ => "dswifi9",
        }
    );
    println!("cargo:rustc-link-lib=static=mm9");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-arg=Wl,--end-group");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!("-I{}/libs/libnds/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/dswifi/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/maxmod/include", blocksds_path))
        .clang_arg(format!(
            "-isystem{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include",
            wonderful_path
        ))
        .header(format!("{}/libs/libnds/include/nds.h", blocksds_path))
        .header(format!("{}/libs/dswifi/include/dswifi9.h", blocksds_path))
        .wrap_static_fns(true)
        .wrap_static_fns_path("src/arm9_bindings.c")
        .use_core()
        .trust_clang_mangling(false)
        .prepend_enum_name(false)
        .clang_arg("-mfloat-abi=soft")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("-DARM9")
        .clang_arg("-D__NDS__")
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(".")
        .wrap_unsafe_ops(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/arm9_bindings.rs")
        .expect("Couldn't write bindings!");
    let maxmod_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_arg(format!("-I{}/libs/libnds/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/dswifi/include", blocksds_path))
        .clang_arg(format!("-I{}/libs/maxmod/include", blocksds_path))
        .clang_arg(format!(
            "-isystem{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include",
            wonderful_path
        ))
        .header(format!("{}/libs/maxmod/include/maxmod9.h", blocksds_path))
        .wrap_static_fns(true)
        .wrap_static_fns_path("src/maxmod9.c")
        .use_core()
        .trust_clang_mangling(false)
        .prepend_enum_name(false)
        .clang_arg("-mfloat-abi=soft")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("-DARM9")
        .clang_arg("-D__NDS__")
        .clang_macro_fallback()
        .clang_macro_fallback_build_dir(".")
        .wrap_unsafe_ops(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    maxmod_bindings
        .write_to_file("src/maxmod9.rs")
        .expect("Couldn't write bindings!");

        cc::Build::new()
        .include(env::var("CARGO_MANIFEST_DIR").unwrap())
        .include(format!("{}/libs/libnds/include", blocksds_path))
        .include(format!("{}/libs/dswifi/include", blocksds_path))
        .include(format!("{}/libs/maxmod/include", blocksds_path))
        .include(format!(
            "{}/toolchain/gcc-arm-none-eabi/arm-none-eabi/include/",
            wonderful_path
        ))
            .object(format!("{}/sys/crts/ds_arm9_crt0.o", &blocksds_path))
            .file("src/arm9_bindings.c")
            .file("src/maxmod9.c")
            .compiler(format!(
                "{}/toolchain/gcc-arm-none-eabi/bin/arm-none-eabi-gcc",
                wonderful_path
            ))
            .no_default_flags(true)
            .define("ARM9", "1")
            .define("__BLOCKSDS__", "1")
            .define("__NDS__", "1")
            .flag(&format!(
                "-include{}/wrapper.h",
                env::current_dir().unwrap().display()
            ))
            .flag("-march=armv5te")
            .flag("-mfloat-abi=soft")
            .flag("-mtune=arm946e-s")
            .flag("-mthumb")
            .flag("-mthumb-interwork")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-fomit-frame-pointer")
            .flag("-w")
            .compile("bindings");
}
