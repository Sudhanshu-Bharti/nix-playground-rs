use git2::{Repository, };
use crate::constants::app_config::{CHECKOUT_LINK, DEFAULT_CHECKOUT_DIR};

pub fn _clean() -> Result<(), git2::Error> {
    let repo = Repository::open(DEFAULT_CHECKOUT_DIR)?;
    // TODO
    let np_dir: bool = ;
    let checkout_link = np_dir/ CHECKOUT_LINK;
    let head = repo.head()?.peel_to_tree()?;
    let index = repo.index()?;



    log::info!("Finished creating diff");

    Ok(())
}