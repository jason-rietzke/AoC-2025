# 🎄 Advent of Code 2025 🎄

Welcome to my Advent of Code 2025 solutions! This repository contains my attempts at solving the daily programming puzzles from [Advent of Code 2025](https://adventofcode.com/2025).

## 📊 System Specifications

- **CPU:** AMD Ryzen AI 9 HX 370 w/ Radeon 890M
- **Cores:** 24
- **OS:** Arch Linux

## 🏃 Performance Results

All solutions are implemented in Rust and compiled with `--release` optimizations.

### Day 1

```
Part 1: 1168 (in 624.518µs)
Part 2: 7199 (in 1.077075ms)
```

*Total time: 4.140 ms*

### Day 2

```
Part 1: 30599400849 (in 37.413528ms)
Part 2: 46270373595 (in 66.672895ms)
```

*Total time: 107.177 ms*

### Day 3

```
Part 1: 17158 (in 398.966µs)
Part 2: 170449335646486 (in 541.383µs)
```

*Total time: 3.007 ms*

### Day 4

```
Part 1: 1489 (in 306.434µs)
Part 2: 8890 (in 2.076934ms)
```

*Total time: 4.621 ms*

### Day 5

```
Part 1: 775 (in 188.803µs)
Part 2: 350684792662845 (in 58.71µs)
```

*Total time: 2.254 ms*

### Day 6

```
Part 1: 4722948564882 (in 214.852µs)
Part 2: 9581313737063 (in 534.349µs)
```

*Total time: 2.676 ms*

### Day 7

```
Part 1: 1690 (in 225.041µs)
Part 2: 221371496188107 (in 163.256µs)
```

*Total time: 2.799 ms*

### Day 8

```
Part 1: 163548 (in 14.488429ms)
	Parsing: 118.041µs
	Calculating links: 13.043025ms
	Calculating circuits: 1.310742ms
	Evaluating result: 7.564µs
Part 2: 772452514 (in 21.92004ms)
	Parsing: 75.361µs
	Calculating links: 19.453025ms
	Calculating circuits: 2.272591ms
	Evaluating result: 30ns
```

*Total time: 39.503 ms*

### Day 9

```
Part 1: 4782896435 (in 502.48µs)
Part 2: 1540060480 (in 29.536856ms)
```

*Total time: 32.979 ms*


---

**Total Runtime:** 265.888 ms

## 🛠️ Building and Running

To build all solutions:
```bash
cargo build --release
```

To run all days:
```bash
./run-all.sh
```

To run a specific day:
```bash
cargo run -p day01 --release
```

## 📝 Notes

Each day's solution is organized in its own workspace member with:
- `src/main.rs` - The solution code
- `data.txt` - The puzzle input
- `Cargo.toml` - Day-specific dependencies

---

*Last updated: 2025-12-10 02:39:03*
