# copyedit - Detect Copy-Paste and Edit Errors Across Languages

`copyedit` is a lightweight, language-agnostic static analyzer designed to catch copy-edit mistakes that compilers and linters usually miss: small, plausible-looking lines that are syntactically valid but semantically suspicious.

It operates purely on tokenized text (no parsing, no language-specific rules), making it suitable for any programming language, config file, or expression-based script.

## Features

- Detects subtle copy-paste errors missed by typical linters
- Language-agnostic: works on any file with assignment-like statements
- Lightweight: single-pass, line-by-line analysis
- Heuristic, not opinionated: flags suspicious patterns for human review

## Rules Implemented

### R1 - Identical Right-Hand Sides (RHS)

Detects multiple assignments with identical RHS expressions but different targets.

```python
x1 = y + z
x2 = y + z  # same RHS → possible copy-paste error
```

### R2 - Repeated Left-Hand Side (LHS)

Detects multiple assignments to the same variable with similar RHS patterns.

```python
x = y + z
x = a + b  # same LHS in close proximity → maybe should be another variable?
```

### R5 - Self-Assignment

Detects useless or accidental self-assignments.

```python
x = x  # likely unintended
```

### (todo) R10 - Multi-Increment / Index Mismatch

Detects sequential code where one variable changes but others do not.

```python
sum1 = a[0] + b[0]
sum2 = a[1] + b[0]  # index mismatch → likely meant b[1]
```

## How It Works

1. **Tokenization:** Each line is split into identifiers, numbers, and symbols (language-agnostic regex)
2. **Normalization:** Identifiers and literals are replaced with placeholders (<VAR>, <NUM>), allowing semantic comparison
3. **Sliding-window analysis:** Lines are compared within a configurable window (default ±5) for suspicious repetition
4. **Heuristic matching:** Simple rule-based checks are applied to detect the patterns above

## Usage

```bash
copyedit check path/to/source/
```

Options:

```
--window N      # number of neighboring lines to compare (default: 5)
--ext .py,.cpp  # file extensions to include (default: all)
--json          # output results as JSON
--ignore tests/ # skip specific folders
```

Output example:

```
file.py:42  [R1]  x2 = y + z   — identical RHS as line 41
file.py:73  [R7]  diff = x - x — repeated operand
```

## Limitations

- Not a parser — works purely by token similarity
- May produce false positives in repetitive code (e.g. mathematical constants)
- Designed for signal, not certainty — human judgment required

## Implementation Details

The implementation is written in Rust for performance and portability. It includes:

1. **Pattern recognition module** (`src/pattern.rs`) - Handles line tokenization and pattern matching
2. **Analysis engine** (`src/analysis.rs`) - Implements the 5 detection rules across files
3. **Command-line interface** (`src/main.rs`) - Provides user-friendly options and output
4. **Utility functions** (`src/utilities.rs`) - Common helpers for file handling

All patterns are detected using simple semantic analysis rather than language-specific parsing.

## Running Tests

To run tests:

```bash
cargo test
