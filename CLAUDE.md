# Claude Course Material Generation Guide

This document explains how to generate new chapters and Labs for this learning project.

---

## Course Design Principles

### 1. Separate Theory and Practice
- `theory.md`: Concept explanations with code examples
- `lab_xx/`: Hands-on practice, write code yourself

### 2. Separate Problem, Workspace, and Solution
Each Lab structure:
```
lab_xx_name/
├── problem/
│   └── main.rs        ← Original problem (copy back to src/ to redo)
├── src/
│   └── main.rs        ← Workspace (learner writes code here)
├── solution/
│   └── main.rs        ← Reference answer (check after completing)
├── tests/
│   └── test_xxx.rs    ← Automated tests (verify implementation)
├── Cargo.toml
├── README.md
└── test.txt           ← Test data (if needed)
```

### 3. Test-Driven
- Every Lab must have automated tests
- Learners use `cargo test` to verify their implementation
- Tests should clearly indicate what went wrong

---

## Lab Problem Format

`problem/main.rs` and `src/main.rs` should start with:

```rust
//! Lab N: Title
//!
//! ## Goal
//! One sentence describing what this Lab does
//!
//! ## Requirements
//! 1. Specific feature A
//! 2. Specific feature B
//! 3. Specific feature C
//!
//! ## Hints
//! - Hint 1 (which function/module to use)
//! - Hint 2
//!
//! ## Verification
//! ```bash
//! cargo test
//! cargo run -- <args>
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Criterion 1
//! - [ ] Criterion 2
//!
//! Check solution/main.rs after completing

fn main() {
    // TODO: Implementation description
    //
    // Suggested steps:
    // 1. First step
    // 2. Second step

    todo!()  // or println!("Please implement...")
}
```

---

## Test File Format

`tests/test_xxx.rs`:

```rust
//! Lab N Tests
//!
//! Run with: cargo test

use std::process::Command;

fn run_program(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}

#[test]
fn test_01_basic_function() {
    // Test basic functionality
}

#[test]
fn test_02_error_handling() {
    // Test error handling
}
```

---

## Steps to Generate a New Chapter

### 1. Create Directory Structure

```
chapter_XX_name/
├── README.md              # Chapter overview
├── checkpoint.md          # Self-assessment
├── 01_topic_a/
│   ├── theory.md
│   ├── lab_01_xxx/
│   └── lab_02_xxx/
└── 02_topic_b/
    ├── theory.md
    └── lab_03_xxx/
```

### 2. Write theory.md

Structure:
```markdown
# Topic Title

## Section Goals
> One sentence summary

After completing this section, you will be able to:
- Skill 1
- Skill 2

---

## 1. Concept A

### Why Learn This?
(Motivation and practical applications)

### Core Concepts
(Code examples)

### Relation to System
(How to observe with tools)

---

## Summary
(How concepts connect)

## Next Steps
(Point to Labs)
```

### 3. Create Labs

For each Lab:

1. Create directory structure
2. Write `Cargo.toml`
3. Write `problem/main.rs` (problem)
4. Copy to `src/main.rs`
5. Write `solution/main.rs` (answer)
6. Write `tests/test_xxx.rs` (tests)
7. Write `README.md` (detailed instructions, optional)

### 4. Create checkpoint.md

```markdown
# Chapter X Checkpoint

## Self-Assessment Questions
- [ ] Question 1
- [ ] Question 2

## Implementation Verification
### Lab 1: XXX
- [ ] Acceptance criterion 1
- [ ] Acceptance criterion 2

## Concept Connection Quiz
1. Question (with reference answer)
```

---

## Redoing a Lab

When a learner wants to redo a Lab:

```bash
# Copy original problem back to src/
cp problem/main.rs src/main.rs
```

---

## Naming Conventions

- Chapter directories: `chapter_XX_name/` (XX is two-digit number)
- Topic directories: `01_topic_name/`
- Lab directories: `lab_XX_short_name/`
- Test files: `test_short_name.rs`

---

## Example: Prompt for Generating a New Lab

```
Please create a new Lab:

Chapter: chapter_02_os
Topic: 01_process_thread
Lab name: lab_01_thread_pool
Goal: Implement a simple Thread Pool

Requirements:
1. Create a fixed number of worker threads
2. Submit tasks to the pool
3. Tasks are executed by some worker

Please follow the CLAUDE.md format to create the complete Lab structure.
```
