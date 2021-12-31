use std::env;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use zip::read::{ZipArchive, ZipFile};

fn main() {
    let dst_path = get_destination_path();
    let archive = get_archive_path();

    fs::create_dir_all(&dst_path).unwrap();
    if let Ok(file) = fs::File::open(&archive) {
        unzip_file(dst_path, file);
    }
    println!(
        "cargo:rerun-if-changed={}",
        archive.to_str().expect("cannot create a string from path")
    );
}

fn get_destination_path() -> PathBuf {
    let out_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&out_dir).join("..").join("assets").join("bios")
}

fn get_archive_path() -> PathBuf {
    let dst_path = get_destination_path();
    dst_path.with_extension("zip")
}

fn unzip_file(dst_path: PathBuf, file: fs::File) {
    if let Ok(mut archive) = ZipArchive::new(file) {
        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            extract_file(dst_path.clone(), file)
        }
    }
}

/// Extract a file that is contain is an archive
fn extract_file(output_dir: PathBuf, mut file: ZipFile) {
    let dest_path = archive_file_output_path(&output_dir, &file).unwrap();
    if file.is_dir() {
        fs::create_dir_all(&dest_path).unwrap();
    } else {
        may_create_parent_subdir(&dest_path);
        let mut dest_file = create_destination_file(&dest_path, file.unix_mode());
        io::copy(&mut file, &mut dest_file).unwrap();
    }
}

/// Create the output path for the file that is contain is the archive
fn archive_file_output_path(output_dir: &PathBuf, file: &ZipFile) -> Result<PathBuf, String> {
    file.enclosed_name()
        .ok_or_else(|| format!("cannot get enclosed for {}", file.name()))
        .map(|name| output_dir.clone().join(name))
}

fn may_create_parent_subdir(path: &PathBuf) {
    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}

fn create_destination_file(path: &PathBuf, mode: Option<u32>) -> File {
    let file = File::create(path).unwrap();
    #[cfg(unix)]
    {
        use fs::Permissions;
        use std::os::unix::fs::PermissionsExt;
        if let Some(mode) = mode {
            file.set_permissions(Permissions::from_mode(mode))
                .expect("cannot set file Permissions");
        }
    }
    file
}
