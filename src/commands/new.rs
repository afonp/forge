use std::fs;
use std::process::Command;

use crate::git;
use crate::template;
use crate::utils;

/// scaffold a single exercise directory.
/// returns true if created successfully (not a duplicate).
fn scaffold(name: &str) -> bool {
    let dir = utils::exercises_dir().join(name);

    if dir.exists() {
        utils::warn(&format!("exercise already exists: {}", name));
        return false;
    }

    if let Err(e) = fs::create_dir_all(&dir) {
        utils::error(&format!("could not create directory: {}", e));
        return false;
    }

    // write solution.cpp from template
    let tmpl = template::read_template();
    if let Err(e) = fs::write(dir.join("solution.cpp"), &tmpl) {
        utils::error(&format!("could not write solution.cpp: {}", e));
        return false;
    }

    // write Makefile
    if let Err(e) = fs::write(dir.join("Makefile"), template::makefile()) {
        utils::error(&format!("could not write Makefile: {}", e));
        return false;
    }

    // write input.txt and expected.txt (empty)
    fs::write(dir.join("input.txt"), "").ok();
    fs::write(dir.join("expected.txt"), "").ok();

    // write notes.md
    if let Err(e) = fs::write(dir.join("notes.md"), template::notes_md(name)) {
        utils::error(&format!("could not write notes.md: {}", e));
        return false;
    }

    utils::success(&format!("created: {}", dir.display()));
    utils::file_line("solution.cpp", "your solution");
    utils::file_line("Makefile", "make / make run / make test / make debug");
    utils::file_line("input.txt", "paste test input here");
    utils::file_line("expected.txt", "paste expected output here");
    utils::file_line("notes.md", "problem notes");

    true
}

/// open the exercise in an editor
fn open_in_editor(name: &str, use_code: bool, use_editor: bool) {
    let dir = utils::exercises_dir().join(name);
    if use_code {
        Command::new("code").arg(&dir).spawn().ok();
    } else if use_editor {
        if let Ok(editor) = std::env::var("EDITOR") {
            Command::new(&editor).arg(&dir).spawn().ok();
        } else {
            Command::new("code").arg(&dir).spawn().ok();
        }
    }
}

/// create a single exercise
pub fn single(name: &str, code: bool, editor: bool) {
    template::ensure_template();

    let base = utils::exercises_dir();
    fs::create_dir_all(&base).ok();

    let created = scaffold(name);

    if created {
        if let Some(repo) = git::ensure_repo(&base) {
            git::commit(&repo, &format!("add exercise {}", name));
        }
        if code || editor {
            open_in_editor(name, code, editor);
        }
    }
}

/// create multiple problems for a contest
pub fn contest(contest_name: &str, problems: &[String], code: bool, editor: bool) {
    template::ensure_template();

    let base = utils::exercises_dir();
    fs::create_dir_all(&base).ok();

    let mut created: Vec<String> = Vec::new();

    for p in problems {
        let name = format!("{}_{}", contest_name, p);
        if scaffold(&name) {
            created.push(p.clone());
        }
    }

    if !created.is_empty() {
        if let Some(repo) = git::ensure_repo(&base) {
            let labels = created.join(", ");
            let msg = format!("add contest {} ({})", contest_name, labels);
            git::commit(&repo, &msg);
        }
        if code || editor {
            for p in &created {
                let name = format!("{}_{}", contest_name, p);
                open_in_editor(&name, code, editor);
            }
        }
    }
}
