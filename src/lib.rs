//! # Logic Expression Pest Parser
//!
//! This crate provides tools to parse, evaluate and generate truth tables for boolean logic expression.

/// # Abstract Syntax Tree (AST) Module
///
/// Defines the `Expression` enum, which represents a node in the AST, and implements methods for its creation, evaluation, and variable collection.
pub mod ast;

/// # Parser Module
///
/// Defines the Pest `Grammar`, parser errors, and the `parse` function.
pub mod parser;

/// # Truth Table Module
///
/// Defines `TruthTable` and `TruthTableRow`, implements the `Display` trait for both structs and the `From` trait for `TruthTable`.
pub mod truth_table;
