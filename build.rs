use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let c_src = [
        "rpi-ws281x-smi/smileds.c",
        "rpi-ws281x-smi/smi/rpi_dma_utils.c",
        // "rpi-ws281x-smi/smi/rpi_pixleds.c",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .target("armv7-unknown-linux-musleabihf")
        .compiler("arm-linux-musleabihf-gcc")
        .define("PHYS_REG_BASE", "0xFE000000")
        .files(c_src.iter())
        .include("rpi-ws281x-smi/smi");

    build.compile("smileds");
}
