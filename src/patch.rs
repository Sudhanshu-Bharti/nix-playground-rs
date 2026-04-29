use git2::{Repository, DiffFormat};
use crate::constants::app_config::DEFAULT_CHECKOUT_DIR;

pub fn create_diff() -> Result<(), git2::Error> {
    let repo = Repository::open(DEFAULT_CHECKOUT_DIR)?;

    let head = repo.head()?.peel_to_tree()?;
    let index = repo.index()?;

    let diff = repo.diff_tree_to_index(Some(&head), Some(&index), None)?;

    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        print!("{}", std::str::from_utf8(line.content()).unwrap());
        true
    })?;

    log::info!("Finished creating diff");

    Ok(())
}

// apply patch
pub fn apply_patch(repo : &Repository, patch_file: &Path)->anyhow::Result<>{
    let ext = patch_file.extension().and_then(|s| s.to_str());
    if matches!(ext, Some("gz"| "bz2" | "xz" | "lzma")){
        error("Currently compressed patch file is not supported yet, please see the issue https://github.com/LaunchPlatform/nix-playground/issues/5 to learn more");
        anyhow::bail!("Currently compressed patch file is not supported yet");
    }
    let bytes = fs::read(patch_file)?;

    match Diff::from_buffer(&bytes){
        Ok(diff) => {
            repo.apply(&diff, git2::ApplyLocation::WorkDir, None)?;
        }
        Err(error) => {
            info!("Git apply failed, falling back: {}", e);
            let status = Command::new("patch")
                .arg("-f")
                .arg("-i")
                .arg(patch_file)
                .arg("-p1")
                .current_dir(repo.workdir().unwrap())
                .status()?;
            if !status.success() {
                anyhow::bail!("patch Cmd failed");
            }
        }
    }
    Ok(())
}