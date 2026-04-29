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