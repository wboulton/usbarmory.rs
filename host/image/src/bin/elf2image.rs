//! Converts an ELF into an image suitable for flashing into the eMMC

use std::{
    env,
    fs::{self, File},
    path::Path,
    process::Command,
};
use std::os::raw::{c_char, c_int};
use anyhow::{bail, format_err};
use image::write::Image;
use xmas_elf::ElfFile;

fn main() -> Result<(), anyhow::Error> {
    // NOTE(skip) program name
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        bail!("expected exactly one argument");
    }

    let path = Path::new(&args[0]);
    let stem =
        Path::new(path.file_stem().ok_or_else(|| {
            format_err!("unable to determine the file stem of {}", path.display())
        })?);
    let bytes = fs::read(path)?;
    let elf = ElfFile::new(&bytes).map_err(|s| format_err!("{}", s))?;
    // we always need to initialize the DDR when cold booting from the eMMC
    let skip_dcd = true;
    let image = Image::from_elf(&elf, skip_dcd)?;
    let file = &mut File::create(stem.with_extension("bin"))?;
    image.write(file)?;
    let bin_path = stem.with_extension("bin");
    let imx_path = stem.with_extension("imx");

    /*
        The documentation for the imx file format is completely non-existant, the only way to fix the write.rs file would be
        to either figure out exactly what is wrong, which would require REing the file format based on the mkimage library.
        Instead I have implemented this temporary solution by calling the mkimage tool and eventually I will just completely
        rewrite write.rs by porting the mkimage tool to rust. Or I may try to link the mkimage tool in as a library, but I tried
        that and had many issues with files being linked in for the wrong architecture. 
    */
    let mkimage_path = "./mkimage"; // adjust this path as needed

    let status = Command::new(mkimage_path)
        .arg("-n")
        .arg("example.dcd")
        .arg("-T")
        .arg("imximage")
        .arg("-e")
        .arg("0x90010000")
        .arg("-d")
        .arg(bin_path.as_os_str())
        .arg(imx_path.as_os_str())
        .status()?;

    //let result = Imx::generate_imx(file, imx_path);

    Ok(())
}
