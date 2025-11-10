use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use logical_expression_pest_parser::ast::Expression;
use logical_expression_pest_parser::parser::{Rule, parse};
use logical_expression_pest_parser::truth_table::TruthTable;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
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
    Parse {
        /// Path to the file for parsing
        #[arg(short, long, value_name = "FILE", group = "input", required = true)]
        file: Option<PathBuf>,

        /// Logical expression for parsing
        #[arg(
            short,
            long,
            value_name = "EXPRESSION",
            group = "input",
            required = true
        )]
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

    if let Some(command) = &cli.command {
        match command {
            Commands::Credits => {
                println!("-=- Logical Expression Parser -=-");
                println!("By: {}", env!("CARGO_PKG_AUTHORS"));
                println!("Version: {}", env!("CARGO_PKG_VERSION"));
                println!("License: {}", env!("CARGO_PKG_LICENSE"));
                println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
                println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
            }

            Commands::Parse {
                file,
                expression,
                ast: show_ast,
            } => {
                let content: String = match (file, expression) {
                    (Some(f), None) => {
                        println!("Processing file: {}\n", f.display());
                        fs::read_to_string(f)
                            .with_context(|| format!("Failed to read file: {}", f.display()))?
                    }

                    (None, Some(e)) => {
                        println!("Processing expression from console");
                        format!("{}\n", e)
                    }

                    _ => unreachable!(),
                };

                let mut pairs = parse(&content)?;
                let file_pair = pairs.next().context("Unexpected EOF")?;

                let mut expression_count = 0;

                for pair in file_pair.into_inner() {
                    if pair.as_rule() == Rule::expression {
                        expression_count += 1;
                        println!("Expression {}", expression_count);
                        println!("Input: \"{}\"", pair.as_str());

                        let expression = Expression::ast(pair);

                        if *show_ast {
                            println!("\nAST: {:?}", expression);
                        }

                        println!("\n{}", TruthTable::from(&expression));
                    }
                }
            }
        }
    }

    Ok(())
}
