use std::env;
use std::path::PathBuf;

const WIRINGX: &str = "duo-wiringx-1.0.3";

fn main() {
    println!("cargo:rerun-if-changed={}", WIRINGX);

    let include_dirs = [
        "",
        "platform/",
        "platform/linksprite/",
        "platform/lemaker/",
        "platform/solidrun/",
        "platform/raspberrypi/",
        "platform/hardkernel/",
        "platform/xunlong/",
        "platform/radxa/",
        "platform/milkv/",
        "soc/",
        "soc/allwinner/",
        "soc/nxp/",
        "soc/broadcom/",
        "soc/amlogic/",
        "soc/samsung/",
        "soc/rockchip/",
        "soc/sophgo/",
    ]
    .map(|path| WIRINGX.to_string() + "/src/" + path);

    let source_files = [
        "*.c",
        "platform/*.c",
        "platform/linksprite/*.c",
        "platform/lemaker/*.c",
        "platform/solidrun/*.c",
        "platform/raspberrypi/*.c",
        "platform/hardkernel/*.c",
        "platform/xunlong/*.c",
        "platform/radxa/*.c",
        "platform/milkv/*.c",
        "soc/*.c",
        "soc/allwinner/*.c",
        "soc/nxp/*.c",
        "soc/broadcom/*.c",
        "soc/amlogic/*.c",
        "soc/samsung/*.c",
        "soc/rockchip/*.c",
        "soc/sophgo/*.c",
    ]
    .map(|path| WIRINGX.to_string() + "/src/" + path);

    let mut build = cc::Build::new();
    build.files(source_files.iter().flat_map(|pattern| {
        glob::glob(pattern)
            .expect("Failed to read glob pattern")
            .map(|entry| entry.unwrap())
    }));

    for dir in include_dirs {
        build.include(dir);
    }

    build.flag("-Wl,-rpath=/usr/local/lib/");
    build.flag("-Wl,-rpath=/usr/lib/");
    build.flag("-Wl,-rpath=/lib/");
    build.flag("-Wno-int-conversion");

    build.flag_if_supported("-w");

    build.compile("wiringx");

    let bindings = bindgen::Builder::default()
        .header(WIRINGX.to_string() + "/src/wiringx.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
