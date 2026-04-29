use std::{
    env,
    path::{Path, PathBuf},
    io::{self,Read,Cursor},
    process::Command,
};
use git2::Repository;
use log::{info,warn,error};

pub struct Package{
    pub flake: &str,
    pub attr_name: &str
}

pub fn parse_pkg(pkg_name: &str, default_flake: &str )-> Package{
    match pkg_name.split_once('#'){
        Some((flake, attr)) => Package {
            flake: flake.to_string(),
            attr_name: attr.to_string()
        },
        None => Package {
            flake: default_flake.to_string(),
            attr_name: default_flake.to_string()
        }
    }
}

pub fn ensure_np_dir(playground_dir : &Path) -> io::Result<PathBuf>{
    if !playground_dir.exists() {
        error!("No checkout found in the current folder");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Missing Dir"));
    }
    Ok(playground_dir.to_path_buf())
}

pub struct CwdGuard {
    original : PathBuf,
}

impl CwdGuard {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let original  = env::current_dir()?;
        env::set_current_dir(&path)?;
        Ok(Self { origin })
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self){
        let _ = fs::set_current_dir(&self.original);
    }
}

// extract tar
pub fn extract_tar(
    input: Vec<u8>,
    strip_components: Option<usize>,
    output: &Path,
)-> std::io::Result<()>{
    let cursor = Cursor::new(input);
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(cursor));

    for entry in archive.entries()? {
        let mut entry = entry?;
        if let Some(strip) = strip_components {
            let path = entry.path()?;
            let new_path: PathBuf = path.skip(strip).collect();
            entry.unpack(output.join(new_path))?;
        } else {
            entry.unpack(output)?;
        }
    }
    Ok(())
}