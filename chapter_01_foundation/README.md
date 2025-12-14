# Chapter 1: Foundation — Rust, Linux, Core Concepts

## Chapter Goals

After completing this chapter, you will be able to:

1. **Rust**
   - Understand and apply ownership, borrowing, lifetimes for memory management
   - Use `Result` and `?` for elegant error handling
   - Write multithreaded programs with `Arc`, `Mutex`, `mpsc`
   - Understand async/await basics

2. **Linux**
   - Use `ps`, `top`, `htop`, `grep` and other basic commands
   - Use `strace` to observe program system calls
   - Understand `/proc` virtual filesystem structure

3. **Integration**
   - Observe your program's behavior at the "system level"
   - Build "code ↔ OS behavior" mental connections

---

## Chapter Structure

```
chapter_01_foundation/
├── README.md                 # This file
├── checkpoint.md             # Self-assessment checkpoint
│
├── 01_rust_fundamentals/     # Rust core concepts
│   ├── theory.md            # Theory explanation
│   ├── lab_01_mini_cat/     # Lab: mini cat/grep
│   └── lab_02_parallel_sum/ # Lab: parallel computation
│
└── 02_linux_basics/          # Linux environment
    ├── theory.md            # Theory explanation
    ├── lab_03_strace/       # Lab: strace observation
    └── lab_04_mini_ps/      # Lab: mini ps
```

---

## Learning Roadmap

```
Week 1-2: Rust Core
┌─────────────────────────────────────────────────────────┐
│  Day 1-3: Ownership / Borrowing / Lifetimes             │
│  Day 4-5: Error Handling (Result / ?)                   │
│  Day 6-7: Lab 1 - Implement mini cat/grep              │
│  Day 8-10: Arc / Mutex / mpsc                          │
│  Day 11-12: Lab 2 - Parallel computation               │
│  Day 13-14: Async basics                               │
└─────────────────────────────────────────────────────────┘

Week 3: Linux Environment
┌─────────────────────────────────────────────────────────┐
│  Day 15-16: Basic commands + strace concepts            │
│  Day 17-18: Lab 3 - Use strace to observe programs     │
│  Day 19-20: /proc filesystem                           │
│  Day 21: Lab 4 - Implement mini ps                     │
└─────────────────────────────────────────────────────────┘
```

---

## Quick Start

1. Read `01_rust_fundamentals/theory.md`
2. Complete Lab 1 and Lab 2
3. Read `02_linux_basics/theory.md`
4. Complete Lab 3 and Lab 4
5. Use `checkpoint.md` to verify your learning

---

## Requirements

- Rust 1.70+ (recommended: install via rustup)
- Linux environment (native Linux, WSL2, or Docker)
- Editor: VS Code + rust-analyzer recommended
