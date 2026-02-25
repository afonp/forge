use git2::{IndexAddOption, Repository, Signature};
use std::path::Path;

use crate::utils;

/// ensure the exercises directory is a git repo. initialize one if it isn't.
pub fn ensure_repo(path: &Path) -> Option<Repository> {
    match Repository::open(path) {
        Ok(repo) => Some(repo),
        Err(_) => match Repository::init(path) {
            Ok(repo) => {
                utils::success(&format!("initialized git repo at {}", path.display()));
                Some(repo)
            }
            Err(e) => {
                utils::warn(&format!("could not initialize git repo: {}", e));
                None
            }
        },
    }
}

/// stage all changes and commit with the given message.
pub fn commit(repo: &Repository, message: &str) {
    let sig = match repo.signature() {
        Ok(s) => s,
        Err(_) => match Signature::now("forge", "forge@local") {
            Ok(s) => s,
            Err(e) => {
                utils::warn(&format!("could not create git signature: {}", e));
                return;
            }
        },
    };

    let mut index = match repo.index() {
        Ok(i) => i,
        Err(e) => {
            utils::warn(&format!("could not get git index: {}", e));
            return;
        }
    };

    // stage all changes
    if let Err(e) = index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None) {
        utils::warn(&format!("could not stage files: {}", e));
        return;
    }

    // also pick up removals
    if let Err(e) = index.update_all(["*"].iter(), None) {
        // not fatal, continue
        let _ = e;
    }

    if let Err(e) = index.write() {
        utils::warn(&format!("could not write index: {}", e));
        return;
    }

    let tree_oid = match index.write_tree() {
        Ok(oid) => oid,
        Err(e) => {
            utils::warn(&format!("could not write tree: {}", e));
            return;
        }
    };

    let tree = match repo.find_tree(tree_oid) {
        Ok(t) => t,
        Err(e) => {
            utils::warn(&format!("could not find tree: {}", e));
            return;
        }
    };

    // get parent commit if it exists
    let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
    let parents: Vec<&git2::Commit> = parent.iter().collect();

    match repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents) {
        Ok(_) => utils::success(&format!("committed: {}", message)),
        Err(e) => utils::warn(&format!("could not commit: {}", e)),
    }
}
