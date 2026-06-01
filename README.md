# Universal Machine Virtual Machine (Rust)

A virtual machine implemented in Rust for CSC 411 (Computer Organization) at the University of Rhode Island.

This project implements the Universal Machine (UM) architecture, including segmented memory management, instruction decoding, and execution of machine programs. The system is capable of loading and executing UM binaries while efficiently managing dynamic memory segments and program state.

---

## Overview

The Universal Machine is a low-level virtual machine designed to execute a custom instruction set architecture. This project involved building the machine from the ground up, including instruction execution, segmented memory management, input/output operations, and program loading.

A significant focus of the project was performance optimization through profiling and iterative improvements, resulting in substantial execution time reductions on benchmark programs.

---

## Technologies

- Rust
- Cargo
- Systems Programming
- Virtual Machine Architecture
- Memory Management
- Performance Optimization
- Git & GitHub

---

## Features

### Instruction Execution

Implemented support for:

- Conditional Move
- Segmented Load
- Segmented Store
- Addition
- Multiplication
- Division
- NAND
- Halt
- Map Segment
- Unmap Segment
- Output
- Input
- Load Program
- Orthography

### Memory Management

- Dynamic segmented memory allocation
- Segment reuse and efficient memory handling
- Program loading and execution support
- Safe memory operations using Rust

### Performance Optimization

- Profiled execution bottlenecks
- Reduced unnecessary memory operations
- Improved instruction execution efficiency
- Optimized benchmark performance on large UM programs

---

## Project Structure

```text
src/
├── main.rs
├── memory.rs
├── instruction_process.rs
├── math_core.rs
├── input_output.rs
├── rumload.rs
└── lib.rs

Cargo.toml
README.md
```

---

## Benchmark Results

The virtual machine was tested using benchmark programs including:

- midmark.um
- sandmark.umz

Performance improvements were achieved through optimization of instruction execution and memory access patterns.

---

## Learning Outcomes

Through this project, I gained experience with:

- Systems programming in Rust
- Virtual machine implementation
- Instruction set architecture design
- Memory management techniques
- Performance profiling and optimization
- Debugging complex software systems
- Version control using Git and GitHub

---

## Author

**Andres Henao**

University of Rhode Island  
Bachelor of Science in Computer Science
