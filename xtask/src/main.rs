use bootloader::{DiskImageBuilder};
use std::path::{PathBuf};

fn main() {
    let target_dir = PathBuf::from("target");

    let kernel_elf = target_dir
        .join("x86_64-blotos")
        .join("debug")
        .join("blotos");
    let out_img = target_dir.join("bios.img");

    DiskImageBuilder::new(kernel_elf)
        .create_bios_image(&out_img)
        .unwrap();
}