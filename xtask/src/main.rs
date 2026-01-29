use bootloader::BiosBoot;
use std::path::{Path, PathBuf};

fn main() {
    let target_dir = PathBuf::from("target");

    let kernel_elf = target_dir
        .join("x86_64-blotos")
        .join("debug")
        .join("blotos");

    let out_img = target_dir.join("bios.img");

    let bios = BiosBoot::new(Path::new(&kernel_elf));
    bios.create_disk_image(Path::new(&out_img))
        .expect("échec création image BIOS");
}