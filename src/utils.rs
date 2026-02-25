use colored::Colorize;
use std::path::PathBuf;

/// returns ~/cp/exercises/
pub fn exercises_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not resolve home directory")
        .join("cp")
        .join("exercises")
}

/// returns ~/.cp/templates/template.cpp
pub fn template_path() -> PathBuf {
    dirs::home_dir()
        .expect("could not resolve home directory")
        .join(".cp")
        .join("templates")
        .join("template.cpp")
}

pub fn success(msg: &str) {
    println!("{} {}", "[✓]".green(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", "[~]".yellow(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "[!]".red(), msg);
}

/// print indented file description line
pub fn file_line(name: &str, desc: &str) {
    println!("    {:<15}{}", name, desc);
}
