mod commands;
mod git;
mod template;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "forge", about = "competitive programming exercise scaffolder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new exercise or contest
    New {
        /// exercise name or contest name
        name: String,

        /// problem labels (for contest mode, e.g. a b c d)
        problems: Vec<String>,

        /// open in vs code after creation
        #[arg(short = 'c', long = "code")]
        code: bool,

        /// open in $EDITOR after creation
        #[arg(short = 'o', long = "editor")]
        editor: bool,
    },

    /// list all exercises
    List,

    /// open an exercise in your editor
    Open {
        /// exercise name
        name: String,
    },

    /// remove compiled binaries from an exercise
    Clean {
        /// exercise name
        name: String,
    },

    /// check and install dependencies (g++, make)
    Setup,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            name,
            problems,
            code,
            editor,
        } => {
            if problems.is_empty() {
                commands::new::single(&name, code, editor);
            } else {
                commands::new::contest(&name, &problems, code, editor);
            }
        }
        Commands::List => commands::list::run(),
        Commands::Open { name } => commands::open::run(&name),
        Commands::Clean { name } => commands::clean::run(&name),
        Commands::Setup => commands::setup::run(),
    }
}
