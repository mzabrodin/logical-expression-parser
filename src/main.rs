use anyhow::{Context, Result};
use clap::{ArgGroup, Parser, Subcommand};
use logical_expression_pest_parser::parse_expressions;
use logical_expression_pest_parser::truth_table::TruthTable;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    arg_required_else_help = true,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print author
    #[arg(long)]
    author: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Display project information
    Credits,

    /// Parse a file with logical expressions or string with one logical expression
    #[command(group(ArgGroup::new("input").args(["file", "expression"]).required(true)))]
    Parse {
        /// Path to the file for parsing
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,

        /// Logical expression for parsing
        #[arg(short, long, value_name = "EXPRESSION")]
        expression: Option<String>,

        /// Boolean flag for showing or not AST
        #[arg(short, long)]
        ast: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.author {
        println!("{}", env!("CARGO_PKG_AUTHORS"));
        return Ok(());
    }

    match &cli.command {
        Some(Commands::Credits) => print_credits(),
        Some(Commands::Parse {
            file,
            expression,
            ast,
        }) => run_parse(file.as_deref(), expression.as_deref(), *ast)?,
        None => {}
    }

    Ok(())
}

fn print_credits() {
    println!("-=- Logical Expression Parser -=-");
    println!("By: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}

fn run_parse(file: Option<&Path>, expression: Option<&str>, show_ast: bool) -> Result<()> {
    let content = match (file, expression) {
        (Some(f), None) => {
            println!("Processing file: {}\n", f.display());
            let mut content = fs::read_to_string(f)
                .with_context(|| format!("Failed to read file: {}", f.display()))?;
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content
        }

        (None, Some(e)) => {
            println!("Processing expression from console");
            format!("{e}\n")
        }

        _ => unreachable!(),
    };

    for (index, (input, expression)) in parse_expressions(&content)?.into_iter().enumerate() {
        println!("Expression {}", index + 1);
        println!("Input: \"{input}\"");

        if show_ast {
            println!("\nAST: {expression:?}");
        }

        println!("\n{}\n", TruthTable::from(&expression));
    }

    Ok(())
}
