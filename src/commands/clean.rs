use std::process::Command;

use crate::git;
use crate::utils;

pub fn run(name: &str) {
    let dir = utils::exercises_dir().join(name);

    if !dir.exists() {
        utils::error(&format!("exercise not found: {}", name));
        return;
    }

    match Command::new("make").arg("clean").current_dir(&dir).output() {
        Ok(output) => {
            if output.status.success() {
                utils::success(&format!("cleaned: {}", name));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                utils::error(&format!("make clean failed: {}", stderr.trim()));
                return;
            }
        }
        Err(e) => {
            utils::error(&format!("could not run make clean: {}", e));
            return;
        }
    }

    let base = utils::exercises_dir();
    if let Some(repo) = git::ensure_repo(&base) {
        git::commit(&repo, &format!("clean exercise {}", name));
    }
}
