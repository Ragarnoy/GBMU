use std::env;
use std::fs;
use std::io;
use std::path::Path;

use zip::ZipArchive;

fn main() {
    let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("..").join("assets").join("bios");
    let src_path = dst_path.clone().with_extension("zip");

    if let Ok(file) = fs::File::open(&src_path) {
        if let Ok(mut archive) = ZipArchive::new(file) {
            fs::create_dir_all(&dst_path).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath = match file.enclosed_name() {
                    Some(path) => dst_path.clone().join(path),
                    None => continue,
                };
                if (&*file.name()).ends_with('/') {
                    fs::create_dir_all(&outpath).unwrap();
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                    }
                }
            }
        }
    }
    println!("cargo:rerun-if-changed=assets/bios.zip");
}
