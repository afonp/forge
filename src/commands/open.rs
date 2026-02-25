use std::process::Command;

use crate::utils;

pub fn run(name: &str) {
    let dir = utils::exercises_dir().join(name);

    if !dir.exists() {
        utils::error(&format!("exercise not found: {}", name));
        return;
    }

    // try $EDITOR first, then code, then just print the path
    if let Ok(editor) = std::env::var("EDITOR") {
        match Command::new(&editor).arg(&dir).spawn() {
            Ok(_) => utils::success(&format!("opened {} in {}", name, editor)),
            Err(e) => utils::error(&format!("could not open {}: {}", editor, e)),
        }
    } else {
        match Command::new("code").arg(&dir).spawn() {
            Ok(_) => utils::success(&format!("opened {} in vs code", name)),
            Err(_) => {
                utils::warn(&format!("no editor found. path: {}", dir.display()));
            }
        }
    }
}
