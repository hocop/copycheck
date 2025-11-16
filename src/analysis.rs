use crate::pattern::{PatternType, normalize_identifiers, detect_assignment, Pattern};
use crate::utilities::is_text_extension;
use ignore::WalkBuilder;
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

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
        println!("Analyzing file: {}", file_path.display());
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
        println!("Scanning directory: {}", path.display());
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
                    println!("Found text file: {}", entry_path.display());
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
            let rhs_clean: Vec<String> = rhs.iter()
                .take_while(|token| !token.starts_with('#'))
                .map(|s| s.to_string())
                .collect();

            // Filter out comment parts from LHS
            let lhs_clean: Vec<String> = lhs.iter()
                .take_while(|token| !token.starts_with('#'))
                .map(|s| s.to_string())
                .collect();

            if lhs_clean.is_empty() || rhs_clean.is_empty() {
                continue;
            }

            // Normalize the tokens for semantic comparison
            let norm_lhs = normalize_identifiers(&lhs_clean);
            let norm_rhs = normalize_identifiers(&rhs_clean);

            // Check all rules
            let mut pattern_type = None;

            // Rule R1: Identical Right-Hand Sides
            if i > 0 && i < window + 1 {
                for j in (i as isize - window as isize).max(0) as usize..i {
                    if let Some((_prev_lhs, _prev_op, prev_rhs)) = detect_assignment(lines[j]) {
                        // Filter out comment parts from previous RHS
                        let prev_rhs_clean: Vec<String> = prev_rhs.iter()
                            .take_while(|token| !token.starts_with('#'))
                            .map(|s| s.to_string())
                            .collect();

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
                                lhs: lhs_clean.clone(),
                                rhs: rhs_clean.clone(),
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
                        if let Some((_prev_lhs, _prev_op, prev_rhs)) = detect_assignment(lines[j]) {
                            // Filter out comment parts from previous LHS
                            let prev_lhs_clean: Vec<String> = _prev_lhs.iter()
                                .take_while(|token| !token.starts_with('#'))
                                .map(|s| s.to_string())
                                .collect();

                            // Normalize the previous LHS for comparison
                            let prev_norm_lhs = normalize_identifiers(&prev_lhs_clean);

                            if prev_lhs_clean.is_empty() {
                                continue;
                            }

                            if norm_lhs == prev_norm_lhs {
                                pattern_type = Some(PatternType::RepeatedLhs);
                                patterns.push(Pattern {
                                    pattern_type: PatternType::RepeatedLhs,
                                    line_num: j + 1, // Use the previous line's number for comparison
                                    content: lines[j].to_string(),
                                    lhs: prev_lhs_clean,
                                    rhs: prev_rhs.clone(),
                                    operators: vec![_prev_op.clone()],
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
                if let Some(lhs_name) = lhs_clean.get(0) {
                    if rhs_clean.iter().any(|v| v == lhs_name) {
                        pattern_type = Some(PatternType::SelfAssignment);
                        patterns.push(Pattern {
                            pattern_type: PatternType::SelfAssignment,
                            line_num: i + 1,
                            content: line.to_string(),
                            lhs: lhs_clean.clone(),
                            rhs: rhs_clean.clone(),
                            operators: vec![op.clone()],
                        });
                    }
                }
            }

            // Rule R7: Repeated Operand
            if pattern_type.is_none() {
                if rhs_clean.len() >= 2 {
                    if rhs_clean[0] == rhs_clean[1] || lhs_clean.iter().any(|v| v == &rhs_clean[0]) && lhs_clean.iter().any(|v| v == &rhs_clean[1]) {
                        pattern_type = Some(PatternType::RepeatedOperand);
                        // Use the pattern_type variable
                        if let Some(pt) = pattern_type {
                            patterns.push(Pattern {
                                pattern_type: pt,
                                line_num: i + 1,
                                content: line.to_string(),
                                lhs: lhs_clean.clone(),
                                rhs: rhs_clean.clone(),
                                operators: vec![op.clone()],
                            });
                        }
                    }
                }
            }

            // Rule R10: Multi-Increment (disabled for now)
            // (no changes needed for this disabled rule)

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
        println!("No issues found in {}", file_path);
    }

    Ok(())
}

pub fn count_lines(paths: &[PathBuf], extensions: Option<&str>) -> Result<HashMap<String, usize>, Box<dyn std::error::Error>> {
    // Initialize a BTreeMap to store line counts by file extension
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    // Parse the extensions parameter into a set for efficient lookup
    let _ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    paths.iter()
        .flat_map(|path| WalkBuilder::new(path).build().filter_map(Result::ok))
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            // Get file extension and convert to lowercase
            entry.path().extension()
                .and_then(|os_str| os_str.to_str())
                .map(|extension| extension.to_lowercase())
                .map(|extension| (entry, extension))
        })
        .filter(|(_entry, extension)| {
            // Check if this extension is in the allowed set (if any)
            if let Some(ref _ext_set) = _ext_set {
                _ext_set.contains(extension)
            } else {
                // If no extensions specified, include all
                is_text_extension(extension)
            }
        })
        // Collect all matching files into a vector for parallel processing
        .collect::<Vec<_>>()
        // Try to count lines for each file (skip files that can't be read)
        .iter()
        .filter_map(|(entry, extension)| {
            count_file_lines(entry.path()).ok().map(|count| (extension.clone(), count))
        })
        // Collect results and update the counts map
        .for_each(|(ext, count)| {
            counts.entry(ext).and_modify(|c| *c += count).or_insert(count);
        });

    // Convert BTreeMap to HashMap before returning
    Ok(counts.clone().into_iter().collect())
}

/// Count lines in a single file
fn count_file_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    // Use stdio to read line by line, skip empty lines
    // This is more memory efficient than reading entire file
    let file = fs::File::open(path)?;
    let mut count = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if !line.trim().is_empty() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn count_file_sizes(paths: &[PathBuf], extensions: Option<&str>) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    // Initialize a BTreeMap to store size counts by file extension
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();

    // Parse the extensions parameter into a set for efficient lookup
    let _ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    paths.iter()
        .flat_map(|path| WalkBuilder::new(path).build().filter_map(Result::ok))
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            // Get file extension and convert to lowercase
            entry.path().extension()
                .and_then(|os_str| os_str.to_str())
                .or(Some("_"))
                .map(|extension| extension.to_lowercase())
                .map(|extension| (entry, extension))
        })
        .filter(|(_entry, extension)| {
            // Check if this extension is in the allowed set (if any)
            if let Some(ref _ext_set) = _ext_set {
                _ext_set.contains(extension)
            } else {
                // Include all files regardless of extension (unlike count_lines)
                true
            }
        })
        // Collect all matching files into a vector for parallel processing
        .collect::<Vec<_>>().iter()
        // Try to count file size for each file (skip files that can't be read)
        .filter_map(|(entry, extension)| {
            count_file_size(entry.path()).map(|size| (extension.clone(), size)).ok()
        })
        // Collect results and update the counts map
        .for_each(|(ext, count)| {
            let mut found = false;
            for (k, v) in counts.iter_mut() {
                if k == &ext {
                    *v += count;
                    found = true;
                    break;
                }
            }
            if !found {
                counts.insert(ext, count);
            }
        });

    // Convert BTreeMap to HashMap before returning
    Ok(counts.clone().into_iter().collect())
}

/// Count size of a single file
fn count_file_size(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    // Use metadata to get file size
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}
