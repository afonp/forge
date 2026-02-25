use std::fs;

use crate::utils;

pub const TEMPLATE: &str = include_str!("../assets/template.cpp");

/// ensure the global template file exists at ~/.cp/templates/template.cpp.
/// if it doesn't, create it from the embedded template.
pub fn ensure_template() {
    let path = utils::template_path();
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    if let Err(e) = fs::write(&path, TEMPLATE) {
        utils::warn(&format!(
            "could not write template to {}: {}",
            path.display(),
            e
        ));
    }
}

/// read the template contents. prefers the user's copy at ~/.cp/templates/template.cpp,
/// falls back to the embedded version.
pub fn read_template() -> String {
    let path = utils::template_path();
    if path.exists() {
        if let Ok(contents) = fs::read_to_string(&path) {
            return contents;
        }
    }
    TEMPLATE.to_string()
}

/// makefile content for an exercise
pub fn makefile() -> &'static str {
    r#"CXX      = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra -DLOCAL
TARGET   = solution
SRC      = solution.cpp

all: $(SRC)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRC)

run: all
	./$(TARGET)

test: all
	./$(TARGET) < input.txt

debug: $(SRC)
	$(CXX) -std=c++17 -g -fsanitize=address,undefined -DLOCAL -Wall -Wextra -o $(TARGET) $(SRC)
	./$(TARGET) < input.txt

clean:
	rm -f $(TARGET)

.PHONY: all run test debug clean
"#
}

/// notes.md template
pub fn notes_md(name: &str) -> String {
    format!(
        r#"# {}

## problem


## approach


## complexity

- time:
- space:

## notes

"#,
        name
    )
}
