# README

## Name
Andres Henao  

## Partner
Sean Dumont


---

## Overview
This assignment focused on improving the performance of a Universal Machine (UM) implementation using profiling and iterative optimization techniques. The goal was to identify bottlenecks and improve execution time, particularly on the large `sandmark` benchmark.

---

## Most Expensive Routine
The most expensive routine in the final program is the main execution loop of the Universal Machine, where instructions are repeatedly fetched, decoded, and executed.

---

## Assembly Analysis
Examining the compiled assembly code of the execution loop shows frequent memory accesses, register operations, and branching for instruction decoding and execution.

Many values are already kept in registers, and the compiler applies optimizations such as inlining and instruction scheduling. There are no obvious manual improvements that could be made directly at the assembly level without changing the overall program structure.

Further performance gains would likely come from reducing memory accesses or improving data representation rather than modifying assembly code directly.

---

## Performance Summary
- Baseline sandmark: 7.844s  
- Final sandmark (LTO enabled): ~5.0s  

This represents a significant performance improvement on the large benchmark.

---

## Hours Spent
- Analysis: ~3 hours  
- Implementation: ~4 hours