#[derive(Debug)]
pub enum PatternType {
    IdenticalRhs,
    RepeatedLhs,
    SelfAssignment,
}

#[derive(Debug)]
pub struct Pattern {
    pub pattern_type: PatternType,
    pub line_num: usize,
    pub content: String,
    // These fields are used in analysis.rs to store tokenized values
    #[allow(dead_code)]
    pub lhs: Vec<String>,
    #[allow(dead_code)]
    pub rhs: Vec<String>,
    #[allow(dead_code)]
    pub operators: Vec<String>,
}

pub fn tokenize_line(line: &str) -> Vec<String> {
    // Remove comments from the line (assume // for C-like languages)
    let line = line.split("//").next().unwrap_or(line).trim().to_string();
    if line.is_empty() {
        return vec![];
    }

    // Regular expression to capture identifiers, numbers, operators, etc.
    let re = regex::Regex::new(r"(?:[^\s[\[\],]]+|\[|\]|\d+)").unwrap();
    let mut tokens = vec![];

    for cap in re.captures_iter(line.as_str()) {
        if let Some(m) = cap.get(0) {
            tokens.push(m.as_str().to_string());
        }
    }

    tokens
}

pub fn normalize_identifiers(tokens: &[String]) -> Vec<String> {
    tokens.iter().map(|t| {
        if t.parse::<i64>().is_ok() {
            "<NUM>".to_string()
        } else if t.chars().all(|c| c.is_ascii_alphanumeric()) {
            "<VAR>".to_string()
        } else {
            t.clone()
        }
    }).collect()
}

pub fn detect_assignment(line: &str) -> Option<(Vec<String>, String, Vec<String>)> {
    let tokens = tokenize_line(line);
    if tokens.is_empty() || tokens.len() < 3 {
        return None; // Need at least 3 tokens: lhs, operator, rhs
    }

    // Look for assignment patterns
    for i in 0..(tokens.len() - 2) {
        if ["=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "&&="].contains(&tokens[i].as_str()) {
            // LHS: tokens[0..i], operator: tokens[i], RHS: tokens[i+1..]
            let lhs = tokens[0..i].to_vec();
            let op = tokens[i].clone();
            let rhs = tokens[i+1..].to_vec();
            return Some((lhs, op, rhs));
        }
    }

    None
}
