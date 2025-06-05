// TODO: this is unfinished and unused, instead the elf2image file is just calling a patched and compiled version of mkimage
// eventually I may want to finish this so I don't have to use the mkimage file

/*
    rust recreation of mkimage. Designed to specifically run the command 
    $ mkimage -n example.dcd -T imximage -e 0x90010000 -d argc[1] argc[2]

    arg1 will have the bin file, arg2 will have the imx path
*/

/*
 * { "config", required_argument, NULL, 'n' } (dcd file to use: example.dcd)
 * { "type", required_argument, NULL, 'T' } (file type to create: imximage)
 * { "entry-point", required_argument, NULL, 'e' } (entry point: 0x90010000)
 * { "image", required_argument, NULL, 'd' } (data file: argc[1].bin)
 */

let dcd_addr = 0x00910000 as u32;
 
pub fn generate_imx(file: &mut File, imx_path: &Path,) -> Result<(), anyhow::Error> {
    /*let params = ImageToolParams {
        os: IH_OS_LINUX,
        arch: IH_ARCH_PPC,
        type_: IH_TYPE_KERNEL,
        comp: IH_COMP_GZIP,
        dtc: "-I dts -O dtb -p 500",
        imagename: "",
        imagename2: "",
    };*/
    //{	IH_TYPE_IMXIMAGE,   "imximage",   "Freescale i.MX Boot Image",}
    
    println!(file.to_str().unwrap());

    let dfd = "example.dcd"
    let dflag = 1;
    let ep = 0x90010000;
    let eflag = 1
    let imagename = file.to_str().unwrap();
    let imagefile = File::create(imx_path);

	let datafile = std::fs::File::open(dfd)?;

	let header_size = 760;
	let file_size = datafile.metadata()?.len() + header_size;

    // Parse dcd


    let imximage_init_loadsize = 0;
    let imximage_ivt_offset = 0xffffffff;
    if (imximage_init_loadsize < imximage_ivt_offset + header_size) {
        let imximage_init_loadsize = imximage_ivt_offset + header_size;
    }
    let alloc_len = imximage_init_loadsize - imximage_ivt_offset;
    
    Ok(())
}