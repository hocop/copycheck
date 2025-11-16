use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool to detect copy-paste and edit errors across programming languages
#[derive(Parser)]
#[command(name = "copyedit-check")]
#[command(about = "Detect copy-paste and edit errors", long_about = None)]
struct Cli {
    /// One or more paths to search (default is current directory)
    #[arg(default_value = ".", global = true)]
    paths: Vec<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check for copy-edit errors in code
    Check {
        /// Number of neighboring lines to compare (default: 5)
        #[arg(long, default_value_t = 5, value_name = "WINDOW")]
        window: usize,

        /// File extensions to include (default: all)
        #[arg(long, value_name = "EXTENSIONS")]
        extensions: Option<String>,

        /// Output results as JSON
        #[arg(long)]
        json: bool,

        /// Skip specific folders
        #[arg(long, value_name = "PATH")]
        ignore: Vec<PathBuf>,
    },
}

mod analysis;
mod pattern;
mod utilities;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { window, extensions, json, ignore } => {
            if let Err(e) = analysis::check_copy_edit_errors(&cli.paths, *window, extensions.as_deref(), json, ignore) {
                eprintln!("Error checking copy-edit errors: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::analysis::analyze_file_content;
    use super::pattern::{Pattern, PatternType};
    use std::fs;

    #[test]
    fn test_analyze_file_content() {
        // Test with test_input/test.py
        let file_path = "test_input/test.py";
        let content = fs::read_to_string(file_path).expect("Unable to read test file");
        let window = 5;
        let json = &false;
        let extensions = None;

        let patterns = analyze_file_content(file_path, &content, window, json, extensions);

        // We should find patterns in the test file
        assert!(!patterns.is_empty());

        // Check for specific pattern types
        let identical_rhs_patterns: Vec<&Pattern> = patterns.iter()
            .filter(|p| matches!(p.pattern_type, PatternType::IdenticalRhs))
            .collect();

        let repeated_lhs_patterns: Vec<&Pattern> = patterns.iter()
            .filter(|p| matches!(p.pattern_type, PatternType::RepeatedLhs))
            .collect();

        assert!(identical_rhs_patterns.len() > 0);
        assert!(repeated_lhs_patterns.len() > 0);
    }

    #[test]
    fn test_analyze_complex_file() {
        // Test with test_input/complex_test.py
        let file_path = "test_input/complex_test.py";
        let content = fs::read_to_string(file_path).expect("Unable to read complex test file");
        let window = 10;
        let json = &false;
        let extensions = None;

        let patterns = analyze_file_content(file_path, &content, window, json, extensions);

        // We should find patterns in the complex test file
        assert!(!patterns.is_empty());

        // Check for specific pattern types
        let identical_rhs_patterns: Vec<&Pattern> = patterns.iter()
            .filter(|p| matches!(p.pattern_type, PatternType::IdenticalRhs))
            .collect();

        let repeated_lhs_patterns: Vec<&Pattern> = patterns.iter()
            .filter(|p| matches!(p.pattern_type, PatternType::RepeatedLhs))
            .collect();

        let self_assignment_patterns: Vec<&Pattern> = patterns.iter()
            .filter(|p| matches!(p.pattern_type, PatternType::SelfAssignment))
            .collect();

        assert!(identical_rhs_patterns.len() > 0);
        assert!(repeated_lhs_patterns.len() > 0);
        assert!(self_assignment_patterns.len() > 0);
    }

    #[test]
    fn test_empty_file() {
        // Test with an empty file content
        let file_path = "empty_test";
        let content = "";
        let window = 5;
        let json = &false;
        let extensions = None;

        let patterns = analyze_file_content(file_path, &content, window, json, extensions);

        // Should return empty vector for empty content
        assert!(patterns.is_empty());
    }
}
