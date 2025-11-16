use crate::pattern::{PatternType, normalize_identifiers, detect_assignment, Pattern};
use crate::utilities::is_text_extension;
use ignore::WalkBuilder;
use std::fs;
use std::path::PathBuf;


/// Filters out tokens that start with comment markers
fn filter_comment_tokens(tokens: Vec<String>) -> Vec<String> {
    tokens
    .into_iter()
    .take_while(|token| !(token.starts_with('#') || token.starts_with("//")))
    .collect()
}

/// Check for copy-edit errors in the specified files
pub fn check_copy_edit_errors(paths: &[PathBuf], window: usize, extensions: Option<&str>,
                             json: &bool, _ignore_paths: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    // Parse extensions into a set for filtering
    let _ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    // Find all text files to analyze
    let text_files = find_text_files(paths, extensions, _ignore_paths);

    // Analyze each file
    for file_path in text_files {
        // println!("Analyzing file: {}", file_path.display());
        let content = fs::read_to_string(&file_path)?;
        let result = analyze_file_content(&file_path.display().to_string(), &content, window, json, extensions);
        if let Err(e) = result {
            eprintln!("Error analyzing file {}: {}", file_path.display(), e);
        }
    }

    Ok(())
}

/// Helper function to find text files to analyze
fn find_text_files(paths: &[PathBuf], extensions: Option<&str>, _ignore_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut text_files = vec![];

    for path in paths {
        // println!("Scanning directory: {}", path.display());
        let walk = WalkBuilder::new(path);

        // Add ignore paths - ignoring functionality needs adjustment
        // This implementation may need to change for the ignore feature
        for entry in walk.build().filter_map(Result::ok) {
            let entry_path = entry.path();

            // Check if path should be ignored
            let _should_ignore = _ignore_paths.iter()
                .any(|ignore_path| entry_path.starts_with(ignore_path));

            // Only process files, not directories
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                // Get file extension
                let extension = entry_path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                // Check if file has a text extension
                let include_file = match extensions {
                    Some(exts) => exts.contains(&extension),
                    None => is_text_extension(&extension),
                };

                if include_file {
                    text_files.push(entry_path.to_path_buf());
                    // println!("Found text file: {}", entry_path.display());
                }
            }
        }
    }

    text_files
}

/// Analyze the content of a single file
fn analyze_file_content(file_path: &str, content: &str, window: usize, json: &bool, _extensions: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut patterns = vec![];

    // Process each line and look for suspicious patterns
    for (i, line) in lines.iter().enumerate() {
        if let Some((lhs, op, rhs)) = detect_assignment(line) {
            // Filter out comment parts from RHS
            let rhs: Vec<String> = filter_comment_tokens(rhs);

            if lhs.is_empty() || rhs.is_empty() {
                continue;
            }

            // Normalize the tokens for semantic comparison
            let norm_lhs = normalize_identifiers(&lhs);
            let norm_rhs = normalize_identifiers(&rhs);

            // Check all rules
            let mut pattern_type = None;

            // Rule R1: Identical Right-Hand Sides
            if i > 0 && i < window + 1 {
                for j in (i as isize - window as isize).max(0) as usize..i {
                    if let Some((_prev_lhs, _prev_op, prev_rhs)) = detect_assignment(lines[j]) {
                        // Filter out comment parts from previous RHS
                        let prev_rhs_clean: Vec<String> = filter_comment_tokens(prev_rhs);

                        // Normalize the previous RHS for comparison
                        let prev_norm_rhs = normalize_identifiers(&prev_rhs_clean);

                        if prev_rhs_clean.is_empty() {
                            continue;
                        }

                        if norm_rhs == prev_norm_rhs {
                            pattern_type = Some(PatternType::IdenticalRhs);
                            // Store the comparison pattern
                            let pattern = Pattern {
                                pattern_type: PatternType::IdenticalRhs,
                                line_num: i + 1,
                                content: line.to_string(),
                                lhs: lhs.clone(),
                                rhs: rhs.clone(),
                                operators: vec![op.clone()],
                            };
                            // Use the fields to avoid unused warnings
                            let _ = pattern.lhs;
                            let _ = pattern.rhs;
                            let _ = pattern.operators;
                            patterns.push(pattern);
                            break;
                        }
                    }
                }
            }

            // Rule R2: Repeated Left-Hand Side
            if pattern_type.is_none() {
                if i > 0 && i < window + 1 {
                    for j in (i as isize - window as isize).max(0) as usize..i {
                        if let Some((prev_lhs, prev_op, prev_rhs)) = detect_assignment(lines[j]) {
                            // Normalize the previous LHS for comparison
                            let prev_norm_lhs = normalize_identifiers(&prev_lhs);

                            if prev_lhs.is_empty() {
                                continue;
                            }

                            if norm_lhs == prev_norm_lhs {
                                pattern_type = Some(PatternType::RepeatedLhs);
                                patterns.push(Pattern {
                                    pattern_type: PatternType::RepeatedLhs,
                                    line_num: j + 1, // Use the previous line's number for comparison
                                    content: lines[j].to_string(),
                                    lhs: prev_lhs.clone(),
                                    rhs: prev_rhs.clone(),
                                    operators: vec![prev_op.clone()],
                                });
                                break;
                            }
                        }
                    }
                }
            }

            // Rule R5: Self-Assignment
            if pattern_type.is_none() {
                // Check for self-assignment (same identifier on both LHS and RHS)
                if lhs == rhs {
                    patterns.push(Pattern {
                        pattern_type: PatternType::SelfAssignment,
                        line_num: i + 1,
                        content: line.to_string(),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        operators: vec![op.clone()],
                    });
                }
            }
        }
    }

    // Print the results
    if !patterns.is_empty() {
        for pattern in patterns {
            if *json {
                // JSON format
                println!(
                    "{{ \"file\": \"{}\", \"line\": {}, \"rule\": \"{:?}\", \"code\": \"{} \" }}",
                    file_path, pattern.line_num,
                    pattern.pattern_type, pattern.content.trim()
                );
            } else {
                // Plain text format
                println!(
                    "{}:{:>4}  [{:?}]  {}",
                    file_path, pattern.line_num,
                    pattern.pattern_type, pattern.content.trim()
                );
            }
        }
    } else {
        // println!("No issues found in {}", file_path);
    }

    Ok(())
}
