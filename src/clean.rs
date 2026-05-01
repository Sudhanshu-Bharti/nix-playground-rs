use std::fs;
use git2::{Repository, };
use crate::constants::app_config::{CHECKOUT_LINK, DEFAULT_CHECKOUT_DIR, PLAYGROUND_DIR};
use crate::utils::{ensure_np_dir};

pub fn _clean() -> Result<(), git2::Error> {
    let np_dir = ensure_np_dir(PLAYGROUND_DIR.as_ref())?;
    let checkout_link = np_dir.join(CHECKOUT_LINK);

    let checkout_dir = if checkout_link.exists() {
        checkout_link.read_link().ok()
    } else {
        None
    };
    log::info!("Deleting checkout link {}", checkout_link);
    fs::remove_file(checkout_link)?;
    if checkout_dir.is_none() {
        log::info!("Deleting checkout dir {}", checkout_dir);
    }
    log::info!("Deleting nix-playground dir {}", np_dir.display());

    if np_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&np_dir) {
            eprintln!("Failed to remove dir: {}", e);
        }
    }
    log::info!("Done");
}