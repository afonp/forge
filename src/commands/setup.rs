use std::process::Command;

use crate::utils;

fn has_tool(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_cmd(program: &str, args: &[&str]) -> bool {
    utils::success(&format!("running: {} {}", program, args.join(" ")));
    Command::new(program)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn run() {
    let has_gpp = has_tool("g++");
    let has_make = has_tool("make");

    if has_gpp {
        utils::success("g++ found");
    } else {
        utils::warn("g++ not found");
    }

    if has_make {
        utils::success("make found");
    } else {
        utils::warn("make not found");
    }

    if has_gpp && has_make {
        utils::success("all dependencies installed, you're good to go");
        return;
    }

    utils::success("installing missing dependencies...");

    #[cfg(target_os = "windows")]
    install_windows(has_gpp, has_make);

    #[cfg(target_os = "macos")]
    install_macos(has_gpp, has_make);

    #[cfg(target_os = "linux")]
    install_linux(has_gpp, has_make);

    // verify after install
    println!();
    let ok_gpp = has_tool("g++");
    let ok_make = has_tool("make");

    if ok_gpp {
        utils::success("g++ ready");
    } else {
        utils::error("g++ still not found — you may need to restart your terminal");
    }

    if ok_make {
        utils::success("make ready");
    } else {
        utils::error("make still not found — you may need to restart your terminal");
    }

    if ok_gpp && ok_make {
        utils::success("setup complete");
    } else {
        utils::warn("restart your terminal and run 'forge setup' again to verify");
    }
}

#[cfg(target_os = "windows")]
fn install_windows(has_gpp: bool, has_make: bool) {
    // try scoop first — cleanest for cli tools
    let has_scoop = has_tool("scoop");

    if has_scoop {
        if !has_gpp {
            if !run_cmd("scoop", &["install", "gcc"]) {
                utils::error("failed to install gcc via scoop");
            }
        }
        if !has_make {
            if !run_cmd("scoop", &["install", "make"]) {
                utils::error("failed to install make via scoop");
            }
        }
        return;
    }

    // try winget
    let has_winget = has_tool("winget");

    if has_winget {
        if !has_gpp || !has_make {
            utils::success("installing msys2 via winget (includes g++ and make)...");
            if run_cmd(
                "winget",
                &[
                    "install",
                    "-e",
                    "--id",
                    "MSYS2.MSYS2",
                    "--accept-package-agreements",
                    "--accept-source-agreements",
                ],
            ) {
                utils::success("msys2 installed");
                utils::warn("run the following in msys2 terminal to install g++ and make:");
                println!("    pacman -S --noconfirm mingw-w64-x86_64-gcc make");
                utils::warn("then add C:\\msys64\\mingw64\\bin to your PATH");
            } else {
                utils::error("failed to install msys2 via winget");
            }
        }
        return;
    }

    // no package manager found — install scoop
    utils::success("no package manager found, installing scoop...");
    let scoop_install = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force; irm get.scoop.sh | iex",
        ])
        .status();

    match scoop_install {
        Ok(s) if s.success() => {
            utils::success("scoop installed");
            // scoop is now available but may need a new shell for PATH
            // try to use it directly via its known path
            let scoop_path = format!(
                "{}\\scoop\\shims\\scoop.cmd",
                std::env::var("USERPROFILE").unwrap_or_default()
            );
            if !has_gpp {
                run_cmd(&scoop_path, &["install", "gcc"]);
            }
            if !has_make {
                run_cmd(&scoop_path, &["install", "make"]);
            }
        }
        _ => {
            utils::error("could not install scoop automatically");
            utils::warn("install manually: https://scoop.sh");
            utils::warn("then run: scoop install gcc make");
        }
    }
}

#[cfg(target_os = "macos")]
fn install_macos(_has_gpp: bool, _has_make: bool) {
    // xcode command line tools gives clang (as g++) and make
    utils::success("installing xcode command line tools...");
    if !run_cmd("xcode-select", &["--install"]) {
        // xcode-select --install returns non-zero if already installed or if it opens a GUI dialog
        utils::warn("if a dialog appeared, follow the prompts to install");
        utils::warn("if already installed, you may need to update: softwareupdate --install -a");
    }
}

#[cfg(target_os = "linux")]
fn install_linux(has_gpp: bool, has_make: bool) {
    if has_gpp && has_make {
        return;
    }

    // detect package manager and install
    if has_tool("apt") {
        utils::success("detected apt (debian/ubuntu)");
        run_cmd("sudo", &["apt", "update", "-y"]);
        run_cmd("sudo", &["apt", "install", "-y", "build-essential"]);
    } else if has_tool("dnf") {
        utils::success("detected dnf (fedora/rhel)");
        run_cmd("sudo", &["dnf", "install", "-y", "gcc-c++", "make"]);
    } else if has_tool("pacman") {
        utils::success("detected pacman (arch)");
        run_cmd("sudo", &["pacman", "-S", "--noconfirm", "gcc", "make"]);
    } else if has_tool("zypper") {
        utils::success("detected zypper (opensuse)");
        run_cmd("sudo", &["zypper", "install", "-y", "gcc-c++", "make"]);
    } else if has_tool("apk") {
        utils::success("detected apk (alpine)");
        run_cmd("sudo", &["apk", "add", "g++", "make"]);
    } else {
        utils::error("could not detect package manager");
        utils::warn("install g++ and make manually using your distribution's package manager");
    }
}
