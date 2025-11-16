Here’s a clean, practical **README.md** draft for your hypothetical tool:

---

# 🕵️‍♂️ copyedit-check — Detect copy-paste & edit errors across languages

`copyedit-check` is a lightweight, **language-agnostic static analyzer** designed to catch *copy-edit mistakes* that compilers and linters usually miss — small, plausible-looking lines that are *syntactically valid* but *semantically suspicious*.

It operates purely on tokenized text (no parsing, no language-specific rules), making it suitable for **any programming language**, config file, or expression-based script.

---

## ✨ Features

* 🔍 **Detects subtle copy-paste errors** missed by typical linters.
* 🌍 **Language-agnostic:** works on any file with assignment-like statements.
* ⚡ **Lightweight:** single-pass, line-by-line analysis.
* 🧠 **Heuristic, not opinionated:** flags *suspicious patterns* for human review.

---

## 🧾 Rules Implemented

### **R1 — Identical Right-Hand Sides (RHS)**

Detects multiple assignments with identical RHS expressions but different targets.

```python
x1 = y + z
x2 = y + z  # ⚠️ same RHS → possible copy-paste error
```

**Why it matters:** Often happens when duplicating a formula and forgetting to change one variable.

---

### **R2 — Repeated Left-Hand Side (LHS)**

Detects multiple assignments to the same variable with similar RHS patterns.

```python
x = y + z
x = a + b  # ⚠️ same LHS in close proximity → maybe should be another variable?
```

**Why it matters:** Common when copying a computation but forgetting to rename the target.

---

### **R5 — Self-Assignment**

Detects useless or accidental self-assignments.

```python
x = x  # ⚠️ likely unintended
```

**Why it matters:** Harmless but almost never intentional. Usually leftover from refactoring.

---

### **R7 — Repeated Operand (Identity or Null Operation)**

Flags operations where both operands are identical.

```python
diff = x - x  # ⚠️ always 0
ratio = y / y  # ⚠️ always 1
mask = a & a   # ⚠️ redundant
```

**Why it matters:** Often a sign of copy-paste or logic mistakes in symmetric code.

---

### **R10 — Multi-Increment / Index Mismatch**

Detects sequential code where one variable changes but others do not.

```python
sum1 = a[0] + b[0]
sum2 = a[1] + b[0]  # ⚠️ index mismatch → likely meant b[1]
```

**Why it matters:** Common bug when duplicating indexed or parameterized code without updating all parts consistently.

---

## 🧠 How It Works

1. **Tokenization:**
   Each line is split into identifiers, numbers, and symbols (language-agnostic regex).

2. **Normalization:**
   Identifiers and literals are replaced with placeholders (`<VAR>`, `<NUM>`), allowing semantic comparison.

3. **Sliding-window analysis:**
   Lines are compared within a configurable window (default ±5) for suspicious repetition.

4. **Heuristic matching:**
   Simple rule-based checks are applied to detect the patterns above.

---

## ⚙️ Usage

```bash
python copyedit_check.py path/to/source/
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

---

## 🧩 Limitations

* Not a parser — works purely by token similarity.
* May produce false positives in repetitive code (e.g. mathematical constants).
* Designed for **signal, not certainty** — human judgment required.

---

## 🛠️ Future Work

* “Multi-increment error” generalization using token-level alignment.
* Support for configurable rule thresholds.
* Integration with pre-commit hooks and CI pipelines.

---

Would you like me to now write the **first version of the actual Python script** implementing these five rules (R1, R2, R5, R7, R10) with a simple CLI?
