use std::fs;

use crate::utils;

pub fn run() {
    let base = utils::exercises_dir();

    if !base.exists() {
        utils::warn("no exercises found");
        return;
    }

    let mut entries: Vec<(String, String)> = Vec::new();

    if let Ok(dir) = fs::read_dir(&base) {
        for entry in dir.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().to_string();
                // skip hidden directories
                if name.starts_with('.') {
                    continue;
                }
                let date = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.created().ok().or_else(|| m.modified().ok()))
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Local> = t.into();
                        dt.format("%Y-%m-%d %H:%M").to_string()
                    })
                    .unwrap_or_else(|| "unknown".to_string());
                entries.push((name, date));
            }
        }
    }

    if entries.is_empty() {
        utils::warn("no exercises found");
        return;
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, date) in &entries {
        println!("  {:<30} {}", name, date);
    }

    println!();
    utils::success(&format!("{} exercise(s)", entries.len()));
}
